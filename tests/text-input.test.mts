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

test("Text Input supports typing, field switching, deletion, and mouse focus", async () => {
  const page = await session.browser.newPage({
    viewport: { width: 1200, height: 762 },
  });
  const pageErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error.message));

  try {
    await page.goto(
      `${session.baseUrl}/demos/interactive.html?demo=text-input-interactive`
    );
    await waitForTerminal(page);
    assert.equal(await page.title(), "Interactive Ratatui Text Input");
    await waitForTerminalText(page, "ada@example.com");
    await waitForTerminalText(page, "Choose a name");

    await clickTerminalText(page, "Email");
    await page.keyboard.type("+work");
    await waitForTerminalText(page, "ada@example.com+work");

    await page.keyboard.press("Tab");
    await page.keyboard.type("termui");
    await waitForTerminalText(page, "termui");

    await page.keyboard.press("Backspace");
    await waitForTerminalText(page, "termu");

    await clickTerminalText(page, "Email");
    await page.keyboard.type(".dev");
    await waitForTerminalText(page, "ada@example.com+work.dev");

    assert.deepEqual(pageErrors, []);
  } finally {
    await page.close();
  }
});
