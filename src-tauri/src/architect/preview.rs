//! Read-only projection of queued team assignments.
//!
//! No worker is started, changed, stopped or granted authority by this module.
//! Actual runtime admission and conflicts are enforced by the architect engine.
use super::{active, conflicts, TeamSnapshot, Worker};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ScheduleWave {
    pub worker_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BlockedWorker {
    pub worker_id: String,
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SchedulePreview {
    pub revision: u64,
    pub max_parallel: usize,
    pub active_worker_ids: Vec<String>,
    pub waves: Vec<ScheduleWave>,
    pub blocked: Vec<BlockedWorker>,
    pub note: String,
}

/// Deterministically predict waves after currently active workers *might* finish.
/// This is only an informational plan. It ignores live state changes, other teams,
/// native process capacity and authorization; callers must never use it as admission.
pub(super) fn build(team: &TeamSnapshot) -> SchedulePreview {
    simulate(&team.workers, team.config.max_parallel, team.revision)
}

fn simulate(workers: &[Worker], configured_limit: usize, revision: u64) -> SchedulePreview {
    let max_parallel = if configured_limit == 0 {
        workers.len().max(1)
    } else {
        configured_limit
    };
    let active_worker_ids: Vec<_> = workers
        .iter()
        .filter(|worker| active(&worker.status))
        .map(|worker| worker.id.clone())
        .collect();
    let mut completed: HashSet<&str> = workers
        .iter()
        .filter(|worker| worker.status == "completed" || active(&worker.status))
        .map(|worker| worker.id.as_str())
        .collect();
    let mut pending: Vec<usize> = workers
        .iter()
        .enumerate()
        .filter_map(|(i, worker)| (worker.status == "queued").then_some(i))
        .collect();
    let mut waves = Vec::new();

    // Each iteration must consume at least one queued worker. This bounds the
    // planner even for cycles, missing/failed dependencies, or invalid snapshots.
    while !pending.is_empty() {
        let mut selected = Vec::new();
        for &index in &pending {
            if selected.len() >= max_parallel {
                break;
            }
            let worker = &workers[index];
            if worker
                .depends_on
                .iter()
                .all(|dependency| completed.contains(dependency.as_str()))
                && !selected
                    .iter()
                    .any(|&other: &usize| conflicts(worker, &workers[other]))
            {
                selected.push(index);
            }
        }
        if selected.is_empty() {
            break;
        }
        let worker_ids = selected
            .iter()
            .map(|&index| workers[index].id.clone())
            .collect();
        for &index in &selected {
            completed.insert(workers[index].id.as_str());
        }
        pending.retain(|index| !selected.contains(index));
        waves.push(ScheduleWave { worker_ids });
    }

    let blocked = pending
        .into_iter()
        .map(|index| {
            let worker = &workers[index];
            let reason = worker
                .depends_on
                .iter()
                .find_map(
                    |dependency| match workers.iter().find(|w| w.id == *dependency) {
                        None => Some("Unknown dependency".to_owned()),
                        Some(prerequisite)
                            if matches!(
                                prerequisite.status.as_str(),
                                "failed" | "stopped" | "cancelled"
                            ) || prerequisite.abandoned =>
                        {
                            Some("Dependency stopped or failed".to_owned())
                        }
                        _ => None,
                    },
                )
                .unwrap_or_else(|| {
                    "Unresolved dependency or cycle; no safe execution wave".to_owned()
                });
            BlockedWorker {
                worker_id: worker.id.clone(),
                reason,
            }
        })
        .collect();

    SchedulePreview {
        revision,
        max_parallel,
        active_worker_ids,
        waves,
        blocked,
        note: "Preview only. Waves assume active workers finish successfully. Actual scheduler, other teams, process availability, file leases and permissions may change execution. No workers were started.".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn worker(
        id: &str,
        status: &str,
        dependencies: &[&str],
        files: &[&str],
        read_only: bool,
    ) -> Worker {
        Worker {
            id: id.into(),
            session_id: format!("session-{id}"),
            name: id.into(),
            profile_id: "default".into(),
            provider: "codex".into(),
            model: "test".into(),
            reasoning_effort: None,
            status: status.into(),
            assignment: "fixture".into(),
            summary: String::new(),
            error: None,
            depends_on: dependencies.iter().map(|s| s.to_string()).collect(),
            files: files.iter().map(|s| s.to_string()).collect(),
            read_only,
            attempt: 0,
            created_at: 0,
            updated_at: 0,
            keys: Vec::new(),
            abandoned: false,
            result_revision: 0,
            checkpoint: None,
            timed_out: false,
        }
    }

    #[test]
    fn predicts_dependency_waves_without_running_workers() {
        let tasks = vec![
            worker("a", "queued", &[], &["src/a"], false),
            worker("b", "queued", &[], &["src/b"], false),
            worker("c", "queued", &["a", "b"], &["src/c"], false),
        ];
        let before = tasks.iter().map(|w| w.status.clone()).collect::<Vec<_>>();
        let result = simulate(&tasks, 2, 7);
        assert_eq!(result.revision, 7);
        assert_eq!(result.waves.len(), 2);
        assert_eq!(result.waves[0].worker_ids, vec!["a", "b"]);
        assert_eq!(result.waves[1].worker_ids, vec!["c"]);
        assert!(result.blocked.is_empty());
        assert_eq!(
            tasks.iter().map(|w| w.status.clone()).collect::<Vec<_>>(),
            before
        );
    }

    #[test]
    fn writing_scopes_cannot_run_in_same_wave() {
        let tasks = vec![
            worker("first", "queued", &[], &["src"], false),
            worker("second", "queued", &[], &["src/file.rs"], false),
            worker("third", "queued", &[], &["docs"], false),
        ];
        let result = simulate(&tasks, 3, 0);
        assert_eq!(result.waves[0].worker_ids, vec!["first", "third"]);
        assert_eq!(result.waves[1].worker_ids, vec!["second"]);
    }

    #[test]
    fn parallel_cap_limits_even_nonconflicting_workers() {
        let tasks = (0..5)
            .map(|i| {
                worker(
                    &format!("w{i}"),
                    "queued",
                    &[],
                    &[&format!("file-{i}")],
                    false,
                )
            })
            .collect::<Vec<_>>();
        let result = simulate(&tasks, 2, 0);
        let sizes = result
            .waves
            .iter()
            .map(|wave| wave.worker_ids.len())
            .collect::<Vec<_>>();
        assert_eq!(sizes, vec![2, 2, 1]);
    }

    #[test]
    fn zero_limit_means_unlimited_matching_existing_team_scheduler() {
        let tasks = vec![
            worker("a", "queued", &[], &["a"], false),
            worker("b", "queued", &[], &["b"], false),
        ];
        let result = simulate(&tasks, 0, 0);
        assert_eq!(result.max_parallel, 2);
        assert_eq!(result.waves.len(), 1);
    }

    #[test]
    fn read_only_workers_can_overlap_a_writer() {
        let tasks = vec![
            worker("writer", "queued", &[], &["src"], false),
            worker("reviewer", "queued", &[], &["src"], true),
        ];
        assert_eq!(simulate(&tasks, 2, 0).waves[0].worker_ids.len(), 2);
    }

    #[test]
    fn dependency_failures_and_unknown_ids_are_explicit() {
        let tasks = vec![
            worker("a", "failed", &[], &["a"], false),
            worker("b", "queued", &["a"], &["b"], false),
            worker("c", "queued", &["missing"], &["c"], false),
        ];
        let result = simulate(&tasks, 2, 0);
        assert!(result.waves.is_empty());
        assert_eq!(result.blocked[0].reason, "Dependency stopped or failed");
        assert_eq!(result.blocked[1].reason, "Unknown dependency");
    }

    #[test]
    fn malformed_cycles_do_not_hang_or_launch() {
        let tasks = vec![
            worker("a", "queued", &["b"], &[], false),
            worker("b", "queued", &["a"], &[], false),
        ];
        let result = simulate(&tasks, 2, 0);
        assert!(result.waves.is_empty());
        assert_eq!(result.blocked.len(), 2);
        assert!(result.blocked[0].reason.contains("cycle"));
    }

    #[test]
    fn active_workers_are_reported_not_redispatched() {
        let tasks = vec![
            worker("a", "running", &[], &["src"], false),
            worker("b", "queued", &["a"], &["src"], false),
        ];
        let result = simulate(&tasks, 1, 0);
        assert_eq!(result.active_worker_ids, vec!["a"]);
        assert_eq!(result.waves.len(), 1);
        assert_eq!(result.waves[0].worker_ids, vec!["b"]);
    }

    #[test]
    fn no_work_returns_empty_waves_and_blocked() {
        let tasks = vec![worker("done", "completed", &[], &[], false)];
        let result = simulate(&tasks, 4, 0);
        assert!(result.waves.is_empty());
        assert!(result.blocked.is_empty());
    }
}
