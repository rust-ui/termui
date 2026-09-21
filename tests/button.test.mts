import assert from "node:assert/strict";
import { createServer, type Server } from "node:http";
import { promises as fs } from "node:fs";
import path from "node:path";
import { after, before, test } from "node:test";
import { chromium, type Browser, type Page } from "playwright";

const publicRoot = path.resolve("public");
const mimeTypes: Record<string, string> = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".wasm": "application/wasm",
};

let browser: Browser;
let server: Server;
let baseUrl: string;

before(async () => {
  server = createServer(async (request, response) => {
    try {
      const pathname = decodeURIComponent(
        new URL(request.url ?? "/", "http://127.0.0.1").pathname
      );
      const filePath = path.resolve(publicRoot, `.${pathname}`);
      if (!filePath.startsWith(`${publicRoot}${path.sep}`)) {
        response.writeHead(403).end();
        return;
      }

      const contents = await fs.readFile(filePath);
      response
        .writeHead(200, {
          "Content-Type":
            mimeTypes[path.extname(filePath)] ?? "application/octet-stream",
        })
        .end(contents);
    } catch {
      response.writeHead(404).end();
    }
  });

  await new Promise<void>((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });

  const address = server.address();
  if (!address || typeof address === "string") {
    throw new Error("Static demo server did not bind to a TCP port");
  }

  baseUrl = `http://127.0.0.1:${address.port}`;
  browser = await chromium.launch();
});

after(async () => {
  await browser?.close();
  if (server?.listening) {
    await new Promise<void>((resolve, reject) => {
      server.close((error) => (error ? reject(error) : resolve()));
    });
  }
});

async function clickTerminalText(page: Page, targetText: string) {
  const position = await page
    .locator("#terminal_ratzilla_grid pre")
    .evaluateAll((rows, text) => {
      for (let row = 0; row < rows.length; row += 1) {
        const column = rows[row].textContent?.indexOf(text) ?? -1;
        if (column >= 0) {
          return { row, column };
        }
      }
      return null;
    }, targetText);

  assert.ok(position, `Could not find terminal text: ${targetText}`);
  await page
    .locator("#terminal_ratzilla_grid pre")
    .nth(position.row)
    .locator("span")
    .nth(position.column)
    .click();
}

test("Button responds to mouse and keyboard actions", async () => {
  const page = await browser.newPage({ viewport: { width: 1200, height: 762 } });
  const pageErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error.message));

  try {
    await page.goto(`${baseUrl}/demos/button-interactive/index.html`);
    await page.waitForFunction(
      () => document.querySelectorAll("#terminal_ratzilla_grid pre").length > 0
    );
    assert.equal(await page.title(), "Interactive Ratatui Button");

    const terminal = page.locator("#terminal_ratzilla_grid");
    await clickTerminalText(page, "Increment +1");
    assert.match(await terminal.innerText(), /Count:\s*1/);

    await clickTerminalText(page, "Reset");
    assert.match(await terminal.innerText(), /Count:\s*0/);

    await page.keyboard.press("Space");
    assert.match(await terminal.innerText(), /Count:\s*1/);

    await page.keyboard.press("r");
    assert.match(await terminal.innerText(), /Count:\s*0/);
    assert.deepEqual(pageErrors, []);
  } finally {
    await page.close();
  }
});
