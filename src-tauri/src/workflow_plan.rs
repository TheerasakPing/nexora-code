//! Bounded, read-only R4 workflow DAG validation. No execution or side effects.
//! Actual approval, durable effects, and recovery are separate release gates.
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

const SCHEMA_VERSION: u32 = 1;
const MAX_NODES: usize = 128;
const MAX_DEPENDENCIES: usize = 16;
const MAX_BYTES: usize = 128 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    Agent,
    McpTool,
    Condition,
    Approval,
    Test,
    Review,
    Git,
    Notification,
    Artifact,
}

impl NodeKind {
    /// Even a declared read-only tool is not an OS-enforced guarantee.
    /// Require a manual approval barrier before any potentially effectful node.
    fn requires_approval(self) -> bool {
        matches!(
            self,
            Self::Agent
                | Self::McpTool
                | Self::Test
                | Self::Git
                | Self::Notification
                | Self::Artifact
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkflowNode {
    pub id: String,
    pub kind: NodeKind,
    pub title: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkflowPlan {
    pub schema_version: u32,
    pub nodes: Vec<WorkflowNode>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowWave {
    pub node_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowPreview {
    pub schema_version: u32,
    pub node_count: usize,
    pub waves: Vec<WorkflowWave>,
    pub approval_nodes: Vec<String>,
    pub effectful_nodes: Vec<String>,
    pub note: String,
}

/// Inspect a proposed DAG without storing, running or authorizing any node.
/// An approval dependency indicates an explicit human gate in the plan; no
/// approval is granted by this validator or an agent's generated JSON.
pub fn validate(plan: &WorkflowPlan) -> Result<WorkflowPreview, String> {
    let size = serde_json::to_vec(plan)
        .map_err(|error| error.to_string())?
        .len();
    if size > MAX_BYTES {
        return Err("Workflow plan exceeds 128 KiB.".into());
    }
    if plan.schema_version != SCHEMA_VERSION {
        return Err("Unsupported workflow schema version. Expected version 1.".into());
    }
    if plan.nodes.is_empty() || plan.nodes.len() > MAX_NODES {
        return Err("Workflow must contain 1-128 nodes.".into());
    }

    let mut ids = HashMap::<&str, usize>::new();
    let mut folded = HashSet::<String>::new();
    for (index, node) in plan.nodes.iter().enumerate() {
        if !valid_id(&node.id) {
            return Err(format!("Node {} has an invalid ID.", index + 1));
        }
        if !folded.insert(node.id.to_ascii_lowercase()) || ids.insert(&node.id, index).is_some() {
            return Err(format!("Duplicate workflow node ID: {}.", node.id));
        }
        if node.title.trim().is_empty()
            || node.title.len() > 160
            || node.title.chars().any(char::is_control)
        {
            return Err(format!("Node {} has an invalid title.", node.id));
        }
        if node.depends_on.len() > MAX_DEPENDENCIES {
            return Err(format!("Node {} has too many dependencies.", node.id));
        }
    }

    let mut inputs = vec![Vec::<usize>::new(); plan.nodes.len()];
    for (index, node) in plan.nodes.iter().enumerate() {
        let mut seen = HashSet::new();
        for dependency in &node.depends_on {
            let Some(&parent) = ids.get(dependency.as_str()) else {
                return Err(format!(
                    "Node {} references unknown dependency {}.",
                    node.id, dependency
                ));
            };
            if parent == index {
                return Err(format!("Node {} cannot depend on itself.", node.id));
            }
            if !seen.insert(parent) {
                return Err(format!(
                    "Node {} declares duplicate dependency {}.",
                    node.id, dependency
                ));
            }
            inputs[index].push(parent);
        }
    }

    let mut remaining: Vec<_> = (0..plan.nodes.len()).collect();
    let mut finished = HashSet::<usize>::new();
    let mut approval_reachable = vec![false; plan.nodes.len()];
    let mut waves = Vec::new();

    while !remaining.is_empty() {
        let available: Vec<usize> = remaining
            .iter()
            .copied()
            .filter(|index| {
                inputs[*index]
                    .iter()
                    .all(|parent| finished.contains(parent))
            })
            .collect();
        if available.is_empty() {
            return Err("Workflow contains a dependency cycle.".into());
        }

        for &index in &available {
            let node = &plan.nodes[index];
            let from_approval = inputs[index]
                .iter()
                .any(|parent| approval_reachable[*parent]);
            if node.kind.requires_approval() && !from_approval {
                return Err(format!(
                    "Node {} requires an upstream manual approval gate.",
                    node.id
                ));
            }
            approval_reachable[index] = node.kind == NodeKind::Approval || from_approval;
        }
        for index in &available {
            finished.insert(*index);
        }
        remaining.retain(|index| !finished.contains(index));
        waves.push(WorkflowWave {
            node_ids: available
                .iter()
                .map(|index| plan.nodes[*index].id.clone())
                .collect(),
        });
    }

    Ok(WorkflowPreview {
        schema_version: SCHEMA_VERSION,
        node_count: plan.nodes.len(),
        waves,
        approval_nodes: plan
            .nodes
            .iter()
            .filter(|node| node.kind == NodeKind::Approval)
            .map(|node| node.id.clone())
            .collect(),
        effectful_nodes: plan
            .nodes
            .iter()
            .filter(|node| node.kind.requires_approval())
            .map(|node| node.id.clone())
            .collect(),
        note: "Validation only. Approval nodes are NOT approved. No workflow nodes were executed, no tool grants were issued, and retries/UNKNOWN recovery require a durable journal before deployment.".into(),
    })
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_-".contains(&byte))
}

#[tauri::command]
pub fn workflow_validate_plan(plan: WorkflowPlan) -> crate::Result<WorkflowPreview> {
    validate(&plan)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str, kind: NodeKind, dependencies: &[&str]) -> WorkflowNode {
        WorkflowNode {
            id: id.into(),
            kind,
            title: format!("Task {id}"),
            depends_on: dependencies.iter().map(|id| (*id).into()).collect(),
        }
    }

    fn plan(nodes: Vec<WorkflowNode>) -> WorkflowPlan {
        WorkflowPlan {
            schema_version: 1,
            nodes,
        }
    }

    #[test]
    fn builds_stable_dependency_waves_with_explicit_approval() {
        let plan = plan(vec![
            node("read", NodeKind::Condition, &[]),
            node("approve", NodeKind::Approval, &["read"]),
            node("git", NodeKind::Git, &["approve"]),
            node("test", NodeKind::Test, &["approve"]),
            node("finish", NodeKind::Review, &["git", "test"]),
        ]);
        let preview = validate(&plan).unwrap();
        let waves = preview
            .waves
            .into_iter()
            .map(|wave| wave.node_ids)
            .collect::<Vec<_>>();
        assert_eq!(
            waves,
            vec![
                vec!["read"],
                vec!["approve"],
                vec!["git", "test"],
                vec!["finish"]
            ]
        );
        assert_eq!(preview.approval_nodes, ["approve"]);
        assert_eq!(preview.effectful_nodes, ["git", "test"]);
    }

    #[test]
    fn effectful_nodes_without_human_approval_do_not_validate() {
        for kind in [
            NodeKind::Agent,
            NodeKind::McpTool,
            NodeKind::Test,
            NodeKind::Git,
            NodeKind::Notification,
            NodeKind::Artifact,
        ] {
            let invalid = plan(vec![node("effect", kind, &[])]);
            assert!(validate(&invalid).unwrap_err().contains("manual approval"));
        }
    }

    #[test]
    fn approval_is_not_inferred_from_sibling_branch() {
        let invalid = plan(vec![
            node("approve", NodeKind::Approval, &[]),
            node("read", NodeKind::Condition, &[]),
            node("effect", NodeKind::McpTool, &["read"]),
        ]);
        assert!(validate(&invalid).unwrap_err().contains("manual approval"));
    }

    #[test]
    fn disallows_cycles_self_references_duplicates_and_missing_edges() {
        for invalid in [
            plan(vec![
                node("a", NodeKind::Condition, &["b"]),
                node("b", NodeKind::Review, &["a"]),
            ]),
            plan(vec![node("a", NodeKind::Condition, &["a"])]),
            plan(vec![
                node("a", NodeKind::Condition, &[]),
                node("A", NodeKind::Review, &[]),
            ]),
            plan(vec![node("a", NodeKind::Condition, &["missing"])]),
            plan(vec![
                node("a", NodeKind::Condition, &[]),
                node("b", NodeKind::Review, &["a", "a"]),
            ]),
        ] {
            assert!(validate(&invalid).is_err());
        }
    }

    #[test]
    fn schema_and_identifier_validation_fail_closed() {
        let mut future = plan(vec![node("safe", NodeKind::Condition, &[])]);
        future.schema_version = 2;
        assert!(validate(&future).unwrap_err().contains("schema"));
        for id in [
            "../escape",
            "unsafe/path",
            "with space",
            "",
            "a:stream",
            "a".repeat(65).as_str(),
        ] {
            assert!(validate(&plan(vec![node(id, NodeKind::Condition, &[])])).is_err());
        }
        let mut bad = plan(vec![node("a", NodeKind::Condition, &[])]);
        bad.nodes[0].title = "\n".into();
        assert!(validate(&bad).is_err());
    }

    #[test]
    fn bounds_nodes_dependencies_and_request_bytes() {
        assert!(validate(&plan(vec![])).is_err());
        let large = plan(
            (0..129)
                .map(|i| node(&format!("n{i}"), NodeKind::Review, &[]))
                .collect(),
        );
        assert!(validate(&large).is_err());
        let deps = (0..17).map(|i| format!("n{i}")).collect::<Vec<_>>();
        let mut invalid = plan(
            deps.iter()
                .map(|id| node(id, NodeKind::Review, &[]))
                .collect(),
        );
        invalid.nodes.push(WorkflowNode {
            id: "overloaded".into(),
            kind: NodeKind::Review,
            title: "Many dependencies".into(),
            depends_on: deps,
        });
        assert!(validate(&invalid)
            .unwrap_err()
            .contains("too many dependencies"));
        let mut bytes = plan(vec![node("a", NodeKind::Review, &[])]);
        bytes.nodes[0].title = "x".repeat(128 * 1024);
        assert!(validate(&bytes).is_err());
    }

    #[test]
    fn serde_rejects_unknown_fields_and_node_types() {
        let data = serde_json::json!({
            "schemaVersion": 1,
            "nodes": [{"id":"a","kind":"read_files","title":"Read","dependsOn":[]}]
        });
        assert!(serde_json::from_value::<WorkflowPlan>(data).is_err());
        let data = serde_json::json!({
            "schemaVersion": 1,
            "nodes": [{"id":"a","kind":"condition","title":"Read","extra":"reject"}]
        });
        assert!(serde_json::from_value::<WorkflowPlan>(data).is_err());
    }

    #[test]
    fn no_execution_state_mutation_during_validation() {
        let plan = plan(vec![
            node("approve", NodeKind::Approval, &[]),
            node("analyze", NodeKind::Agent, &["approve"]),
        ]);
        let before = serde_json::to_value(&plan).unwrap();
        let result = validate(&plan).unwrap();
        assert_eq!(result.node_count, 2);
        assert_eq!(serde_json::to_value(&plan).unwrap(), before);
        assert!(result.note.contains("NOT approved"));
    }
}
