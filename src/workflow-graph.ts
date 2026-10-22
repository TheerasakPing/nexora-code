import type { WorkflowPlan, WorkflowPreview } from './workflow-studio';

/** Read-only, bounded diagram model. The native validator is authoritative. */
export interface WorkflowGraphNode {
  id: string;
  title: string;
  kind: string;
  dependencies: string[];
  isApproval: boolean;
  mayHaveEffects: boolean;
}
export interface WorkflowGraphWave {
  index: number;
  nodes: WorkflowGraphNode[];
}

export function buildWorkflowGraph(
  plan: WorkflowPlan,
  preview: WorkflowPreview,
): WorkflowGraphWave[] {
  if (!Array.isArray(plan.nodes) || plan.nodes.length > 128
    || !Array.isArray(preview.waves) || preview.waves.length > 128
    || preview.nodeCount !== plan.nodes.length) {
    throw new Error('Workflow preview is inconsistent with the current plan.');
  }
  const declared = new Map<string, WorkflowPlan['nodes'][number]>();
  for (const node of plan.nodes) {
    if (!node || typeof node.id !== 'string' || !node.id
      || typeof node.title !== 'string' || typeof node.kind !== 'string'
      || declared.has(node.id)) {
      throw new Error('Workflow contains a missing, malformed or duplicate node.');
    }
    declared.set(node.id, node);
  }
  const approval = new Set(preview.approvalNodes);
  const effectful = new Set(preview.effectfulNodes);
  const scheduled = new Set<string>();
  const mapped: WorkflowGraphWave[] = [];
  for (const [index, wave] of preview.waves.entries()) {
    if (!Array.isArray(wave.nodeIds) || wave.nodeIds.length > 128) {
      throw new Error('Invalid execution wave returned by the validator.');
    }
    const nodes: WorkflowGraphNode[] = [];
    for (const id of wave.nodeIds) {
      const source = declared.get(id);
      if (!source || scheduled.has(id)) {
        throw new Error('Native workflow preview references unknown or duplicate nodes.');
      }
      const dependencies = source.dependsOn ?? [];
      if (!Array.isArray(dependencies) || dependencies.some(dep => typeof dep !== 'string')) {
        throw new Error('Invalid workflow dependency list.');
      }
      nodes.push({
        id, title: source.title, kind: source.kind, dependencies: [...dependencies],
        isApproval: approval.has(id), mayHaveEffects: effectful.has(id),
      });
      scheduled.add(id);
    }
    mapped.push({ index: index + 1, nodes });
  }
  if (scheduled.size !== declared.size) {
    throw new Error('Workflow preview does not include every declared node.');
  }
  return mapped;
}

function textElement<K extends keyof HTMLElementTagNameMap>(
  tag: K, className: string, text: string,
): HTMLElementTagNameMap[K] {
  const element = document.createElement(tag);
  element.className = className;
  element.textContent = text; // All untrusted workflow text remains inert.
  return element;
}

/** Accessible CSS column diagram, without HTML injection or external graph libs. */
export function renderWorkflowGraph(waves: WorkflowGraphWave[]): HTMLElement {
  const graph = textElement('section', 'workflow-graph', '');
  graph.setAttribute('aria-label', 'Proposed workflow dependency graph');
  const sequence = textElement('ol', 'workflow-graph-waves', '');
  for (const wave of waves) {
    const column = textElement('li', 'workflow-graph-wave', '');
    column.append(textElement('h3', 'workflow-graph-wave-label', 'Wave ' + wave.index));
    const cards = textElement('ul', 'workflow-graph-nodes', '');
    for (const node of wave.nodes) {
      const card = textElement('li', 'workflow-graph-node', '');
      card.append(textElement('strong', 'workflow-graph-node-title', node.title));
      card.append(textElement('span', 'workflow-graph-node-kind', node.kind + ' · ' + node.id));
      if (node.dependencies.length) {
        card.append(textElement('small', 'workflow-graph-node-dependencies',
          'After: ' + node.dependencies.join(', ')));
      }
      if (node.isApproval) {
        card.append(textElement('small', 'workflow-graph-node-caution',
          'Approval gate in plan — not actual consent'));
      }
      if (node.mayHaveEffects) {
        card.append(textElement('small', 'workflow-graph-node-caution',
          'Potential side effects — no operation executed'));
      }
      cards.append(card);
    }
    column.append(cards);
    sequence.append(column);
  }
  graph.append(sequence);
  return graph;
}
