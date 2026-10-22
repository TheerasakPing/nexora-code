import test from 'node:test';
import assert from 'node:assert/strict';
import {
  createWorkflowValidation, parseWorkflowInput, SAMPLE_WORKFLOW,
  MAX_WORKFLOW_BYTES,
} from '../src/workflow-studio.ts';

const validResult = {
  schemaVersion: 1, nodeCount: 2, waves: [{ nodeIds: ['approve'] }, { nodeIds: ['git'] }],
  approvalNodes: ['approve'], effectfulNodes: ['git'],
  note: 'No nodes were executed or permissions granted.',
};

test('template parses into versioned DAG with approval dependency', () => {
  const plan = parseWorkflowInput(SAMPLE_WORKFLOW);
  assert.equal(plan.schemaVersion, 1);
  assert.equal(plan.nodes[2].dependsOn[0], 'approve');
  assert.equal(plan.nodes[2].kind, 'git');
});
test('malformed JSON, wrong root type, missing fields and oversized input are rejected locally', () => {
  for (const input of [
    '{broken', 'null', '[]', '"hello"', '{}', '{"schemaVersion":1}',
    '{"schemaVersion":"1","nodes":[]}',
  ]) assert.throws(() => parseWorkflowInput(input));
  assert.throws(() => parseWorkflowInput('x'.repeat(MAX_WORKFLOW_BYTES + 1)), /128 KiB/);
});
test('native result is reported after validation, no executor action is offered', async () => {
  const events = [];
  const calls = [];
  const controller = createWorkflowValidation(async plan => {
    calls.push(plan);
    return validResult;
  }, state => events.push(state));
  await controller.check(SAMPLE_WORKFLOW);
  assert.deepEqual(events.map(event => event.phase), ['validating', 'valid']);
  assert.deepEqual(events.at(-1).result, validResult);
  assert.equal(calls.length, 1);
});
test('native rejection is shown without optimistic valid state', async () => {
  const events = [];
  const controller = createWorkflowValidation(async () => { throw new Error('Cycle detected.'); },
    state => events.push(state));
  await controller.check(SAMPLE_WORKFLOW);
  assert.deepEqual(events.map(event => event.phase), ['validating', 'invalid']);
  assert.match(events.at(-1).message, /Cycle/);
});
test('local parse rejection never calls trusted native API', async () => {
  const events = [];
  const controller = createWorkflowValidation(async () => {
    throw new Error('Must not be called');
  }, state => events.push(state));
  await controller.check('{nope}');
  assert.deepEqual(events.map(event => event.phase), ['invalid']);
  assert.match(events[0].message, /Invalid JSON/);
});
test('stale successful native response is ignored after user edits a plan', async () => {
  const events = [];
  let finish;
  const controller = createWorkflowValidation(() => new Promise(resolve => { finish = resolve; }),
    state => events.push(state));
  const waiting = controller.check(SAMPLE_WORKFLOW);
  controller.invalidate();
  finish(validResult);
  await waiting;
  assert.deepEqual(events.map(event => event.phase), ['validating', 'idle']);
});
test('late native error is ignored after dialog disposal', async () => {
  const events = [];
  let fail;
  const controller = createWorkflowValidation(() => new Promise((_, reject) => { fail = reject; }),
    state => events.push(state));
  const waiting = controller.check(SAMPLE_WORKFLOW);
  controller.dispose();
  fail(new Error('late error'));
  await waiting;
  assert.deepEqual(events.map(event => event.phase), ['validating']);
});
test('malicious source text remains plan data rather than executable markup', () => {
  const hostile = '<img src=x onerror=alert(1)>';
  const input = JSON.stringify({
    schemaVersion: 1,
    nodes: [{ id: 'inspect', kind: 'condition', title: hostile }],
  });
  const parsed = parseWorkflowInput(input);
  assert.equal(parsed.nodes[0].title, hostile);
});
