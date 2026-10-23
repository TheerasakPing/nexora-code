import { buildWorkflowGraph, renderWorkflowGraph } from './workflow-graph.ts';

export interface WorkflowPlan {
  schemaVersion: number;
  nodes: { id: string; kind: string; title: string; dependsOn?: string[] }[];
}
export interface WorkflowPreview {
  schemaVersion: number;
  nodeCount: number;
  waves: { nodeIds: string[] }[];
  approvalNodes: string[];
  effectfulNodes: string[];
  note: string;
}
export type ValidationState =
  | { phase: 'idle' }
  | { phase: 'validating' }
  | { phase: 'valid'; result: WorkflowPreview }
  | { phase: 'invalid'; message: string };
export const MAX_WORKFLOW_BYTES = 128 * 1024;
export const SAMPLE_WORKFLOW = JSON.stringify({
  schemaVersion: 1,
  nodes: [
    { id: 'inspect', kind: 'condition', title: 'Inspect input' },
    { id: 'approve', kind: 'approval', title: 'External operator consent required', dependsOn: ['inspect'] },
    { id: 'propose', kind: 'git', title: 'Propose patch, NOT executed', dependsOn: ['approve'] },
    { id: 'review', kind: 'review', title: 'Review the proposal', dependsOn: ['propose'] },
  ],
}, null, 2);

export function parseWorkflowInput(text: string): WorkflowPlan {
  if (new TextEncoder().encode(text).length > MAX_WORKFLOW_BYTES) {
    throw new Error('Workflow exceeds the 128 KiB validation limit.');
  }
  let value: unknown;
  try { value = JSON.parse(text); }
  catch { throw new Error('Invalid JSON. Check brackets, quotes and commas.'); }
  if (typeof value !== 'object' || value === null || Array.isArray(value)) {
    throw new Error('Workflow must be a JSON object.');
  }
  const plan = value as Record<string, unknown>;
  if (!Number.isInteger(plan.schemaVersion) || !Array.isArray(plan.nodes)) {
    throw new Error('Workflow requires schemaVersion and nodes.');
  }
  return value as WorkflowPlan; // Further strict verification is native Rust.
}

/** Validates only the current editor generation. Never invokes an executor. */
export function createWorkflowValidation(
  nativeValidate: (plan: WorkflowPlan) => Promise<WorkflowPreview>,
  onChange: (state: ValidationState) => void,
) {
  let generation = 0, disposed = false;
  let state: ValidationState = { phase: 'idle' };
  const emit = (next: ValidationState) => {
    if (!disposed) { state = next; onChange(next); }
  };
  return {
    get state() { return state; },
    async check(text: string): Promise<void> {
      const ticket = ++generation;
      let plan: WorkflowPlan;
      try { plan = parseWorkflowInput(text); }
      catch (error) {
        emit({ phase: 'invalid', message: error instanceof Error ? error.message : String(error) });
        return;
      }
      emit({ phase: 'validating' });
      try {
        const result = await nativeValidate(plan);
        if (ticket === generation) emit({ phase: 'valid', result });
      } catch (error) {
        if (ticket === generation) emit({ phase: 'invalid', message: error instanceof Error ? error.message : String(error) });
      }
    },
    invalidate(): void { ++generation; emit({ phase: 'idle' }); },
    dispose(): void { ++generation; disposed = true; },
  };
}

function node<K extends keyof HTMLElementTagNameMap>(tag: K, cls = '', content = ''): HTMLElementTagNameMap[K] {
  const el = document.createElement(tag); el.className = cls; el.textContent = content; return el;
}
function button(text: string, cls = 'secondary-button'): HTMLButtonElement {
  const el = node('button', cls, text); el.type = 'button'; return el;
}

/** Purely read-only dialog: all input/native data rendered as text nodes, never HTML. */
export function mountWorkflowStudio(
  dialog: HTMLDialogElement,
  nativeValidate: (plan: WorkflowPlan) => Promise<WorkflowPreview>,
): { dispose(): void } {
  const root = node('section', 'workflow-studio');
  const head = node('header', 'workflow-studio-heading');
  const close = button('Close', 'workflow-studio-close');
  close.setAttribute('aria-label', 'Close Workflow Studio');
  head.append(node('h2', '', 'Workflow Studio'), close);
  const label = node('label', 'workflow-studio-label', 'Workflow plan · JSON');
  const editor = node('textarea', 'workflow-studio-editor');
  editor.value = SAMPLE_WORKFLOW; editor.spellcheck = false;
  editor.setAttribute('aria-label', 'Workflow plan JSON');
  editor.setAttribute('aria-describedby', 'workflow-studio-warning');
  label.append(editor);
  const warning = node('p', 'workflow-studio-warning',
    'An Approval node is not real human consent. No workflow, Git, Agent or MCP actions run here.');
  warning.id = 'workflow-studio-warning';
  const actions = node('div', 'workflow-studio-actions');
  const reset = button('Reset example'), check = button('Validate plan', 'primary-button');
  actions.append(reset, check);
  const results = node('section', 'workflow-studio-results');
  results.setAttribute('aria-live', 'polite');
  results.setAttribute('aria-label', 'Workflow validation results');
  root.append(head, node('p', 'workflow-studio-intro',
    'Inspect dependency waves and approval gates without running any work.'),
  label, warning, actions, results);
  dialog.className = 'settings-dialog workflow-studio-dialog';
  dialog.replaceChildren(root);
  let disposed = false;
  let planForGraph: WorkflowPlan | undefined;
  const controller = createWorkflowValidation(nativeValidate, value => {
    if (disposed || root.parentElement !== dialog || !dialog.open) return;
    check.disabled = value.phase === 'validating';
    results.replaceChildren();
    if (value.phase === 'idle') {
      results.append(node('p', 'workflow-studio-hint', 'Choose Validate plan to check dependency waves.'));
    } else if (value.phase === 'validating') {
      results.append(node('p', 'workflow-studio-hint', 'Checking plan with native Rust…'));
    } else if (value.phase === 'invalid') {
      const error = node('p', 'workflow-studio-error', value.message);
      error.setAttribute('role', 'alert'); results.append(error);
    } else {
      results.append(node('p', 'workflow-studio-summary',
        'Structure valid · ' + value.result.nodeCount + ' nodes · ' + value.result.waves.length + ' waves'));
      try {
        const graph = buildWorkflowGraph(planForGraph!, value.result);
        results.append(renderWorkflowGraph(graph));
      } catch (error) {
        results.append(node('p', 'workflow-studio-error',
          error instanceof Error ? error.message : String(error)));
      }
      results.append(
        node('p', 'workflow-studio-summary',
          'Approval nodes: ' + (value.result.approvalNodes.join(', ') || 'none')),
        node('p', 'workflow-studio-summary',
          'Potential effects: ' + (value.result.effectfulNodes.join(', ') || 'none')),
        node('p', 'workflow-studio-note', value.result.note));
    }
  });
  function dispose() {
    if (disposed) return;
    disposed = true; controller.dispose();
    dialog.removeEventListener('close', onClose);
    reset.removeEventListener('click', onReset);
    check.removeEventListener('click', onCheck);
    editor.removeEventListener('input', onInput);
    close.removeEventListener('click', onClickClose);
  }
  const onClose = () => dispose();
  const onReset = () => { editor.value = SAMPLE_WORKFLOW; planForGraph = undefined; controller.invalidate(); };
  const onCheck = () => {
    try { planForGraph = parseWorkflowInput(editor.value); }
    catch { planForGraph = undefined; }
    void controller.check(editor.value);
  };
  const onInput = () => { planForGraph = undefined; controller.invalidate(); };
  const onClickClose = () => { if (dialog.open) dialog.close(); };
  reset.addEventListener('click', onReset);
  check.addEventListener('click', onCheck);
  editor.addEventListener('input', onInput);
  close.addEventListener('click', onClickClose);
  dialog.addEventListener('close', onClose);
  if (!dialog.open) dialog.showModal();
  results.append(node('p', 'workflow-studio-hint', 'Choose Validate plan to check dependency waves.'));
  editor.focus({ preventScroll: true });
  return { dispose };
}
