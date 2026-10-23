//! Read-only R3 per-worker Git checkout isolation audit.
//!
//! A Team's workers currently inherit the lead's checkout. Path strings,
//! even paths containing .git, are not proof of a separate Git worktree.
//! This audit is information for the Team Lead, NOT admission or a sandbox.
use super::TeamSnapshot;
use crate::{storage::Store, workspace};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug)]
pub(super) struct AuditInput {
    pub revision: u64,
    pub main_root: Option<PathBuf>,
    pub lead_root: Option<PathBuf>,
    pub workers: Vec<WorkerInput>,
}
#[derive(Debug)]
pub(super) struct WorkerInput {
    pub id: String,
    pub name: String,
    pub resolved_root: Option<PathBuf>,
    /// From the worker's own persisted Session, not untrusted Agent prose.
    pub explicitly_linked: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerIsolationStatus {
    pub worker_id: String,
    pub name: String,
    pub classification: &'static str,
    pub checkout_path: Option<String>,
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IsolationAudit {
    pub revision: u64,
    pub isolated_count: usize,
    pub shared_count: usize,
    pub unverified_count: usize,
    pub workers: Vec<WorkerIsolationStatus>,
    pub note: String,
}

/// Gather DB-owned facts only. The caller releases the Store mutex before
/// running any git worktree queries, keeping UI/event loops responsive.
pub(super) fn collect(store: &Store, team: &TeamSnapshot) -> Result<AuditInput, String> {
    let parent = store.session(&team.session_id)?;
    let main_root = store.workspace_root(&parent.project_id, None).ok();
    let lead_root = store
        .workspace_root(&parent.project_id, Some(&parent.id))
        .ok();
    let workers = team
        .workers
        .iter()
        .map(|worker| {
            let session = store.session(&worker.session_id).ok();
            let valid = session
                .as_ref()
                .is_some_and(|session| session.project_id == parent.project_id);
            let resolved_root = session.as_ref().filter(|_| valid).and_then(|session| {
                store
                    .workspace_root(&session.project_id, Some(&session.id))
                    .ok()
            });
            // Do not launder a symlink alias into a verified checkout just
            // because workspace_root canonicalized its target.
            let explicitly_linked = valid
                && session.as_ref().is_some_and(|session| {
                    session.worktree_path.as_ref().is_some_and(|path| {
                        std::fs::symlink_metadata(path).is_ok_and(|metadata| {
                            metadata.is_dir() && !metadata.file_type().is_symlink()
                        })
                    })
                });
            WorkerInput {
                id: worker.id.clone(),
                name: worker.name.clone(),
                resolved_root,
                explicitly_linked,
            }
        })
        .collect();
    Ok(AuditInput {
        revision: team.revision,
        main_root,
        lead_root,
        workers,
    })
}

/// Read-only and deterministic given its filesystem snapshot. No Git
/// operations with effects (fetch, checkout, add, prune or remove) are used.
pub(super) fn evaluate(input: AuditInput) -> IsolationAudit {
    let mut statuses = Vec::with_capacity(input.workers.len());
    let mut isolated_count = 0;
    let mut shared_count = 0;
    let mut unverified_count = 0;
    for worker in &input.workers {
        let shared_lead = worker.resolved_root.is_some() && worker.resolved_root == input.lead_root;
        let shared_other = worker.resolved_root.is_some()
            && input
                .workers
                .iter()
                .any(|other| other.id != worker.id && other.resolved_root == worker.resolved_root);
        let (classification, reason) = if shared_lead {
            (
                "shared_lead",
                "Worker uses the Team Lead checkout; not isolated.",
            )
        } else if shared_other {
            ("shared_workers", "Two or more workers share one checkout.")
        } else if let (Some(main), Some(root)) = (&input.main_root, &worker.resolved_root) {
            if worker.explicitly_linked
                && workspace::is_registered_linked_worktree(main, root).unwrap_or(false)
            {
                (
                    "verified_linked_worktree",
                    "Unique, registered linked Git worktree; not an OS security sandbox.",
                )
            } else {
                (
                    "unverified",
                    "No separately registered Git linked worktree could be verified.",
                )
            }
        } else {
            (
                "unverified",
                "Worker checkout or project root is unavailable.",
            )
        };
        match classification {
            "verified_linked_worktree" => isolated_count += 1,
            "shared_lead" | "shared_workers" => shared_count += 1,
            _ => unverified_count += 1,
        }
        statuses.push(WorkerIsolationStatus {
            worker_id: worker.id.clone(),
            name: worker.name.clone(),
            classification,
            checkout_path: worker
                .resolved_root
                .as_ref()
                .map(|path| path.display().to_string()),
            reason: reason.into(),
        });
    }
    IsolationAudit {
        revision: input.revision,
        isolated_count,
        shared_count,
        unverified_count,
        workers: statuses,
        note: "Read-only audit. No workers were moved or started. A registered Git worktree is not a filesystem sandbox; provider tools can still write outside assigned scopes. Current Architect workers inherit their lead's checkout unless separately provisioned.".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn entry(id: &str, path: Option<PathBuf>, linked: bool) -> WorkerInput {
        WorkerInput {
            id: id.into(),
            name: id.into(),
            resolved_root: path,
            explicitly_linked: linked,
        }
    }
    fn input(
        main: Option<PathBuf>,
        lead: Option<PathBuf>,
        workers: Vec<WorkerInput>,
    ) -> AuditInput {
        AuditInput {
            revision: 17,
            main_root: main,
            lead_root: lead,
            workers,
        }
    }

    #[test]
    fn inherited_lead_checkout_is_never_marked_isolated() {
        let base = std::env::temp_dir();
        let result = evaluate(input(
            Some(base.clone()),
            Some(base.clone()),
            vec![
                entry("a", Some(base.clone()), false),
                entry("b", Some(base), true),
            ],
        ));
        assert_eq!(result.revision, 17);
        assert_eq!(result.isolated_count, 0);
        assert_eq!(result.shared_count, 2);
        assert!(result
            .workers
            .iter()
            .all(|w| w.classification == "shared_lead"));
    }

    #[test]
    fn missing_and_unregistered_checkout_paths_fail_closed() {
        let path = std::env::temp_dir();
        let result = evaluate(input(
            Some(path.clone()),
            Some(path.clone()),
            vec![
                entry("missing", None, true),
                entry("unregistered", Some(path.join("not_registered")), true),
            ],
        ));
        assert_eq!(result.unverified_count, 2);
        assert_eq!(result.isolated_count, 0);
        assert!(result.workers[0].reason.contains("unavailable"));
    }

    #[test]
    fn two_workers_sharing_same_nonlead_checkout_are_not_isolated() {
        let lead = std::env::temp_dir();
        let shared = lead.join("elsewhere-but-not-registered");
        let result = evaluate(input(
            Some(lead.clone()),
            Some(lead),
            vec![
                entry("x", Some(shared.clone()), true),
                entry("y", Some(shared), true),
            ],
        ));
        assert_eq!(result.shared_count, 2);
        assert!(result
            .workers
            .iter()
            .all(|worker| worker.classification == "shared_workers"));
    }

    #[test]
    fn a_unique_registered_git_worktree_is_positively_verified() {
        use std::process::Command;
        let folder =
            std::env::temp_dir().join(format!("nexora-audit-git-{}", crate::storage::id()));
        let main = folder.join("main");
        let linked = folder.join("agent");
        fs::create_dir_all(&main).unwrap();
        let git = |cwd: &std::path::Path, args: &[&str]| {
            let result = Command::new("git")
                .arg("-C")
                .arg(cwd)
                .args(args)
                .env("GIT_CONFIG_NOSYSTEM", "1")
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "git {:?}: {}",
                args,
                String::from_utf8_lossy(&result.stderr)
            );
        };
        git(&main, &["init", "-q"]);
        git(
            &main,
            &[
                "-c",
                "user.name=R3 Audit",
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
        let linked_str = linked.to_str().unwrap();
        git(
            &main,
            &["worktree", "add", "-q", "-b", "agent-audit", linked_str],
        );
        let main = main.canonicalize().unwrap();
        let linked = linked.canonicalize().unwrap();
        let audit = evaluate(input(
            Some(main.clone()),
            Some(main),
            vec![entry("worker-a", Some(linked), true)],
        ));
        assert_eq!(audit.isolated_count, 1);
        assert_eq!(audit.shared_count, 0);
        assert_eq!(audit.unverified_count, 0);
        assert_eq!(audit.workers[0].classification, "verified_linked_worktree");
        fs::remove_dir_all(folder).unwrap();
    }

    #[test]
    fn audit_on_real_db_owned_sessions_does_not_change_persisted_state() {
        let dir = std::env::temp_dir().join(format!("nexora-isolation-{}", crate::storage::id()));
        fs::create_dir(&dir).unwrap();
        let store = Store::open(std::path::Path::new(":memory:")).unwrap();
        let project = store.add_project(&dir).unwrap();
        let parent = store
            .create_session(&project.id, "codex", "default", "ask")
            .unwrap();
        let child = store
            .create_session(&project.id, "codex", "default", "ask")
            .unwrap();
        let original = store.session(&child.id).unwrap();
        let config: super::super::TeamConfig = serde_json::from_value(serde_json::json!({
            "workers": [],
            "maxParallel": 2,
            "maxRounds": 5,
            "maxAssignments": 12,
            "workspace": "project",
            "summaryLimit": 4096
        }))
        .unwrap();
        let mut team = super::super::new_team(parent.id.clone(), config);
        let worker: super::super::Worker = serde_json::from_value(serde_json::json!({
            "id": "worker-one",
            "sessionId": child.id,
            "name": "worker-one",
            "profileId": "default",
            "provider": "codex",
            "model": "default",
            "reasoningEffort": null,
            "status": "queued",
            "assignment": "read project",
            "summary": "",
            "error": null,
            "dependsOn": [],
            "files": ["src"],
            "attempt": 1,
            "createdAt": 0,
            "updatedAt": 0
        }))
        .unwrap();
        team.workers.push(worker);
        let snapshot = collect(&store, &team).unwrap();
        let result = evaluate(snapshot);
        assert_eq!(result.workers.len(), 1);
        assert_eq!(result.shared_count, 1);
        assert_eq!(
            store.session(&child.id).unwrap().worktree_path,
            original.worktree_path
        );
        drop(store);
        fs::remove_dir_all(dir).unwrap();
    }
}
