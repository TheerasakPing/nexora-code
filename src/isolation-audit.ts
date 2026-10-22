/** Read-only native Team checkout inspection. Never interpreted as tool authority. */
export interface IsolationWorker {
  workerId: string;
  name: string;
  classification: string;
  checkoutPath: string | null;
  reason: string;
}
export interface IsolationAudit {
  revision: number;
  isolatedCount: number;
  sharedCount: number;
  unverifiedCount: number;
  workers: IsolationWorker[];
  note: string;
}
const escapeText = (value: string): string => value.replace(/[&<>"']/g, character => ({
  '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;',
}[character]!));

const labels: Record<string, string> = {
  shared_lead: 'Shared with Team Lead',
  shared_workers: 'Shared between workers',
  verified_linked_worktree: 'Registered Git worktree',
  unverified: 'Not verified',
};

/** Pure safe markup from untrusted worker names/reasons and native path metadata. */
export function isolationAuditMarkup(audit: IsolationAudit, currentRevision: number): string {
  const number = (count: number): string => Number.isSafeInteger(count) && count >= 0 ? String(count) : '?';
  const entries = audit.workers.map(worker =>
    '<li><strong>' + escapeText(worker.name) + '</strong>'
    + '<span>' + escapeText(labels[worker.classification] ?? 'Unknown classification') + '</span>'
    + '<small>' + escapeText(worker.reason) + '</small>'
    + (worker.checkoutPath ? '<code>' + escapeText(worker.checkoutPath) + '</code>' : '')
    + '</li>',
  ).join('');
  return '<div class="architect-isolation-result">'
    + '<p class="architect-preview-lede">Read-only checkout audit · '
    + number(audit.isolatedCount) + ' verified · ' + number(audit.sharedCount)
    + ' shared · ' + number(audit.unverifiedCount) + ' unverified</p>'
    + (audit.revision !== currentRevision
      ? '<p class="architect-preview-stale" role="status">Team changed since this audit. Refresh to verify again.</p>'
      : '')
    + (entries ? '<ul class="architect-isolation-workers">' + entries + '</ul>'
      : '<p>No workers to inspect in this team.</p>')
    + '<p class="architect-preview-note">' + escapeText(audit.note) + '</p>'
    + '</div>';
}
