//! Internal R4 durability prototype — NOT a workflow executor or authorization.
//!
//! The journal records intent transactionally before a future trusted executor
//! could dispatch work. Existing native app paths do not invoke it. A true
//! permission proof, effects sandbox and reconciler are required before use.
use crate::workflow_plan::{validate, WorkflowPlan};
use rusqlite::{params, Connection, OptionalExtension};
use sha2::{Digest, Sha256};

pub(crate) struct WorkflowJournal<'a> {
    db: &'a Connection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FinalOutcome {
    Succeeded,
    Failed,
}

impl FinalOutcome {
    fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        }
    }
}

impl<'a> WorkflowJournal<'a> {
    pub(crate) fn open(db: &'a Connection) -> Result<Self, String> {
        db.execute_batch(
            "CREATE TABLE IF NOT EXISTS workflow_journal_runs (
                id TEXT PRIMARY KEY NOT NULL,
                plan_hash TEXT NOT NULL,
                plan_json TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS workflow_journal_nodes (
                run_id TEXT NOT NULL,
                node_id TEXT NOT NULL,
                status TEXT NOT NULL CHECK(status IN
                    ('pending','running','succeeded','failed','cancelled','unknown')),
                generation INTEGER NOT NULL DEFAULT 0,
                intent_key TEXT UNIQUE,
                dependencies_json TEXT NOT NULL,
                PRIMARY KEY (run_id, node_id)
             );
             CREATE TABLE IF NOT EXISTS workflow_journal_events (
                seq INTEGER PRIMARY KEY AUTOINCREMENT,
                run_id TEXT NOT NULL,
                node_id TEXT NOT NULL,
                event TEXT NOT NULL,
                intent_key TEXT,
                generation INTEGER NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
             );",
        )
        .map_err(|error| error.to_string())?;
        Ok(Self { db })
    }

    /// Install an immutable, already validated workflow snapshot.
    /// Run IDs are unique: a repeated create cannot silently reset progress.
    pub(crate) fn create(&self, run_id: &str, plan: &WorkflowPlan) -> Result<(), String> {
        if !valid_token(run_id) {
            return Err("Invalid workflow run ID.".into());
        }
        validate(plan)?;
        let serialized = serde_json::to_string(plan).map_err(|error| error.to_string())?;
        let digest = format!("{:x}", Sha256::digest(serialized.as_bytes()));
        let tx = self.db.unchecked_transaction().map_err(|e| e.to_string())?;
        tx.execute(
            "INSERT INTO workflow_journal_runs(id,plan_hash,plan_json) VALUES(?1,?2,?3)",
            params![run_id, digest, serialized],
        )
        .map_err(|error| error.to_string())?;
        for node in &plan.nodes {
            let dependencies =
                serde_json::to_string(&node.depends_on).map_err(|e| e.to_string())?;
            tx.execute(
                "INSERT INTO workflow_journal_nodes(run_id,node_id,status,dependencies_json)
                  VALUES(?1,?2,'pending',?3)",
                params![run_id, node.id, dependencies],
            )
            .map_err(|error| error.to_string())?;
        }
        tx.commit().map_err(|e| e.to_string())
    }

    pub(crate) fn status(&self, run_id: &str, node_id: &str) -> Result<String, String> {
        self.db
            .query_row(
                "SELECT status FROM workflow_journal_nodes WHERE run_id=?1 AND node_id=?2",
                params![run_id, node_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| e.to_string())?
            .ok_or("Journal node not found.".into())
    }

    /// Idempotent inspection, no admission authority: pending nodes are only
    /// candidates, NOT permission to execute. UNKNOWN never satisfies edges.
    pub(crate) fn ready_candidates(&self, run_id: &str) -> Result<Vec<String>, String> {
        let plan: String = self
            .db
            .query_row(
                "SELECT plan_json FROM workflow_journal_runs WHERE id=?1",
                [run_id],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;
        let plan: WorkflowPlan = serde_json::from_str(&plan).map_err(|e| e.to_string())?;
        let mut ready = Vec::new();
        for node in plan.nodes {
            if self.status(run_id, &node.id)? == "pending"
                && node
                    .depends_on
                    .iter()
                    .all(|parent| self.status(run_id, parent).as_deref() == Ok("succeeded"))
            {
                ready.push(node.id);
            }
        }
        Ok(ready)
    }

    /// Internal effect fencing primitive: writes an intent in one SQLite
    /// transaction BEFORE any external operation could start. This is NOT
    /// consent and is not wired to any executor or app IPC.
    pub(crate) fn record_intent(
        &self,
        run_id: &str,
        node_id: &str,
        generation: u64,
        intent_key: &str,
    ) -> Result<(), String> {
        if generation == 0 || generation > i64::MAX as u64 || !valid_token(intent_key) {
            return Err("Invalid generation or effect intent key.".into());
        }
        let tx = self.db.unchecked_transaction().map_err(|e| e.to_string())?;
        let dependencies: String = tx
            .query_row(
                "SELECT dependencies_json FROM workflow_journal_nodes
                 WHERE run_id=?1 AND node_id=?2 AND status='pending'",
                params![run_id, node_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| e.to_string())?
            .ok_or("Node is not pending or does not exist.".to_owned())?;
        let dependencies: Vec<String> =
            serde_json::from_str(&dependencies).map_err(|e| e.to_string())?;
        for dependency in dependencies {
            let status: String = tx
                .query_row(
                    "SELECT status FROM workflow_journal_nodes
                     WHERE run_id=?1 AND node_id=?2",
                    params![run_id, dependency],
                    |row| row.get(0),
                )
                .map_err(|e| e.to_string())?;
            if status != "succeeded" {
                return Err("Prerequisite did not succeed; UNKNOWN blocks dependents.".into());
            }
        }
        tx.execute(
            "UPDATE workflow_journal_nodes
             SET status='running',generation=?3,intent_key=?4
             WHERE run_id=?1 AND node_id=?2 AND status='pending'",
            params![run_id, node_id, generation as i64, intent_key],
        )
        .map_err(|e| e.to_string())?;
        tx.execute(
            "INSERT INTO workflow_journal_events(run_id,node_id,event,intent_key,generation)
             VALUES(?1,?2,'intent',?3,?4)",
            params![run_id, node_id, intent_key, generation as i64],
        )
        .map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())
    }

    /// A stale process cannot mark a newer intent successful; never accept
    /// an outcome when the state is UNKNOWN after process restart.
    pub(crate) fn record_outcome(
        &self,
        run_id: &str,
        node_id: &str,
        generation: u64,
        intent_key: &str,
        outcome: FinalOutcome,
    ) -> Result<(), String> {
        if generation > i64::MAX as u64 {
            return Err("Generation exceeds storage range.".into());
        }
        let tx = self.db.unchecked_transaction().map_err(|e| e.to_string())?;
        let affected = tx
            .execute(
                "UPDATE workflow_journal_nodes SET status=?5
                 WHERE run_id=?1 AND node_id=?2 AND status='running'
                   AND generation=?3 AND intent_key=?4",
                params![
                    run_id,
                    node_id,
                    generation as i64,
                    intent_key,
                    outcome.as_str()
                ],
            )
            .map_err(|e| e.to_string())?;
        if affected != 1 {
            return Err("Stale intent, unresolved UNKNOWN or duplicate outcome.".into());
        }
        tx.execute(
            "INSERT INTO workflow_journal_events(run_id,node_id,event,intent_key,generation)
             VALUES(?1,?2,?3,?4,?5)",
            params![
                run_id,
                node_id,
                outcome.as_str(),
                intent_key,
                generation as i64
            ],
        )
        .map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())
    }

    /// Startup recovery. Unfinished effects become UNKNOWN; never replay them
    /// automatically, even when they were labeled read-only in user JSON.
    pub(crate) fn recover_interrupted(&self) -> Result<usize, String> {
        let tx = self.db.unchecked_transaction().map_err(|e| e.to_string())?;
        let mut stmt = tx
            .prepare(
                "SELECT run_id,node_id,intent_key,generation FROM workflow_journal_nodes
                 WHERE status='running'",
            )
            .map_err(|e| e.to_string())?;
        let pending = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, i64>(3)?,
                ))
            })
            .map_err(|e| e.to_string())?
            .collect::<rusqlite::Result<Vec<_>>>()
            .map_err(|e| e.to_string())?;
        drop(stmt);
        for (run_id, node_id, intent_key, generation) in &pending {
            tx.execute(
                "UPDATE workflow_journal_nodes SET status='unknown'
                 WHERE run_id=?1 AND node_id=?2 AND status='running'",
                params![run_id, node_id],
            )
            .map_err(|e| e.to_string())?;
            tx.execute(
                "INSERT INTO workflow_journal_events(run_id,node_id,event,intent_key,generation)
                 VALUES(?1,?2,'unknown_after_restart',?3,?4)",
                params![run_id, node_id, intent_key, generation],
            )
            .map_err(|e| e.to_string())?;
        }
        tx.commit().map_err(|e| e.to_string())?;
        Ok(pending.len())
    }

    pub(crate) fn event_count(&self, run_id: &str) -> Result<i64, String> {
        self.db
            .query_row(
                "SELECT COUNT(*) FROM workflow_journal_events WHERE run_id=?1",
                [run_id],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())
    }
}

fn valid_token(text: &str) -> bool {
    !text.is_empty()
        && text.len() <= 128
        && text
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_-".contains(&byte))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow_plan::{NodeKind, WorkflowNode};
    use std::fs;
    use std::path::PathBuf;

    fn plan() -> WorkflowPlan {
        WorkflowPlan {
            schema_version: 1,
            nodes: vec![
                WorkflowNode {
                    id: "approval".into(),
                    kind: NodeKind::Approval,
                    title: "Independent review".into(),
                    depends_on: vec![],
                },
                WorkflowNode {
                    id: "write".into(),
                    kind: NodeKind::Git,
                    title: "Mutating Git operation".into(),
                    depends_on: vec!["approval".into()],
                },
                WorkflowNode {
                    id: "review".into(),
                    kind: NodeKind::Review,
                    title: "Verify outcome".into(),
                    depends_on: vec!["write".into()],
                },
            ],
        }
    }

    fn database() -> (PathBuf, Connection) {
        let path =
            std::env::temp_dir().join(format!("nexora-journal-{}.sqlite", crate::storage::id()));
        let db = Connection::open(&path).unwrap();
        (path, db)
    }

    #[test]
    fn crash_marks_effects_unknown_and_blocks_dependent_work() {
        let (path, db) = database();
        let journal = WorkflowJournal::open(&db).unwrap();
        journal.create("run_one", &plan()).unwrap();
        assert_eq!(
            journal.ready_candidates("run_one").unwrap(),
            vec!["approval"]
        );
        journal
            .record_intent("run_one", "approval", 1, "approve_intent")
            .unwrap();
        journal
            .record_outcome(
                "run_one",
                "approval",
                1,
                "approve_intent",
                FinalOutcome::Succeeded,
            )
            .unwrap();
        assert_eq!(journal.ready_candidates("run_one").unwrap(), vec!["write"]);
        journal
            .record_intent("run_one", "write", 1, "write_intent")
            .unwrap();
        drop(db);

        let reopened = Connection::open(&path).unwrap();
        let recovered = WorkflowJournal::open(&reopened).unwrap();
        assert_eq!(recovered.recover_interrupted().unwrap(), 1);
        assert_eq!(recovered.status("run_one", "write").unwrap(), "unknown");
        assert!(recovered.ready_candidates("run_one").unwrap().is_empty());
        assert!(recovered
            .record_intent("run_one", "write", 2, "retry_same")
            .is_err());
        assert!(recovered
            .record_outcome(
                "run_one",
                "write",
                1,
                "write_intent",
                FinalOutcome::Succeeded
            )
            .is_err());
        assert_eq!(recovered.recover_interrupted().unwrap(), 0);
        assert_eq!(recovered.event_count("run_one").unwrap(), 4);
        drop(reopened);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn stale_generations_duplicate_outcomes_and_intents_are_rejected() {
        let (path, db) = database();
        let journal = WorkflowJournal::open(&db).unwrap();
        journal.create("run_two", &plan()).unwrap();
        assert!(journal.create("run_two", &plan()).is_err());
        journal
            .record_intent("run_two", "approval", 5, "key_a")
            .unwrap();
        assert!(journal
            .record_intent("run_two", "approval", 5, "key_b")
            .is_err());
        assert!(journal
            .record_outcome("run_two", "approval", 4, "key_a", FinalOutcome::Succeeded)
            .is_err());
        assert!(journal
            .record_outcome(
                "run_two",
                "approval",
                5,
                "wrong_key",
                FinalOutcome::Succeeded
            )
            .is_err());
        journal
            .record_outcome("run_two", "approval", 5, "key_a", FinalOutcome::Succeeded)
            .unwrap();
        assert!(journal
            .record_outcome("run_two", "approval", 5, "key_a", FinalOutcome::Succeeded)
            .is_err());
        assert_eq!(journal.event_count("run_two").unwrap(), 2);
        drop(db);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn no_unfinished_result_may_unlock_unknown_or_failed_dependencies() {
        let (path, db) = database();
        let journal = WorkflowJournal::open(&db).unwrap();
        journal.create("run_three", &plan()).unwrap();
        assert!(journal
            .record_intent("run_three", "write", 1, "premature")
            .is_err());
        assert!(journal
            .record_intent("run_three", "approval", 0, "bad")
            .is_err());
        journal
            .record_intent("run_three", "approval", 1, "valid")
            .unwrap();
        journal
            .record_outcome("run_three", "approval", 1, "valid", FinalOutcome::Failed)
            .unwrap();
        assert!(journal.ready_candidates("run_three").unwrap().is_empty());
        assert!(journal
            .record_intent("run_three", "write", 1, "blocked")
            .is_err());
        drop(db);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn fifty_disk_reopens_never_replay_unknown_intents_or_unlock_dependents() {
        // Source-level restart simulation: repeatedly close/reopen the real
        // on-disk SQLite connection. This does NOT simulate an OS power loss.
        let (path, initial) = database();
        drop(initial);
        for cycle in 0..50u64 {
            let run_id = format!("cycle_{cycle}");
            let approve_key = format!("approval_{cycle}");
            let effect_key = format!("effect_{cycle}");

            let db = Connection::open(&path).unwrap();
            let journal = WorkflowJournal::open(&db).unwrap();
            journal.create(&run_id, &plan()).unwrap();
            journal
                .record_intent(&run_id, "approval", cycle + 1, &approve_key)
                .unwrap();
            journal
                .record_outcome(
                    &run_id,
                    "approval",
                    cycle + 1,
                    &approve_key,
                    FinalOutcome::Succeeded,
                )
                .unwrap();
            journal
                .record_intent(&run_id, "write", cycle + 1, &effect_key)
                .unwrap();
            assert_eq!(journal.event_count(&run_id).unwrap(), 3);
            drop(db);

            let reopened = Connection::open(&path).unwrap();
            let recovered = WorkflowJournal::open(&reopened).unwrap();
            assert_eq!(
                recovered.recover_interrupted().unwrap(),
                1,
                "Cycle {cycle}: exactly one interrupted effect must be fenced"
            );
            assert_eq!(recovered.status(&run_id, "approval").unwrap(), "succeeded");
            assert_eq!(recovered.status(&run_id, "write").unwrap(), "unknown");
            assert_eq!(recovered.status(&run_id, "review").unwrap(), "pending");
            assert!(recovered.ready_candidates(&run_id).unwrap().is_empty());
            assert!(recovered
                .record_intent(&run_id, "write", cycle + 2, &format!("replay_{cycle}"))
                .is_err());
            assert!(recovered
                .record_outcome(
                    &run_id,
                    "write",
                    cycle + 1,
                    &effect_key,
                    FinalOutcome::Succeeded
                )
                .is_err());
            assert_eq!(recovered.recover_interrupted().unwrap(), 0);
            assert_eq!(recovered.event_count(&run_id).unwrap(), 4);
            drop(reopened);
        }
        let db = Connection::open(&path).unwrap();
        let journal = WorkflowJournal::open(&db).unwrap();
        let count: i64 = db
            .query_row(
                "SELECT COUNT(*) FROM workflow_journal_nodes WHERE status='unknown'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 50);
        let number_of_events: i64 = db
            .query_row("SELECT COUNT(*) FROM workflow_journal_events", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(number_of_events, 200);
        assert_eq!(journal.recover_interrupted().unwrap(), 0);
        drop(db);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn sqlite_event_write_failure_rolls_back_intent_and_outcome_atomically() {
        let (path, db) = database();
        let journal = WorkflowJournal::open(&db).unwrap();
        journal.create("injected_failure", &plan()).unwrap();
        db.execute_batch(
            "CREATE TEMP TRIGGER fail_audit BEFORE INSERT ON workflow_journal_events
             BEGIN SELECT RAISE(ABORT,'simulated audit disk write failure'); END;",
        )
        .unwrap();
        assert!(journal
            .record_intent("injected_failure", "approval", 1, "first_try")
            .is_err());
        assert_eq!(
            journal.status("injected_failure", "approval").unwrap(),
            "pending"
        );
        assert_eq!(journal.event_count("injected_failure").unwrap(), 0);
        db.execute_batch("DROP TRIGGER fail_audit;").unwrap();

        journal
            .record_intent("injected_failure", "approval", 2, "safe_try")
            .unwrap();
        db.execute_batch(
            "CREATE TEMP TRIGGER fail_audit BEFORE INSERT ON workflow_journal_events
             BEGIN SELECT RAISE(ABORT,'simulated audit disk write failure'); END;",
        )
        .unwrap();
        assert!(journal
            .record_outcome(
                "injected_failure",
                "approval",
                2,
                "safe_try",
                FinalOutcome::Succeeded
            )
            .is_err());
        assert_eq!(
            journal.status("injected_failure", "approval").unwrap(),
            "running"
        );
        assert_eq!(journal.event_count("injected_failure").unwrap(), 1);
        // Recovery must also roll back its UNKNOWN status update if the
        // durable audit event fails; it may never invent a resolved state.
        assert!(journal.recover_interrupted().is_err());
        assert_eq!(
            journal.status("injected_failure", "approval").unwrap(),
            "running"
        );
        assert_eq!(journal.event_count("injected_failure").unwrap(), 1);
        db.execute_batch("DROP TRIGGER fail_audit;").unwrap();

        assert_eq!(journal.recover_interrupted().unwrap(), 1);
        assert_eq!(
            journal.status("injected_failure", "approval").unwrap(),
            "unknown"
        );
        assert!(journal
            .record_outcome(
                "injected_failure",
                "approval",
                2,
                "safe_try",
                FinalOutcome::Succeeded
            )
            .is_err());
        assert_eq!(journal.event_count("injected_failure").unwrap(), 2);
        drop(db);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn invalid_intent_key_and_duplicated_effect_keys_are_rejected() {
        let (path, db) = database();
        let journal = WorkflowJournal::open(&db).unwrap();
        journal.create("run_four", &plan()).unwrap();
        assert!(journal
            .record_intent("run_four", "approval", 1, "../bad")
            .is_err());
        journal
            .record_intent("run_four", "approval", 1, "shared_key")
            .unwrap();
        journal
            .record_outcome(
                "run_four",
                "approval",
                1,
                "shared_key",
                FinalOutcome::Succeeded,
            )
            .unwrap();
        assert!(journal
            .record_intent("run_four", "write", 1, "shared_key")
            .is_err());
        assert_eq!(journal.status("run_four", "write").unwrap(), "pending");
        drop(db);
        fs::remove_file(path).unwrap();
    }
}
