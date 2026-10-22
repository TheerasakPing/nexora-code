//! R3 incomplete worktree-claim recovery report (READ ONLY, internal).
//!
//! A 'preparing' reservation is an uncertain external effect, regardless of
//! whether its Git branch exists. This module never retries, mutates Git, or
//! marks a claim as resolved. Operator reconciliation remains a separate gate.
use crate::{
    storage::Store,
    workspace::{self, ClaimBranchEvidence},
};
use rusqlite::params;
use serde::Serialize;

const MAX_CLAIMS: usize = 512;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InterruptedClaim {
    pub worker_session_id: String,
    pub parent_session_id: String,
    pub worker_id: String,
    pub reserved_branch: String,
    pub base_commit: String,
    pub evidence: &'static str,
    pub linked_path: Option<String>,
    pub explanation: String,
    pub requires_operator_review: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClaimRecoveryReport {
    pub claims: Vec<InterruptedClaim>,
    pub note: &'static str,
}

#[derive(Debug)]
struct PersistedClaim {
    worker_session_id: String,
    parent_session_id: String,
    worker_id: String,
    branch: String,
    base_commit: String,
}

/// Read the existing schema without creating tables. "No claims" is asserted
/// only if the table is known to be absent or its SELECT returns zero rows.
/// All Git, project and corrupt-metadata failures become UNVERIFIED.
pub(crate) fn audit_interrupted(store: &Store) -> Result<ClaimRecoveryReport, String> {
    let exists: i64 = store
        .db
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table'
         AND name='architect_worker_worktree_claims'",
            [],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    let rows = if exists == 0 {
        Vec::new()
    } else {
        let mut statement = store
            .db
            .prepare(
                "SELECT worker_session_id,parent_session_id,worker_id,branch,base_commit
             FROM architect_worker_worktree_claims
             WHERE status='preparing' ORDER BY created_at,worker_session_id LIMIT ?1",
            )
            .map_err(|error| error.to_string())?;
        let claims = statement
            .query_map(params![(MAX_CLAIMS + 1) as i64], |row| {
                Ok(PersistedClaim {
                    worker_session_id: row.get(0)?,
                    parent_session_id: row.get(1)?,
                    worker_id: row.get(2)?,
                    branch: row.get(3)?,
                    base_commit: row.get(4)?,
                })
            })
            .map_err(|error| error.to_string())?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|error| error.to_string())?;
        if claims.len() > MAX_CLAIMS {
            return Err("Too many unresolved worktree claims to inspect safely.".into());
        }
        claims
    };

    let claims = rows.into_iter().map(|claim| {
        let evidence = (|| {
            let parent = store.session(&claim.parent_session_id)?;
            let main = store.workspace_root(&parent.project_id, None)?;
            workspace::inspect_claim_branch(&main, &claim.branch)
        })();
        let (evidence, linked_path, explanation) = match evidence {
            Ok(ClaimBranchEvidence::MissingBranch) => (
                "branch_missing", None,
                "No local branch reference was found. This is not proof that no effect occurred.".to_owned(),
            ),
            Ok(ClaimBranchEvidence::BranchOnly) => (
                "branch_only", None,
                "The reserved branch exists without a registered checkout. Inspect Git and user changes manually.".to_owned(),
            ),
            Ok(ClaimBranchEvidence::VerifiedLinked(path)) => (
                "linked_checkout_present", Some(path.display().to_string()),
                "A registered linked checkout exists; inspect the actual files, sessions and Git metadata before reconciling.".to_owned(),
            ),
            Err(error) => (
                "unverified", None,
                format!("Git or database evidence is unavailable/inconsistent: {error}"),
            ),
        };
        InterruptedClaim {
            worker_session_id: claim.worker_session_id,
            parent_session_id: claim.parent_session_id,
            worker_id: claim.worker_id,
            reserved_branch: claim.branch,
            base_commit: claim.base_commit,
            evidence,
            linked_path,
            explanation,
            requires_operator_review: true,
        }
    }).collect();

    Ok(ClaimRecoveryReport {
        claims,
        note: "Read-only report. Every PREPARING claim remains unresolved and MUST be reconciled by an authorized operator. No checkout, branch, session or journal was modified; never infer that retry/cleanup is safe.",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::{Path, PathBuf},
        process::Command,
    };

    struct Fixture {
        store: Store,
        root: PathBuf,
        parent: String,
    }
    impl Fixture {
        fn new() -> Self {
            let root =
                std::env::temp_dir().join(format!("nexora-claim-audit-{}", crate::storage::id()));
            fs::create_dir_all(&root).unwrap();
            git(&root, &["init", "-q"]);
            git(
                &root,
                &[
                    "-c",
                    "user.name=Audit",
                    "-c",
                    "user.email=audit@example.invalid",
                    "-c",
                    "commit.gpgsign=false",
                    "commit",
                    "-q",
                    "--allow-empty",
                    "-m",
                    "base",
                ],
            );
            let store = Store::open(Path::new(":memory:")).unwrap();
            let project = store.add_project(&root).unwrap();
            let parent = store
                .create_session(&project.id, "codex", "default", "ask")
                .unwrap()
                .id;
            Self {
                store,
                root,
                parent,
            }
        }
        fn claim(&self, worker: &str, branch: &str) {
            self.store
                .db
                .execute_batch(
                    "CREATE TABLE IF NOT EXISTS architect_worker_worktree_claims (
                    worker_session_id TEXT PRIMARY KEY NOT NULL,
                    parent_session_id TEXT NOT NULL,
                    worker_id TEXT NOT NULL,
                    branch TEXT NOT NULL UNIQUE,
                    base_commit TEXT NOT NULL,
                    status TEXT NOT NULL,
                    created_at INTEGER NOT NULL
                );",
                )
                .unwrap();
            self.store.db.execute(
                "INSERT INTO architect_worker_worktree_claims
                 (worker_session_id,parent_session_id,worker_id,branch,base_commit,status,created_at)
                 VALUES(?1,?2,?3,?4,'0123456789012345678901234567890123456789','preparing',1)",
                params![format!("sess_{worker}"),self.parent,worker,branch],
            ).unwrap();
        }
        fn branch(&self, name: &str) {
            git(&self.root, &["branch", name]);
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    fn git(root: &Path, args: &[&str]) {
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn no_table_or_claim_does_not_create_schema() {
        let f = Fixture::new();
        let initial: i64 = f
            .store
            .db
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE name='architect_worker_worktree_claims'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(initial, 0);
        assert!(audit_interrupted(&f.store).unwrap().claims.is_empty());
        let after: i64 = f
            .store
            .db
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE name='architect_worker_worktree_claims'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(after, 0);
    }

    #[test]
    fn branch_only_and_missing_are_distinct_but_both_require_review() {
        let f = Fixture::new();
        let existing = "pandacode/r3-worker-existing";
        f.branch(existing);
        f.claim("existing", existing);
        f.claim("missing", "pandacode/r3-worker-missing");
        let audit = audit_interrupted(&f.store).unwrap();
        assert_eq!(audit.claims.len(), 2);
        assert_eq!(audit.claims[0].evidence, "branch_only");
        assert_eq!(audit.claims[1].evidence, "branch_missing");
        assert!(audit
            .claims
            .iter()
            .all(|item| item.requires_operator_review));
        assert!(audit.claims.iter().all(|item| item.linked_path.is_none()));
    }

    #[test]
    fn real_registered_checkout_is_verified_without_mutating() {
        let f = Fixture::new();
        let branch = "pandacode/r3-worker-linked";
        let linked = f
            .root
            .parent()
            .unwrap()
            .join(format!("linked-{}", crate::storage::id()));
        let linked_arg = linked.to_str().unwrap();
        git(
            &f.root,
            &["worktree", "add", "-q", "-b", branch, linked_arg],
        );
        f.claim("linked", branch);
        let audit = audit_interrupted(&f.store).unwrap();
        assert_eq!(audit.claims[0].evidence, "linked_checkout_present");
        assert_eq!(
            audit.claims[0].linked_path.as_deref(),
            linked.canonicalize().unwrap().to_str()
        );
        assert!(audit.claims[0].requires_operator_review);
        // No Git edits, no claim state changes.
        let state: String = f
            .store
            .db
            .query_row(
                "SELECT status FROM architect_worker_worktree_claims WHERE worker_id='linked'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(state, "preparing");
        fs::remove_dir_all(&linked).unwrap();
    }

    #[test]
    fn missing_registered_checkout_is_unverified_not_clean_absence() {
        let f = Fixture::new();
        let branch = "pandacode/r3-worker-gone";
        let linked = f
            .root
            .parent()
            .unwrap()
            .join(format!("orphan-{}", crate::storage::id()));
        git(
            &f.root,
            &[
                "worktree",
                "add",
                "-q",
                "-b",
                branch,
                linked.to_str().unwrap(),
            ],
        );
        f.claim("gone", branch);
        fs::remove_dir_all(&linked).unwrap();
        let audit = audit_interrupted(&f.store).unwrap();
        assert_eq!(audit.claims[0].evidence, "unverified");
        assert!(audit.claims[0].linked_path.is_none());
        assert!(audit.claims[0].requires_operator_review);
        assert_eq!(audit.claims[0].reserved_branch, branch);
    }

    #[test]
    fn fake_branch_and_bad_project_must_never_claim_success() {
        let f = Fixture::new();
        f.claim("bad", "pandacode/r3-worker-bad");
        f.store
            .db
            .execute(
                "UPDATE architect_worker_worktree_claims
             SET branch='../unsafe',parent_session_id='unknown'
             WHERE worker_id='bad'",
                [],
            )
            .unwrap();
        let audit = audit_interrupted(&f.store).unwrap();
        assert_eq!(audit.claims[0].evidence, "unverified");
        assert!(audit.claims[0].requires_operator_review);
    }

    #[test]
    fn invalid_branch_name_is_unverified_not_missing() {
        let f = Fixture::new();
        f.claim("bad", "fake/invalid-ref");
        let audit = audit_interrupted(&f.store).unwrap();
        assert_eq!(audit.claims[0].evidence, "unverified");
    }
}
