//! Internal-only worker checkout provisioning (not exposed through IPC/tools).
//! Permission approval and active-agent fencing must be added by a future
//! trusted host before making this capability available in production.
use super::{load, save};
use crate::{storage::Store, workspace};
use rusqlite::{params, OptionalExtension};
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Provisioned {
    pub worker_id: String,
    pub session_id: String,
    pub branch: String,
    pub base_commit: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub(crate) struct UnresolvedClaim {
    pub worker_session_id: String,
    pub branch: String,
    pub base_commit: String,
}

fn ensure_claims(store: &Store) -> Result<(), String> {
    store
        .db
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS architect_worker_worktree_claims (
            worker_session_id TEXT PRIMARY KEY NOT NULL,
            parent_session_id TEXT NOT NULL,
            worker_id TEXT NOT NULL,
            branch TEXT NOT NULL UNIQUE,
            base_commit TEXT NOT NULL,
            status TEXT NOT NULL CHECK(status IN ('preparing','attached')),
            created_at INTEGER NOT NULL
        );",
        )
        .map_err(|error| error.to_string())
}

pub(crate) fn unresolved_claims(store: &Store) -> Result<Vec<UnresolvedClaim>, String> {
    ensure_claims(store)?;
    let mut statement = store
        .db
        .prepare(
            "SELECT worker_session_id,branch,base_commit
         FROM architect_worker_worktree_claims
         WHERE status='preparing' ORDER BY created_at,worker_session_id",
        )
        .map_err(|error| error.to_string())?;
    let claims = statement
        .query_map([], |row| {
            Ok(UnresolvedClaim {
                worker_session_id: row.get(0)?,
                branch: row.get(1)?,
                base_commit: row.get(2)?,
            })
        })
        .map_err(|error| error.to_string())?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|error| error.to_string());
    claims
}

/// Staged implementation: the trusted caller must additionally prove absence
/// of live processes and explicit operator consent. Not wired to runtime.
pub(crate) fn provision_paused_worker(
    store: &Store,
    parent_id: &str,
    worker_id: &str,
) -> Result<Provisioned, String> {
    let mut team = load(store, parent_id)?.ok_or("Team is unavailable.")?;
    if team.status != "paused" || team.pending_setup.is_some() {
        return Err("Pause the team and settle setup before provisioning.".into());
    }
    // A paused Team can still have a worker shutting down. Do not create
    // new branch ownership until all existing worker runs are settled.
    if team
        .workers
        .iter()
        .any(|worker| super::active(&worker.status))
    {
        return Err("Wait until every active worker has stopped before provisioning.".into());
    }
    let worker = team
        .workers
        .iter()
        .find(|worker| worker.id == worker_id)
        .ok_or("Worker does not belong to this team.")?;
    if worker.status != "queued" || worker.read_only || worker.abandoned {
        return Err("Only queued writable workers may be provisioned.".into());
    }
    let child_id = worker.session_id.clone();
    let membership: Option<String> = store
        .db
        .query_row(
            "SELECT parent_session_id FROM architect_workers
         WHERE session_id=?1 AND worker_id=?2",
            params![child_id, worker_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| error.to_string())?;
    if membership.as_deref() != Some(parent_id) {
        return Err("Worker membership is not verified.".into());
    }
    let lead = store.session(parent_id)?;
    let child = store.session(&child_id)?;
    if child.project_id != lead.project_id
        || child.status != "idle"
        || child.worktree_path != lead.worktree_path
    {
        return Err("Worker is active, changed project or already has another checkout.".into());
    }
    let main = store.workspace_root(&lead.project_id, None)?;
    let lead_root = store.workspace_root(&lead.project_id, Some(parent_id))?;
    let git_status = workspace::git_status(&lead_root)?;
    if !git_status.is_repo || !git_status.files.is_empty() {
        return Err("Lead checkout must be a clean Git repository.".into());
    }
    let base_commit = workspace::head_commit(&lead_root)?;
    let branch_input = format!("r3-worker-{worker_id}");
    let branch = workspace::worktree_branch_name(&branch_input)?;

    // Reserve before Git mutation. A crash leaves a durable preparing claim;
    // NEVER retry, prune or delete it without operator reconciliation.
    ensure_claims(store)?;
    store
        .db
        .execute(
            "INSERT INTO architect_worker_worktree_claims (
             worker_session_id,parent_session_id,worker_id,branch,
             base_commit,status,created_at
         ) VALUES (?1,?2,?3,?4,?5,'preparing',?6)",
            params![
                child_id,
                parent_id,
                worker_id,
                branch,
                base_commit,
                crate::storage::now()
            ],
        )
        .map_err(|error| {
            format!(
        "Worktree claim already exists or cannot be reserved. Reconcile the existing claim: {error}"
    )
        })?;

    let created = workspace::create_worktree_with(&main, &branch_input, Some(&base_commit), false)
        .map_err(|error| {
            format!(
                "Git operation failed. Durable preparing claim requires operator review: {error}"
            )
        })?;
    if !workspace::is_registered_linked_worktree(&main, &created.path)? {
        return Err(
            "Git linked checkout is not verified; preparing claim requires operator review.".into(),
        );
    }

    let transaction = store
        .db
        .unchecked_transaction()
        .map_err(|error| error.to_string())?;
    let count = transaction
        .execute(
            "UPDATE sessions SET worktree_path=?3,worktree_branch=?4
         WHERE id=?1 AND project_id=?2 AND status='idle'
           AND worktree_path IS ?5",
            params![
                child_id,
                lead.project_id,
                created.path.to_string_lossy(),
                created.branch,
                lead.worktree_path
            ],
        )
        .map_err(|error| error.to_string())?;
    if count != 1 {
        return Err(
            "Worker session changed; preparing claim and Git checkout require manual review."
                .into(),
        );
    }
    transaction
        .execute(
            "UPDATE architect_worker_worktree_claims SET status='attached'
         WHERE worker_session_id=?1 AND status='preparing'",
            [child_id.as_str()],
        )
        .map_err(|error| error.to_string())?;
    save(store, &mut team)?;
    transaction.commit().map_err(|error| {
        format!("DB attachment failed; reconcile the preparing claim and checkout: {error}")
    })?;

    Ok(Provisioned {
        worker_id: worker_id.into(),
        session_id: child_id,
        path: created.path,
        branch: created.branch,
        base_commit,
    })
}

#[cfg(test)]
mod tests {
    use super::super::{new_team, TeamConfig, Worker};
    use super::*;
    use std::{fs, path::Path, process::Command};

    struct Fixture {
        root: PathBuf,
        store: Store,
        lead_id: String,
        child_id: String,
        worker_id: String,
    }
    fn git(root: &Path, args: &[&str]) {
        let result = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "Git {:?}: {}",
            args,
            String::from_utf8_lossy(&result.stderr)
        );
    }
    impl Fixture {
        fn new() -> Self {
            let folder =
                std::env::temp_dir().join(format!("nexora-r3-prov-{}", crate::storage::id()));
            let root = folder.join("source");
            fs::create_dir_all(&root).unwrap();
            git(&root, &["init", "-q"]);
            git(
                &root,
                &[
                    "-c",
                    "user.name=Test",
                    "-c",
                    "user.email=test@example.invalid",
                    "-c",
                    "commit.gpgsign=false",
                    "commit",
                    "-q",
                    "--allow-empty",
                    "-m",
                    "Initial commit",
                ],
            );
            let store = Store::open(Path::new(":memory:")).unwrap();
            let project = store.add_project(&root).unwrap();
            let lead_id = store
                .create_session(&project.id, "codex", "default", "ask")
                .unwrap()
                .id;
            let child_id = store
                .create_session(&project.id, "codex", "default", "ask")
                .unwrap()
                .id;
            let worker_id = crate::storage::id();
            store
                .db
                .execute(
                    "INSERT INTO architect_workers(parent_session_id,worker_id,session_id)
                 VALUES(?1,?2,?3)",
                    params![lead_id, worker_id, child_id],
                )
                .unwrap();
            let config: TeamConfig = serde_json::from_value(serde_json::json!({
                "workers":[],"maxParallel":2,"maxRounds":4,"maxAssignments":8,
                "workspace":"shared","summaryLimit":2048
            }))
            .unwrap();
            let mut team = new_team(lead_id.clone(), config);
            team.status = "paused".into();
            let worker: Worker = serde_json::from_value(serde_json::json!({
                "id":worker_id,"sessionId":child_id,
                "name":"writer","profileId":"default",
                "provider":"codex","model":"default","reasoningEffort":null,
                "status":"queued","assignment":"write isolated",
                "summary":"","error":null,"dependsOn":[],
                "files":["src"],"readOnly":false,
                "attempt":1,"createdAt":0,"updatedAt":0
            }))
            .unwrap();
            team.workers.push(worker);
            save(&store, &mut team).unwrap();
            Self {
                root,
                store,
                lead_id,
                child_id,
                worker_id,
            }
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            if let Some(parent) = self.root.parent() {
                let _ = fs::remove_dir_all(parent);
            }
        }
    }

    #[test]
    fn provisions_unique_verified_worktree_and_persists_session_link() {
        let f = Fixture::new();
        let initial_revision = load(&f.store, &f.lead_id).unwrap().unwrap().revision;
        let created = provision_paused_worker(&f.store, &f.lead_id, &f.worker_id).unwrap();
        assert_eq!(created.worker_id, f.worker_id);
        assert_eq!(created.session_id, f.child_id);
        assert_eq!(created.base_commit.len(), 40);
        assert!(created.branch.starts_with("pandacode/r3-worker-"));
        assert_ne!(created.path, f.root.canonicalize().unwrap());
        assert!(created.path.join(".git").is_file());
        assert!(workspace::is_registered_linked_worktree(
            &f.root.canonicalize().unwrap(),
            &created.path
        )
        .unwrap());
        let child = f.store.session(&f.child_id).unwrap();
        assert_eq!(child.worktree_path.as_deref(), created.path.to_str());
        assert_eq!(
            child.worktree_branch.as_deref(),
            Some(created.branch.as_str())
        );
        assert!(load(&f.store, &f.lead_id).unwrap().unwrap().revision > initial_revision);
        assert!(unresolved_claims(&f.store).unwrap().is_empty());
        assert!(provision_paused_worker(&f.store, &f.lead_id, &f.worker_id).is_err());
    }

    #[test]
    fn team_must_be_paused_and_checkout_clean() {
        let f = Fixture::new();
        let mut team = load(&f.store, &f.lead_id).unwrap().unwrap();
        team.status = "running".into();
        save(&f.store, &mut team).unwrap();
        assert!(provision_paused_worker(&f.store, &f.lead_id, &f.worker_id)
            .unwrap_err()
            .contains("Pause"));
        assert!(unresolved_claims(&f.store).unwrap().is_empty());
        team.status = "paused".into();
        save(&f.store, &mut team).unwrap();
        fs::write(f.root.join("uncommitted.txt"), "dirty").unwrap();
        assert!(provision_paused_worker(&f.store, &f.lead_id, &f.worker_id)
            .unwrap_err()
            .contains("clean"));
        assert!(unresolved_claims(&f.store).unwrap().is_empty());
    }

    #[test]
    fn orphaned_preparing_claim_blocks_unsafe_automatic_retry() {
        let f = Fixture::new();
        let branch =
            workspace::worktree_branch_name(&format!("r3-worker-{}", f.worker_id)).unwrap();
        git(&f.root, &["branch", &branch]);
        assert!(provision_paused_worker(&f.store, &f.lead_id, &f.worker_id)
            .unwrap_err()
            .contains("preparing claim"));
        let unresolved = unresolved_claims(&f.store).unwrap();
        assert_eq!(unresolved.len(), 1);
        assert_eq!(unresolved[0].branch, branch);
        assert_eq!(unresolved[0].worker_session_id, f.child_id);
        assert_eq!(unresolved[0].base_commit.len(), 40);
        assert!(provision_paused_worker(&f.store, &f.lead_id, &f.worker_id).is_err());
        assert!(f
            .store
            .session(&f.child_id)
            .unwrap()
            .worktree_path
            .is_none());
    }

    #[test]
    fn wrong_membership_or_active_worker_never_creates_checkout() {
        let f = Fixture::new();
        assert!(provision_paused_worker(&f.store, &f.lead_id, "outsider").is_err());
        let mut team = load(&f.store, &f.lead_id).unwrap().unwrap();
        team.workers[0].status = "running".into();
        save(&f.store, &mut team).unwrap();
        assert!(provision_paused_worker(&f.store, &f.lead_id, &f.worker_id).is_err());
        team.workers[0].status = "queued".into();
        team.workers[0].read_only = true;
        save(&f.store, &mut team).unwrap();
        assert!(provision_paused_worker(&f.store, &f.lead_id, &f.worker_id).is_err());
        assert!(unresolved_claims(&f.store).unwrap().is_empty());
    }

    #[test]
    fn durable_preparing_claim_survives_sqlite_close_and_reopen() {
        let folder =
            std::env::temp_dir().join(format!("nexora-r3-claim-recovery-{}", crate::storage::id()));
        fs::create_dir_all(&folder).unwrap();
        let db_path = folder.join("state.sqlite");
        {
            let db = Store::open(&db_path).unwrap();
            ensure_claims(&db).unwrap();
            db.db
                .execute(
                    "INSERT INTO architect_worker_worktree_claims (
                    worker_session_id,parent_session_id,worker_id,
                    branch,base_commit,status,created_at
                 ) VALUES ('child','lead','worker','pandacode/r3-worker-worker',
                           '0123456789012345678901234567890123456789','preparing',1)",
                    [],
                )
                .unwrap();
        }
        {
            let db = Store::open(&db_path).unwrap();
            let pending = unresolved_claims(&db).unwrap();
            assert_eq!(pending.len(), 1);
            assert_eq!(pending[0].worker_session_id, "child");
            assert_eq!(pending[0].branch, "pandacode/r3-worker-worker");
        }
        fs::remove_dir_all(folder).unwrap();
    }
}
