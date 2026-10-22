/** Nexora MCP inventory editor. This screen never starts a server or invokes a tool. */
import './mcp-center.css';
import { call, preview } from './bridge';
import { escape } from './dom';
import {
  asDraft, parseMcpDraft, transportSummary, type McpDraft, type McpServer,
} from './mcp-center-model';

const errorMessage = (error: unknown): string =>
  error instanceof Error ? error.message : String(error);

export function mountMcpCenter(host: HTMLElement): () => void {
  let disposed = false;
  let loading = false;
  let records: McpServer[] = [];
  let editingName: string | null = null;

  host.innerHTML = [
    '<section class="mcp-center" aria-label="Nexora MCP registry">',
    '<div class="mcp-center-head"><h3>Nexora MCP Registry</h3>',
    '<div><button type="button" class="secondary-button" data-nexora-mcp="refresh">Refresh</button>',
    '<button type="button" class="secondary-button" data-nexora-mcp="new">Add server</button></div></div>',
    '<p class="integration-note">Saved configuration only. Turning a server on does <strong>not</strong> start it,',
    'connect to it, or give any agent permission to use its tools. MCP execution is not available yet.</p>',
    '<p class="mcp-center-status" role="status" aria-live="polite"></p>',
    '<div class="mcp-center-list" data-nexora-mcp-list></div>',
    '<form class="mcp-center-form" data-nexora-mcp-form hidden>',
    '<h4 data-nexora-mcp-form-title>Add MCP server configuration</h4>',
    '<label for="nexora-mcp-name">Name</label>',
    '<input id="nexora-mcp-name" name="name" pattern="[a-z][a-z0-9_-]*" maxlength="64" required',
    ' autocomplete="off" spellcheck="false" placeholder="filesystem">',
    '<label for="nexora-mcp-description">Description</label>',
    '<input id="nexora-mcp-description" name="description" maxlength="500" autocomplete="off">',
    '<label for="nexora-mcp-kind">Transport</label>',
    '<select id="nexora-mcp-kind" name="kind">',
    '<option value="stdio">Local stdio (configuration only)</option>',
    '<option value="streamable_http">Streamable HTTP (configuration only)</option></select>',
    '<div data-nexora-stdio-fields>',
    '<label for="nexora-mcp-command">Executable name or absolute path</label>',
    '<input id="nexora-mcp-command" name="command" maxlength="1024" autocomplete="off" placeholder="npx">',
    '<label for="nexora-mcp-args">Arguments (JSON array of strings, not a shell command)</label>',
    '<textarea id="nexora-mcp-args" name="argsJson" rows="3" spellcheck="false">[]</textarea></div>',
    '<div data-nexora-http-fields hidden>',
    '<label for="nexora-mcp-url">Server URL (HTTPS or local loopback)</label>',
    '<input id="nexora-mcp-url" name="url" type="url" maxlength="2048" autocomplete="off"',
    ' placeholder="https://example.org/mcp"></div>',
    '<p class="integration-note">Do not enter API keys or passwords. Secure credential support is pending.</p>',
    '<div class="mcp-center-form-actions"><button type="submit" class="primary-button">Save configuration</button>',
    '<button type="button" class="secondary-button" data-nexora-mcp="cancel">Cancel</button></div>',
    '</form></section>',
  ].join('');

  const root = host.querySelector<HTMLElement>('.mcp-center')!;
  const list = root.querySelector<HTMLElement>('[data-nexora-mcp-list]')!;
  const status = root.querySelector<HTMLElement>('.mcp-center-status')!;
  const form = root.querySelector<HTMLFormElement>('[data-nexora-mcp-form]')!;
  const kind = form.querySelector<HTMLSelectElement>('[name="kind"]')!;
  const field = <T extends HTMLInputElement | HTMLTextAreaElement>(name: string): T =>
    form.querySelector<T>('[name="' + name + '"]')!;
  const alive = () => !disposed && host.isConnected;

  function showStatus(message: string, error = false): void {
    if (!alive()) return;
    status.textContent = message;
    status.classList.toggle('mcp-center-error', error);
  }

  function render(): void {
    if (!alive()) return;
    list.innerHTML = records.length ? records.map(server => {
      const name = escape(server.name);
      const action = server.enabled ? 'Disable' : 'Enable';
      return '<div class="mcp-center-entry"><div><strong>' + name +
        '</strong><small>' + escape(server.description || transportSummary(server)) +
        '</small><small>' + escape(transportSummary(server)) + '</small>' +
        '<small>' + (server.enabled ? 'Enabled in registry' : 'Disabled in registry') +
        ' · Not connected</small></div><div class="mcp-center-entry-actions">' +
        '<button type="button" class="secondary-button" data-nexora-mcp="toggle" data-name="' + name +
        '">' + action + '</button>' +
        '<button type="button" class="secondary-button" data-nexora-mcp="edit" data-name="' + name +
        '">Edit</button><button type="button" class="secondary-button" data-nexora-mcp="remove" data-name="' +
        name + '">Remove</button></div></div>';
    }).join('') : '<p class="integration-note">No Nexora MCP servers configured. Codex-managed servers above are separate.</p>';
  }

  function switchTransport(): void {
    root.querySelector<HTMLElement>('[data-nexora-stdio-fields]')!.hidden = kind.value !== 'stdio';
    root.querySelector<HTMLElement>('[data-nexora-http-fields]')!.hidden = kind.value !== 'streamable_http';
  }

  function edit(server?: McpServer): void {
    editingName = server?.name ?? null;
    form.reset();
    field<HTMLInputElement>('name').readOnly = !!server;
    form.querySelector<HTMLElement>('[data-nexora-mcp-form-title]')!.textContent =
      server ? 'Edit ' + server.name : 'Add MCP server configuration';
    const draft = server ? asDraft(server) : {
      name: '', description: '', kind: 'stdio', command: '', argsJson: '[]', url: '',
    } satisfies McpDraft;
    field<HTMLInputElement>('name').value = draft.name;
    field<HTMLInputElement>('description').value = draft.description;
    kind.value = draft.kind;
    field<HTMLInputElement>('command').value = draft.command;
    field<HTMLTextAreaElement>('argsJson').value = draft.argsJson;
    field<HTMLInputElement>('url').value = draft.url;
    form.hidden = false;
    switchTransport();
    field<HTMLInputElement>('name').focus();
  }

  async function mutate(fn: () => Promise<McpServer[]>, onSuccess: string): Promise<void> {
    if (loading || preview) return;
    loading = true;
    root.querySelectorAll<HTMLButtonElement>('button').forEach(button => { button.disabled = true; });
    showStatus('Saving configuration…');
    try {
      const next = await fn();
      if (!alive()) return;
      records = next;
      render();
      form.hidden = true;
      editingName = null;
      showStatus(onSuccess);
    } catch (error) {
      showStatus('Could not save: ' + errorMessage(error), true);
    } finally {
      loading = false;
      if (alive()) root.querySelectorAll<HTMLButtonElement>('button').forEach(button => { button.disabled = false; });
    }
  }

  async function refresh(): Promise<void> {
    if (loading || preview) return;
    loading = true;
    showStatus('Reading Nexora MCP registry…');
    try {
      const next = await call<McpServer[]>('mcp_registry_list');
      if (!alive()) return;
      records = next;
      render();
      showStatus(records.length + ' saved configuration(s). No servers started.');
    } catch (error) {
      showStatus('MCP registry unavailable: ' + errorMessage(error), true);
    } finally {
      loading = false;
    }
  }

  function click(event: Event): void {
    const button = (event.target as HTMLElement).closest<HTMLButtonElement>('button[data-nexora-mcp]');
    if (!button || !root.contains(button) || loading) return;
    const action = button.dataset.nexoraMcp;
    if (action === 'refresh') { void refresh(); return; }
    if (action === 'new') { edit(); return; }
    if (action === 'cancel') { form.hidden = true; editingName = null; return; }
    const server = records.find(record => record.name === button.dataset.name);
    if (!server) return;
    if (action === 'edit') { edit(server); return; }
    if (action === 'toggle') {
      void mutate(() => call<McpServer[]>('mcp_registry_set_enabled', {
        name: server.name, enabled: !server.enabled,
      }), 'Saved. This does not start or stop any MCP server.');
    }
    if (action === 'remove' && window.confirm('Remove saved MCP configuration "' + server.name + '"?')) {
      void mutate(() => call<McpServer[]>('mcp_registry_remove', { name: server.name }),
        'MCP configuration removed. No server process was affected.');
    }
  }

  function submit(event: Event): void {
    event.preventDefault();
    if (loading || preview) return;
    try {
      const draft: McpDraft = {
        name: field<HTMLInputElement>('name').value,
        description: field<HTMLInputElement>('description').value,
        kind: kind.value as McpDraft['kind'],
        command: field<HTMLInputElement>('command').value,
        argsJson: field<HTMLTextAreaElement>('argsJson').value,
        url: field<HTMLInputElement>('url').value,
      };
      const existing = records.find(server => server.name === editingName);
      const server = parseMcpDraft(draft, existing?.enabled ?? false);
      if (!editingName && records.some(record => record.name === server.name)) {
        showStatus('That name already exists. Choose Edit to modify its configuration.', true);
        return;
      }
      void mutate(() => call<McpServer[]>('mcp_registry_upsert', { server }),
        'MCP configuration saved. No server was started or granted tool access.');
    } catch (error) {
      showStatus(errorMessage(error), true);
    }
  }

  root.addEventListener('click', click);
  kind.addEventListener('change', switchTransport);
  form.addEventListener('submit', submit);
  if (preview) {
    showStatus('Browser preview: native MCP inventory is unavailable; no server changes can be saved.');
    root.querySelectorAll<HTMLButtonElement>('button').forEach(button => { button.disabled = true; });
  } else {
    void refresh();
  }
  return () => {
    disposed = true;
    root.removeEventListener('click', click);
    kind.removeEventListener('change', switchTransport);
    form.removeEventListener('submit', submit);
  };
}
