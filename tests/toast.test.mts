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

test("default ToastTrigger opens a bottom-right toast; close button and Escape dismiss it", async () => {
  const page = await session.browser.newPage({
    viewport: { width: 1200, height: 762 },
  });
  const pageErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error.message));

  try {
    await page.goto(`${session.baseUrl}/demos/interactive.html?demo=toast-interactive`);
    await waitForTerminal(page);
    assert.equal(await page.title(), "Interactive Ratatui Toast");

    const triggerLayout = await page
      .locator("#terminal_ratzilla_grid pre")
      .evaluateAll((rows) => {
        const row = rows.findIndex((line) => line.textContent?.includes("Show toast"));
        const text = rows[row]?.textContent ?? "";
        return {
          row,
          rowCount: rows.length,
          column: text.indexOf("Show toast"),
          hintColumn: rows
            .find((line) => line.textContent?.includes("Click Show toast"))
            ?.textContent?.indexOf("Click Show toast"),
          hintWidth: "Click Show toast or press Enter. Click x to dismiss.".length,
        };
      });
    assert.ok(
      triggerLayout.hintColumn !== undefined &&
        Math.abs(
          triggerLayout.column +
            5 -
            (triggerLayout.hintColumn + triggerLayout.hintWidth / 2),
        ) <= 1,
      `ToastTrigger should align with the centered hint: ${JSON.stringify(triggerLayout)}`,
    );
    assert.ok(
      Math.abs(triggerLayout.row + 1.5 - triggerLayout.rowCount / 2) <= 2,
      "ToastTrigger should be centered vertically",
    );

    await clickTerminalText(page, "Show toast");
    await waitForTerminalText(page, "Changes saved");
    await page.waitForTimeout(200);

    const toastLayout = await page
      .locator("#terminal_ratzilla_grid pre")
      .evaluateAll((rows) => {
        const row = rows.findIndex((line) => line.textContent?.includes("Changes saved"));
        const text = rows[row]?.textContent ?? "";
        return {
          row,
          rowCount: rows.length,
          column: text.indexOf("Changes saved"),
          width: text.length,
        };
      });
    assert.ok(
      toastLayout.row > triggerLayout.row,
      `Toast should appear below the centered trigger: ${JSON.stringify({ toastLayout, triggerLayout })}`,
    );
    assert.ok(toastLayout.column > toastLayout.width / 2);

    await clickTerminalText(page, " x ");

    await page.waitForFunction(
      () =>
        !(
          document
            .querySelector("#terminal_ratzilla_grid")
            ?.textContent?.includes("Changes saved") ?? false
        ),
    );

    await page.keyboard.press("Enter");
    await waitForTerminalText(page, "Changes saved");
    await page.keyboard.press("Escape");
    await page.waitForFunction(
      () =>
        !(
          document
            .querySelector("#terminal_ratzilla_grid")
            ?.textContent?.includes("Changes saved") ?? false
        ),
    );

    await clickTerminalText(page, "Show toast");
    await waitForTerminalText(page, "Changes saved");
    await clickTerminalText(page, "Click Show toast");
    await page.waitForFunction(
      () =>
        !(
          document
            .querySelector("#terminal_ratzilla_grid")
            ?.textContent?.includes("Changes saved") ?? false
        ),
    );
    assert.deepEqual(pageErrors, []);
  } finally {
    await page.close();
  }
});

test("non-dismissible ToastTrigger opens toast that Escape cannot close", async () => {
  const page = await session.browser.newPage();
  try {
    await page.goto(
      `${session.baseUrl}/demos/interactive.html?demo=toast-nondismissable-interactive`,
    );
    await waitForTerminal(page);
    await clickTerminalText(page, "Show toast");
    await waitForTerminalText(page, "Upload in progress");
    await page.keyboard.press("Escape");
    await page.waitForTimeout(100);
    await waitForTerminalText(page, "Upload in progress");
    await clickTerminalText(page, "Click Show toast");
    await page.waitForTimeout(100);
    assert.ok(
      (await page.locator("#terminal_ratzilla_grid").textContent())?.includes(
        "Upload in progress",
      ),
      "outside click must not close a non-dismissible toast",
    );
  } finally {
    await page.close();
  }
});

test("colored variant triggers open matching semantic toasts", async () => {
  const page = await session.browser.newPage();
  try {
    await page.goto(
      `${session.baseUrl}/demos/interactive.html?demo=toast-variants-interactive`,
    );
    await waitForTerminal(page);
    for (const [trigger, title] of [
      ["Show default", "Notification"],
      ["Show success", "Success"],
      ["Show info", "Information"],
      ["Show warning", "Warning"],
      ["Show error", "Error"],
    ]) {
      await clickTerminalText(page, trigger);
      await waitForTerminalText(page, title);
    }
  } finally {
    await page.close();
  }
});

test("tracker ToastTrigger opens its own countdown demo", async () => {
  const page = await session.browser.newPage();
  try {
    await page.goto(
      `${session.baseUrl}/demos/interactive.html?demo=toast-tracker-interactive`,
    );
    await waitForTerminal(page);
    await clickTerminalText(page, "Show toast");
    await waitForTerminalText(page, "Build completed");
    const countTrackerCells = () =>
      page
        .locator("#terminal_ratzilla_grid pre")
        .evaluateAll((rows) =>
          Math.max(
            ...rows.map(
              (row) => [...(row.textContent ?? "")].filter((cell) => cell === "█").length,
            ),
          ),
        );
    await page.waitForTimeout(600);
    const fullerTracker = await countTrackerCells();
    await page.waitForTimeout(800);
    const emptierTracker = await countTrackerCells();
    assert.ok(emptierTracker < fullerTracker && emptierTracker > 0);
    await page.waitForFunction(
      () =>
        !(
          document
            .querySelector("#terminal_ratzilla_grid")
            ?.textContent?.includes("Build completed") ?? false
        ),
      undefined,
      { timeout: 6_000 },
    );
  } finally {
    await page.close();
  }
});
