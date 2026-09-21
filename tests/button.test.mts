import assert from "node:assert/strict";
import { after, before, test } from "node:test";
import {
  clickTerminalText,
  startInteractiveDemoSession,
  waitForTerminal,
  waitForTerminalText,
  type InteractiveDemoSession,
} from "./helpers/interactive-demo.ts";

let session: InteractiveDemoSession;

before(async () => {
  session = await startInteractiveDemoSession();
});

after(async () => {
  await session?.close();
});

test("Button responds to mouse and keyboard actions", async () => {
  const page = await session.browser.newPage({
    viewport: { width: 1200, height: 762 },
  });
  const pageErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error.message));

  try {
    await page.goto(`${session.baseUrl}/demos/button-interactive/index.html`);
    await waitForTerminal(page);
    assert.equal(await page.title(), "Interactive Ratatui Button");

    const terminal = page.locator("#terminal_ratzilla_grid");
    await clickTerminalText(page, "Increment +1");
    await waitForTerminalText(page, "Count: 1");
    assert.match(await terminal.innerText(), /Count:\s*1/);

    await clickTerminalText(page, "Reset");
    await waitForTerminalText(page, "Count: 0");
    assert.match(await terminal.innerText(), /Count:\s*0/);

    await page.keyboard.press("Space");
    await waitForTerminalText(page, "Count: 1");
    assert.match(await terminal.innerText(), /Count:\s*1/);

    await page.keyboard.press("r");
    await waitForTerminalText(page, "Count: 0");
    assert.match(await terminal.innerText(), /Count:\s*0/);
    assert.deepEqual(pageErrors, []);
  } finally {
    await page.close();
  }
});
