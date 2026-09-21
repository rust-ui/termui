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

test("profile Drawer opens from 👤; inside clicks stay open and outside clicks close", async () => {
  const page = await session.browser.newPage({
    viewport: { width: 1200, height: 762 },
  });
  const pageErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error.message));

  try {
    await page.goto(
      `${session.baseUrl}/demos/interactive.html?demo=drawer-interactive`
    );
    await waitForTerminal(page);
    assert.equal(await page.title(), "Interactive Ratatui Drawer");

    await clickTerminalText(page, "👤");
    await waitForTerminalText(page, "Profile");
    await clickTerminalText(page, "alex@example.com");
    await page.waitForTimeout(100);
    assert.ok(
      (await page.locator("#terminal_ratzilla_grid").textContent())?.includes("Profile"),
      "click inside drawer must leave it open"
    );

    // Only the hint's left fragment stays visible beside the open right drawer.
    await clickTerminalText(page, "Click");
    await page.waitForFunction(
      () =>
        !(
          document
            .querySelector("#terminal_ratzilla_grid")
            ?.textContent?.includes("Profile") ?? false
        )
    );

    await clickTerminalText(page, "👤");
    await waitForTerminalText(page, "Profile");
    await clickTerminalText(page, " x ");
    await page.waitForFunction(
      () =>
        !(
          document
            .querySelector("#terminal_ratzilla_grid")
            ?.textContent?.includes("Profile") ?? false
        )
    );

    // Let the 180 ms exit animation finish before testing keyboard reopen.
    await page.waitForTimeout(250);
    await page.keyboard.press("Enter");
    await waitForTerminalText(page, "Profile");
    await page.keyboard.press("Escape");
    await page.waitForFunction(
      () =>
        !(
          document
            .querySelector("#terminal_ratzilla_grid")
            ?.textContent?.includes("Profile") ?? false
        )
    );
    assert.deepEqual(pageErrors, []);
  } finally {
    await page.close();
  }
});
