/** UI contracts for the Nexora-owned MCP inventory (no server execution). */
export type McpTransport =
  | { kind: 'stdio'; command: string; args: string[] }
  | { kind: 'streamable_http'; url: string };

export interface McpServer {
  name: string;
  description: string;
  enabled: boolean;
  transport: McpTransport;
}

export interface McpDraft {
  name: string;
  description: string;
  kind: McpTransport['kind'];
  command: string;
  argsJson: string;
  url: string;
}

/** The backend is authoritative; this catches input errors before IPC. */
export function parseMcpDraft(draft: McpDraft, enabled = false): McpServer {
  const name = draft.name.trim();
  const description = draft.description.trim();
  if (!/^[a-z][a-z0-9_-]{0,63}$/.test(name)) {
    throw new Error('Server name must start with a lowercase letter and use only a-z, 0-9, _ or -.');
  }
  if (description.length > 500 || /[\x00-\x1f\x7f]/.test(description)) {
    throw new Error('Description is too long or contains control characters.');
  }
  if (draft.kind === 'stdio') {
    const command = draft.command.trim();
    if (!command || command.length > 1024 || /[\x00-\x1f\x7f]/.test(command)) {
      throw new Error('Enter a valid executable name or absolute path.');
    }
    const basename = command.replaceAll('\\', '/').split('/').pop()?.toLowerCase();
    if (['sh', 'bash', 'zsh', 'cmd', 'cmd.exe', 'powershell', 'powershell.exe', 'pwsh'].includes(basename ?? '')) {
      throw new Error('Shell executables cannot be configured as MCP servers.');
    }
    if (command.includes('..') && command.split(/[\\/]/).includes('..')) {
      throw new Error('MCP executable path must not traverse parent directories.');
    }
    let args: unknown;
    try { args = JSON.parse(draft.argsJson.trim() || '[]'); }
    catch { throw new Error('Arguments must be a JSON array of strings, not shell text.'); }
    if (!Array.isArray(args) || args.length > 48 ||
        args.some(value => typeof value !== 'string' || value.length > 4096 || /[\x00-\x1f\x7f]/.test(value))) {
      throw new Error('Arguments must be an array with at most 48 bounded strings.');
    }
    if (args.some(value => /--(?:token|api-key|password|secret)|authorization:|bearer /i.test(value))) {
      throw new Error('Never put credentials in MCP arguments. Credential vault support is pending.');
    }
    return { name, description, enabled, transport: { kind: 'stdio', command, args } };
  }
  if (draft.kind === 'streamable_http') {
    const url = draft.url.trim();
    let parsed: URL;
    try { parsed = new URL(url); }
    catch { throw new Error('Enter a valid MCP server URL.'); }
    if (url.length > 2048 || parsed.username || parsed.password || parsed.search || parsed.hash ||
        !['http:', 'https:'].includes(parsed.protocol) ||
        (parsed.protocol === 'http:' && !['localhost', '127.0.0.1', '[::1]'].includes(parsed.hostname))) {
      throw new Error('Use HTTPS (or loopback HTTP) without embedded credentials or query parameters.');
    }
    return { name, description, enabled, transport: { kind: 'streamable_http', url } };
  }
  throw new Error('Choose a supported MCP transport.');
}

export function transportSummary(server: McpServer): string {
  return server.transport.kind === 'stdio'
    ? 'stdio · ' + server.transport.command
    : 'HTTP · ' + server.transport.url;
}

export function asDraft(server: McpServer): McpDraft {
  return {
    name: server.name,
    description: server.description,
    kind: server.transport.kind,
    command: server.transport.kind === 'stdio' ? server.transport.command : '',
    argsJson: server.transport.kind === 'stdio' ? JSON.stringify(server.transport.args, null, 2) : '[]',
    url: server.transport.kind === 'streamable_http' ? server.transport.url : '',
  };
}
