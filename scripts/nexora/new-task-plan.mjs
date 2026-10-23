#!/usr/bin/env node
/** Create a new per-request NC-TEAM-001 planner; never overwrite existing work. */
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(fileURLToPath(new URL('../../', import.meta.url)));
const taskDir = join(root, 'docs', 'nexora', 'tasks');
const template = join(root, 'docs', 'nexora', 'templates', 'TASK-CHECKLIST.md');

export function createTaskPlan({ date, slug }, options = {}) {
  if (typeof date !== 'string' || !/^\d{4}-\d{2}-\d{2}$/.test(date) ||
      Number.isNaN(Date.parse(date + 'T00:00:00Z')) ||
      new Date(date + 'T00:00:00Z').toISOString().slice(0, 10) !== date) {
    throw new Error('Date must be a real YYYY-MM-DD calendar date.');
  }
  if (typeof slug !== 'string' || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(slug) || slug.length > 70) {
    throw new Error('Slug must be lowercase kebab-case (1–70 characters).');
  }
  const directory = options.directory ?? taskDir;
  const source = options.template ?? template;
  const destination = join(directory, date + '-' + slug + '.md');
  // An exclusive create means a second invocation cannot overwrite a planner.
  const raw = readFileSync(source, 'utf8');
  if (!raw.startsWith('# Task Checklist Planner')) throw new Error('Invalid task planner template.');
  const content = raw.replace('YYYY-MM-DD-task-slug', date + '-' + slug);
  mkdirSync(directory, { recursive: true });
  writeFileSync(destination, content, { encoding: 'utf8', flag: 'wx', mode: 0o600 });
  return destination;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    const [date, slug, ...extra] = process.argv.slice(2);
    if (extra.length || !date || !slug) throw new Error('Usage: node scripts/nexora/new-task-plan.mjs YYYY-MM-DD task-slug');
    console.log('Created: ' + createTaskPlan({ date, slug }));
  } catch (error) {
    console.error('Planner not created: ' + (error instanceof Error ? error.message : String(error)));
    process.exitCode = 1;
  }
}
