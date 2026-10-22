import type { TeamWorker } from './architect-types';

/** Read-only native R3 planner output. Never grant tool or worker execution from these values. */
export interface SchedulePreview {
  revision: number;
  maxParallel: number;
  activeWorkerIds: string[];
  waves: { workerIds: string[] }[];
  blocked: { workerId: string; reason: string }[];
  note: string;
}

const escaped = (value: string): string => value.replace(/[&<>"']/g, character => ({
  '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;',
}[character]!));

/** Pure HTML view. All native and worker-supplied strings are treated as untrusted. */
export function schedulePreviewMarkup(
  preview: SchedulePreview,
  workers: Pick<TeamWorker, 'id' | 'name'>[],
  currentRevision: number,
): string {
  const names = new Map(workers.map(worker => [worker.id, worker.name]));
  const name = (id: string): string => escaped(names.get(id) ?? id);
  const stale = preview.revision !== currentRevision;
  const active = preview.activeWorkerIds.length
    ? `<p class="architect-preview-running"><strong>Currently active:</strong> ${preview.activeWorkerIds.map(name).join(', ')}</p>`
    : '<p class="architect-preview-running">No active worker reported in this snapshot.</p>';
  const waves = preview.waves.length
    ? `<ol class="architect-preview-waves">${preview.waves.map((wave, index) =>
        `<li><strong>Wave ${index + 1}</strong><span>${wave.workerIds.map(name).join(', ') || 'No assignments'}</span></li>`).join('')}</ol>`
    : '<p>No queued tasks are ready to run in the projected waves.</p>';
  const blocked = preview.blocked.length
    ? `<section aria-label="Blocked assignments"><strong>Blocked (${preview.blocked.length})</strong><ul>${preview.blocked.map(item =>
        `<li><span>${name(item.workerId)}</span><small>${escaped(item.reason)}</small></li>`).join('')}</ul></section>`
    : '<p>No unresolved dependencies in this preview.</p>';
  return `<div class="architect-preview-result">`
    + `<p class="architect-preview-lede">Read-only estimate · up to ${Number.isSafeInteger(preview.maxParallel) && preview.maxParallel >= 0 ? preview.maxParallel : '?'} concurrent agents</p>`
    + (stale ? '<p class="architect-preview-stale" role="status">Team changed since this snapshot. Refresh the preview for current information.</p>' : '')
    + active + waves + blocked
    + `<p class="architect-preview-note">${escaped(preview.note)}</p>`
    + '</div>';
}
