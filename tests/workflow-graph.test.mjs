import test from 'node:test';
import assert from 'node:assert/strict';
import { buildWorkflowGraph } from '../src/workflow-graph.ts';

const plan = {
  schemaVersion: 1,
  nodes: [
    { id: 'inspect', kind: 'condition', title: 'Inspect', dependsOn: [] },
    { id: 'approval', kind: 'approval', title: 'Review manually', dependsOn: ['inspect'] },
    { id: 'effect', kind: 'git', title: 'Would modify repository', dependsOn: ['approval'] },
    { id: 'report', kind: 'review', title: 'Check result', dependsOn: ['effect'] },
  ],
};
const preview = {
  schemaVersion: 1, nodeCount: 4, approvalNodes: ['approval'], effectfulNodes: ['effect'],
  waves: [
    { nodeIds: ['inspect'] }, { nodeIds: ['approval'] },
    { nodeIds: ['effect'] }, { nodeIds: ['report'] },
  ], note: 'No execution, approval or side effects',
};

test('maps dependency waves and effect/approval distinctions', () => {
  const graph = buildWorkflowGraph(plan, preview);
  assert.deepEqual(graph.map(w => w.index), [1, 2, 3, 4]);
  assert.equal(graph[1].nodes[0].isApproval, true);
  assert.equal(graph[1].nodes[0].mayHaveEffects, false);
  assert.equal(graph[2].nodes[0].mayHaveEffects, true);
  assert.deepEqual(graph[2].nodes[0].dependencies, ['approval']);
});

test('preserves stable parallel ordering from trusted validator', () => {
  const p = { schemaVersion: 1, nodes: [
    { id: 'b', kind: 'review', title: 'B' },
    { id: 'a', kind: 'condition', title: 'A' },
  ] };
  const v = { schemaVersion: 1, nodeCount: 2, approvalNodes: [], effectfulNodes: [],
    waves: [{ nodeIds: ['b', 'a'] }], note: '' };
  assert.deepEqual(buildWorkflowGraph(p, v)[0].nodes.map(n => n.id), ['b', 'a']);
});

test('rejects unknown, duplicate and omitted nodes rather than fabricate preview', () => {
  const v = structuredClone(preview);
  v.waves[1].nodeIds = ['not-present'];
  assert.throws(() => buildWorkflowGraph(plan, v), /unknown or duplicate/);
  const repeat = structuredClone(preview);
  repeat.waves[1].nodeIds = ['inspect'];
  assert.throws(() => buildWorkflowGraph(plan, repeat), /unknown or duplicate/);
  const missing = structuredClone(preview);
  missing.waves = missing.waves.slice(1);
  assert.throws(() => buildWorkflowGraph(plan, missing), /does not include every/);
});

test('rejects invalid counts, duplicate declared node IDs and oversized waves', () => {
  assert.throws(() => buildWorkflowGraph(plan, { ...preview, nodeCount: 0 }), /inconsistent/);
  assert.throws(() => buildWorkflowGraph({ ...plan, nodes: [...plan.nodes, plan.nodes[0]] },
    { ...preview, nodeCount: 5 }), /duplicate node/);
  assert.throws(() => buildWorkflowGraph(plan, { ...preview,
    waves: [{ nodeIds: Array(129).fill('inspect') }] }), /Invalid execution wave/);
});

test('hostile title and dependency remain inert data, without mutation', () => {
  const original = structuredClone(plan);
  const hostile = '<img src=x onerror=alert(1)>';
  const p = structuredClone(plan);
  p.nodes[2].title = hostile;
  p.nodes[2].dependsOn = [hostile];
  const result = buildWorkflowGraph(p, preview);
  assert.equal(result[2].nodes[0].title, hostile);
  assert.equal(result[2].nodes[0].dependencies[0], hostile);
  assert.deepEqual(plan, original);
});
