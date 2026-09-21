import { createServer, type Server } from "node:http";
import { promises as fs } from "node:fs";
import path from "node:path";
import { chromium, type Browser, type Page } from "playwright";

const publicRoot = path.resolve("public");
const mimeTypes: Record<string, string> = {
  ".css": "text/css; charset=utf-8",
  ".html": "text/html; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".wasm": "application/wasm",
};

export interface InteractiveDemoSession {
  browser: Browser;
  baseUrl: string;
  close(): Promise<void>;
}

export async function startInteractiveDemoSession(): Promise<InteractiveDemoSession> {
  const server: Server = createServer(async (request, response) => {
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

  try {
    const browser = await chromium.launch();
    return {
      browser,
      baseUrl: `http://127.0.0.1:${address.port}`,
      async close() {
        await browser.close();
        if (server.listening) {
          await new Promise<void>((resolve, reject) => {
            server.close((error) => (error ? reject(error) : resolve()));
          });
        }
      },
    };
  } catch (error) {
    await new Promise<void>((resolve) => server.close(() => resolve()));
    throw error;
  }
}

export async function waitForTerminal(page: Page) {
  await page.waitForFunction(
    () => document.querySelectorAll("#terminal_ratzilla_grid pre").length > 0
  );
}

export async function clickTerminalText(page: Page, targetText: string) {
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

  assertPosition(position, targetText);
  await page
    .locator("#terminal_ratzilla_grid pre")
    .nth(position.row)
    .locator("span")
    .nth(position.column)
    .click();
}

export async function waitForTerminalText(page: Page, text: string) {
  await page.waitForFunction(
    (expected) =>
      document
        .querySelector("#terminal_ratzilla_grid")
        ?.textContent?.includes(expected) ?? false,
    text
  );
}

function assertPosition(
  position: { row: number; column: number } | null,
  targetText: string
): asserts position is { row: number; column: number } {
  if (!position) {
    throw new Error(`Could not find terminal text: ${targetText}`);
  }
}
