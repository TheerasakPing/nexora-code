import { expect, test } from '@playwright/test';

test('Nexora MCP inspector renders inert remote names and disconnects after inspection', async ({page}) => {
  await page.goto('/tests/fixtures/nexora-mcp-inspector.html');
  const root=page.locator('.mcp-center');
  await expect(root.getByText('approved_mock', {exact:true})).toBeVisible();
  await expect(root.getByRole('button', {name:'Inspect catalogs'})).toHaveCount(3);
  const buttons=root.getByRole('button', {name:'Inspect catalogs'});
  await expect(buttons.nth(0)).toBeEnabled();
  await expect(buttons.nth(1)).toBeDisabled();
  await expect(buttons.nth(2)).toBeDisabled();
  await buttons.nth(0).click();
  await expect(root.locator('.mcp-center-inspector li')).toHaveCount(3);
  await expect(root.getByText('Inspection ended; server disconnected')).toBeVisible();
  expect(await page.evaluate(()=> (window as any).__mcpInjected)).toBe(false);
  expect(await root.locator('.mcp-center-inspector img,.mcp-center-inspector svg').count()).toBe(0);
  const calls=await page.evaluate(()=> (window as any).__mcpCalls);
  expect(calls.filter((c:any)=>c.command==='mcp_inspect_catalog')).toEqual([{
    command:'mcp_inspect_catalog',
    args:{serverName:'approved_mock',protocolVersion:'2026-07-28'},
  }]);
});

test('legacy inspection requests explicit compatibility version; disabled and vault-backed configs remain non-inspectable', async ({page}) => {
  await page.goto('/tests/fixtures/nexora-mcp-inspector.html');
  const root=page.locator('.mcp-center');
  const legacy=root.getByRole('button',{name:'Inspect legacy'});
  await expect(legacy).toHaveCount(2);
  await expect(legacy.nth(0)).toBeEnabled();
  await expect(legacy.nth(1)).toBeDisabled();
  await legacy.nth(0).click();
  await expect(root.locator('.mcp-center-inspector li')).toHaveCount(3);
  const called=await page.evaluate(()=> (window as any).__mcpCalls);
  expect(called.some((x:any)=>x.command==='mcp_inspect_catalog' &&
    x.args?.protocolVersion==='2025-11-25')).toBe(true);
});
