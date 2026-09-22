import assert from "node:assert/strict";
import test from "node:test";

import { tocTitleToText } from "../domains/articles/toc.ts";

test("converts structured MDX TOC titles to readable text", () => {
  const title = {
    props: {
      children: [
        "Choose ",
        { props: { children: ["constraints", { value: " deliberately" }] } },
      ],
    },
  };

  assert.equal(tocTitleToText(title), "Choose constraints deliberately");
});

test("keeps plain TOC titles and ignores unsupported values", () => {
  assert.equal(tocTitleToText("Render app state"), "Render app state");
  assert.equal(tocTitleToText(["Input ", "and ", "shutdown"]), "Input and shutdown");
  assert.equal(tocTitleToText({ type: "emphasis" }), "");
});
