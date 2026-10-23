/** Nexora MCP registry and explicitly authorized manual tool console. No automatic agent execution. */
import './mcp-center.css';
import { call, preview } from './bridge';
import { escape } from './dom';
import {
  asDraft, parseMcpDraft, transportSummary, type McpDraft, type McpServer,
} from './mcp-center-model';

interface McpCatalogItem {
  kind: 'tool' | 'resource' | 'prompt';
  name: string;
  label: string;
}
interface McpVaultChangeResult {
  server: string;
  hasCredential: boolean;
  cleanupWarning: boolean;
}
interface McpToolExecutionResult {
  server: string;
  tool: string;
  result: unknown;
  status: 'completed_disconnect';
}
interface McpCatalogSnapshot {
  server: string;
  protocol: string;
  status: 'inspected_disconnect';
  items: McpCatalogItem[];
}

const errorMessage = (error: unknown): string =>
  error instanceof Error ? error.message : String(error);

export function mountMcpCenter(host: HTMLElement): () => void {
  let disposed = false;
  let loading = false;
  let records: McpServer[] = [];
  let editingName: string | null = null;
  let vaultServerName: string | null = null;
  let inspected: { server: string; protocolVersion: string; tools: Set<string> } | null = null;

  host.innerHTML = [
    '<section class="mcp-center" aria-label="Nexora MCP registry">',
    '<div class="mcp-center-head"><h3>Nexora MCP Registry</h3>',
    '<div><button type="button" class="secondary-button" data-nexora-mcp="refresh">Refresh</button>',
    '<button type="button" class="secondary-button" data-nexora-mcp="new">Add server</button></div></div>',
    '<p class="integration-note">Enabling a server only saves configuration. Inspect catalogs requires a separate native approval each time.',
    ' Manual <strong>Run Once</strong> tool calls are available only after a fresh native approval with full JSON arguments. No automatic agent tool access.</p>',
    '<p class="mcp-center-status" role="status" aria-live="polite"></p>',
    '<div class="mcp-center-list" data-nexora-mcp-list></div>',
    '<form class="mcp-center-vault" data-nexora-mcp-vault hidden>',
    '<h4>OS Vault — Store / Rotate MCP HTTP Bearer Token</h4>',
    '<p class="integration-note">Token passes through local Tauri IPC once and is stored in the OS secure store after native approval. It is never displayed again.</p>',
    '<label for="nexora-mcp-token">HTTP Bearer token for <span data-nexora-vault-server></span></label>',
    '<input id="nexora-mcp-token" name="secret" type="password" minlength="1" maxlength="4096" autocomplete="new-password" required>',
    '<div><button type="submit" class="primary-button">Request Approval &amp; Store</button>',
    '<button type="button" class="secondary-button" data-nexora-mcp="cancel-vault">Cancel</button></div></form>',
    '<section class="mcp-center-inspector" data-nexora-mcp-inspector hidden aria-live="polite"></section>',
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
    '<p class="integration-note">Do not enter tokens in server URLs or command arguments. Use the separate OS Vault Manager after saving HTTP configuration.</p>',
    '<div class="mcp-center-form-actions"><button type="submit" class="primary-button">Save configuration</button>',
    '<button type="button" class="secondary-button" data-nexora-mcp="cancel">Cancel</button></div>',
    '</form></section>',
  ].join('');

  const root = host.querySelector<HTMLElement>('.mcp-center')!;
  const list = root.querySelector<HTMLElement>('[data-nexora-mcp-list]')!;
  const status = root.querySelector<HTMLElement>('.mcp-center-status')!;
  const inspectorPanel = root.querySelector<HTMLElement>('[data-nexora-mcp-inspector]')!;
  const vaultForm = root.querySelector<HTMLFormElement>('[data-nexora-mcp-vault]')!;
  const vaultSecret = vaultForm.querySelector<HTMLInputElement>('[name="secret"]')!;
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
        '">Edit</button>' +
        (server.transport.kind === 'streamable_http' ?
          '<button type="button" class="secondary-button" data-nexora-mcp="vault-store" data-name="' + name +
          '">' + (server.credentialRef ? 'Rotate token' : 'Store token') + '</button>' +
          (server.credentialRef ?
            '<button type="button" class="secondary-button" data-nexora-mcp="vault-revoke" data-name="' + name +
            '">Revoke token</button>' : '') : '') +
        '<button type="button" class="secondary-button" data-nexora-mcp="inspect" data-name="' + name +
        '"' + (!server.enabled ? ' disabled' : '') + '>Inspect catalogs</button>' +
        (server.transport.kind === 'stdio' ? '<button type="button" class="secondary-button" data-nexora-mcp="inspect-legacy" data-name="' + name +
        '"' + (!server.enabled ? ' disabled' : '') + '>Inspect legacy</button>' : '') +
        '<button type="button" class="secondary-button" data-nexora-mcp="remove" data-name="' +
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
      inspectorPanel.hidden = true;
      inspectorPanel.replaceChildren();
      inspected = null;
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

  async function inspect(server: McpServer, protocolVersion: string): Promise<void> {
    if (loading || preview || !server.enabled) return;
    loading = true;
    root.querySelectorAll<HTMLButtonElement>('button').forEach(button => { button.disabled = true; });
    inspectorPanel.hidden = true;
    inspectorPanel.replaceChildren();
    inspected = null;
    showStatus('Requesting one-time native approval for ' + server.name + '…');
    try {
      const result = await call<McpCatalogSnapshot>('mcp_inspect_catalog', {
        serverName: server.name,
        protocolVersion,
      });
      if (!alive()) return;
      if (result.server !== server.name || result.status !== 'inspected_disconnect' ||
          !Array.isArray(result.items) || result.items.length > 768) {
        throw new Error('Invalid MCP inspector response');
      }
      // No remote-controlled HTML or scripts; all text is escaped and shown
      // as inert labels only. The catalog never creates executable buttons.
      const grouped = (['tool', 'resource', 'prompt'] as const).map(kind => {
        const entries = result.items.filter(item => item.kind === kind);
        return '<div><strong>' + ({ tool: 'Tools', resource: 'Resources', prompt: 'Prompts' })[kind] +
          ' (' + entries.length + ')</strong>' +
          (entries.length ? '<ul>' + entries.map(item => '<li><span>' + escape(item.label) +
            '</span> <small>' + escape(item.name) + '</small>' +
            (kind === 'tool' ?
              '<div class="mcp-center-manual-tool"><label>JSON arguments for ' + escape(item.name) +
              '<textarea data-nexora-tool-args rows="3" maxlength="2048" spellcheck="false" aria-label="Arguments for ' +
              escape(item.name) + '">{}</textarea></label>' +
              '<button type="button" class="secondary-button" data-nexora-mcp="run-tool" data-tool="' +
              escape(item.name) + '">Run Once — native approval required</button>' +
              '<pre data-nexora-tool-result hidden aria-live="polite"></pre></div>' : '') +
            '</li>').join('') + '</ul>'
            : '<p class="integration-note">None advertised</p>') + '</div>';
      });
      inspected = {
        server: server.name,
        protocolVersion,
        tools: new Set(result.items.filter(item => item.kind === 'tool').map(item => item.name)),
      };
      inspectorPanel.innerHTML = '<h4>MCP catalog — ' + escape(server.name) +
        ' <small>' + escape(result.protocol) + '</small></h4>' +
        '<p class="integration-note">Inspection ended; server disconnected. No tools were executed.</p>' +
        grouped.join('');
      inspectorPanel.hidden = false;
      showStatus('Inspected ' + result.items.length + ' catalog item(s); connection closed.');
    } catch (error) {
      if (alive()) showStatus('MCP inspection denied or failed: ' + errorMessage(error), true);
    } finally {
      loading = false;
      if (alive()) {
        render();
        root.querySelectorAll<HTMLButtonElement>('button').forEach(button => {
          button.disabled = preview || button.disabled && (button.dataset.nexoraMcp === 'inspect' ||
            button.dataset.nexoraMcp === 'inspect-legacy') &&
            !!records.find(item => item.name === button.dataset.name &&
              !item.enabled);
        });
      }
    }
  }

  async function runTool(button: HTMLButtonElement): Promise<void> {
    if (loading || preview || !inspected) return;
    const toolName = button.dataset.tool ?? '';
    const target = records.find(item => item.name === inspected?.server);
    if (!inspected.tools.has(toolName) || !target || !target.enabled) {
      showStatus('Tool execution blocked: inspect an enabled uncredentialed server first.', true);
      return;
    }
    const area = button.closest<HTMLElement>('.mcp-center-manual-tool');
    const json = area?.querySelector<HTMLTextAreaElement>('[data-nexora-tool-args]')?.value ?? '';
    let args: unknown;
    try {
      args = JSON.parse(json);
      if (!args || typeof args !== 'object' || Array.isArray(args) || json.length > 2048) {
        throw new Error('Arguments must be a JSON object up to 2048 characters.');
      }
    } catch (error) {
      showStatus('Invalid tool arguments: ' + errorMessage(error), true);
      return;
    }
    const context = inspected;
    loading = true;
    root.querySelectorAll<HTMLButtonElement>('button').forEach(item => { item.disabled = true; });
    const output = area?.querySelector<HTMLElement>('[data-nexora-tool-result]');
    showStatus('Waiting for one-time native approval of ' + toolName + '…');
    try {
      const reply = await call<McpToolExecutionResult>('mcp_execute_tool_once', {
        serverName: context.server,
        protocolVersion: context.protocolVersion,
        toolName,
        arguments: args,
      });
      if (!alive()) return;
      if (reply.status !== 'completed_disconnect' || reply.server !== context.server ||
          reply.tool !== toolName) {
        throw new Error('Invalid MCP tool result');
      }
      if (output) {
        const text = JSON.stringify(reply.result, null, 2);
        output.textContent = text.length > 16384 ? text.slice(0, 16384) + '\n… (display truncated)' : text;
        output.hidden = false;
      }
      const toolReportedError = !!reply.result && typeof reply.result === 'object' &&
        'isError' in reply.result && reply.result.isError === true;
      showStatus(
        'MCP tool ' + toolName + (toolReportedError ? ' returned an error' : ' completed') +
        '; native approval consumed, connection closed.',
        toolReportedError,
      );
    } catch (error) {
      if (alive()) showStatus('MCP tool denied or failed: ' + errorMessage(error), true);
    } finally {
      loading = false;
      if (alive()) {
        render();
        root.querySelectorAll<HTMLButtonElement>('button').forEach(item => {
          item.disabled = preview || (item.dataset.nexoraMcp === 'inspect' ||
            item.dataset.nexoraMcp === 'inspect-legacy') &&
            !!records.find(entry => entry.name === item.dataset.name &&
              !entry.enabled);
        });
      }
    }
  }

  async function vaultChange(server: McpServer, operation: 'store' | 'revoke', secret?: string): Promise<void> {
    if (loading || preview) return;
    loading = true;
    vaultSecret.value = '';
    vaultForm.hidden = true;
    vaultServerName = null;
    showStatus('Waiting for OS Vault native approval…');
    try {
      const response = await call<McpVaultChangeResult>('mcp_vault_change', {
        serverName: server.name, operation,
        ...(secret === undefined ? {} : { secret }),
      });
      const next = await call<McpServer[]>('mcp_registry_list');
      if (!alive()) return;
      records = next;
      inspectorPanel.hidden = true;
      inspectorPanel.replaceChildren();
      inspected = null;
      render();
      showStatus('MCP token ' + (response.hasCredential ? 'stored / rotated' : 'revoked') +
        (response.cleanupWarning ? '; old OS vault entry cleanup failed — inspect OS credential manager.' : '.'),
        response.cleanupWarning);
    } catch (error) {
      if (alive()) showStatus('MCP vault change denied or failed: ' + errorMessage(error), true);
    } finally {
      loading = false;
      if (alive()) render();
    }
  }

  function submitVault(event: Event): void {
    event.preventDefault();
    const server = records.find(entry => entry.name === vaultServerName);
    const secret = vaultSecret.value;
    vaultSecret.value = '';
    if (!server || server.transport.kind !== 'streamable_http') {
      showStatus('Select a saved HTTP server first.', true);
      return;
    }
    if (secret.length < 1 || secret.length > 4096) {
      showStatus('Bearer token length must be 1-4096 characters.', true);
      return;
    }
    void vaultChange(server, 'store', secret);
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
    if (action === 'run-tool') { void runTool(button); return; }
    if (action === 'cancel-vault') {
      vaultSecret.value = '';
      vaultServerName = null;
      vaultForm.hidden = true;
      return;
    }
    if (action === 'refresh') { void refresh(); return; }
    if (action === 'new') { edit(); return; }
    if (action === 'cancel') { form.hidden = true; editingName = null; return; }
    const server = records.find(record => record.name === button.dataset.name);
    if (!server) return;
    if (action === 'inspect' || action === 'inspect-legacy') {
      void inspect(server, action === 'inspect-legacy' ? '2025-11-25' : '2026-07-28');
      return;
    }
    if (action === 'vault-store' && server.transport.kind === 'streamable_http') {
      vaultServerName = server.name;
      vaultSecret.value = '';
      vaultForm.querySelector<HTMLElement>('[data-nexora-vault-server]')!.textContent = server.name;
      vaultForm.hidden = false;
      vaultSecret.focus();
      return;
    }
    if (action === 'vault-revoke' && server.credentialRef) {
      void vaultChange(server, 'revoke');
      return;
    }
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
        credentialRef: records.find(server => server.name === editingName)?.credentialRef,
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
  vaultForm.addEventListener('submit', submitVault);
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
    vaultForm.removeEventListener('submit', submitVault);
    vaultSecret.value = '';
  };
}
