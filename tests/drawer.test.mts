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

test("Open Trigger opens profile Drawer; inside clicks stay open and outside clicks close", async () => {
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

    const initialLayout = await page
      .locator("#terminal_ratzilla_grid pre")
      .evaluateAll((rows) => {
        const locate = (text: string) => {
          const row = rows.findIndex((line) =>
            line.textContent?.includes(text)
          );
          const content = rows[row]?.textContent ?? "";
          return { row, column: content.indexOf(text), length: text.length };
        };
        return {
          trigger: locate("Open Trigger"),
          hintTitle: locate("Opens the profile drawer"),
          hintDismiss: locate("Click outside or press Esc to close"),
        };
      });
    assert.ok(initialLayout.trigger.row >= 0);
    assert.ok(initialLayout.hintTitle.row >= initialLayout.trigger.row + 2);
    assert.ok(initialLayout.hintDismiss.row > initialLayout.hintTitle.row);
    const triggerCenter =
      initialLayout.trigger.column + initialLayout.trigger.length / 2;
    for (const hint of [initialLayout.hintTitle, initialLayout.hintDismiss]) {
      assert.ok(
        Math.abs(hint.column + hint.length / 2 - triggerCenter) <= 1,
        `hint should align with trigger: ${JSON.stringify(initialLayout)}`
      );
    }

    await clickTerminalText(page, "Open Trigger");
    await waitForTerminalText(page, "Profile");
    await clickTerminalText(page, "alex@example.com");
    await page.waitForTimeout(100);
    assert.ok(
      (await page.locator("#terminal_ratzilla_grid").textContent())?.includes("Profile"),
      "click inside drawer must leave it open"
    );

    // The short hint remains visible to the left of the open right drawer.
    await clickTerminalText(page, "Click outside");
    await page.waitForFunction(
      () =>
        !(
          document
            .querySelector("#terminal_ratzilla_grid")
            ?.textContent?.includes("Profile") ?? false
        )
    );

    await clickTerminalText(page, "Open Trigger");
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
