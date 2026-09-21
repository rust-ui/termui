const demoName = new URLSearchParams(window.location.search).get("demo");

if (!demoName || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(demoName)) {
  throw new Error("Interactive demo URL must include a valid demo name");
}

document.addEventListener(
  "keydown",
  (event) => {
    if (event.key === "Tab" || event.key === "Backspace") {
      event.preventDefault();
    }
  },
  true
);

const manifestUrl = new URL(
  "/demos/interactive-demo.json",
  window.location.origin
);
manifestUrl.searchParams.set(
  "v",
  new URL(import.meta.url).searchParams.get("v") ?? ""
);

const response = await fetch(manifestUrl);
if (!response.ok) {
  throw new Error(`Could not load interactive demo manifest: ${response.status}`);
}

const manifest = await response.json();
const demo = manifest[demoName];
if (!demo) {
  throw new Error(`Unknown interactive demo: ${demoName}`);
}

document.title = demo.title;

const jsUrl = `/demos/${demoName}/${demoName}-${demo.hash}.js`;
const wasmUrl = `/demos/${demoName}/${demoName}-${demo.hash}_bg.wasm`;

const jsPreload = document.createElement("link");
jsPreload.rel = "modulepreload";
jsPreload.href = jsUrl;
jsPreload.integrity = demo.jsIntegrity;
jsPreload.crossOrigin = "anonymous";
document.head.append(jsPreload);

const wasmPreload = document.createElement("link");
wasmPreload.rel = "preload";
wasmPreload.as = "fetch";
wasmPreload.href = wasmUrl;
wasmPreload.integrity = demo.wasmIntegrity;
wasmPreload.crossOrigin = "anonymous";
wasmPreload.type = "application/wasm";
document.head.append(wasmPreload);

const { default: init, ...bindings } = await import(jsUrl);
const wasm = await init({ module_or_path: wasmUrl });

window.wasmBindings = bindings;
dispatchEvent(new CustomEvent("TrunkApplicationStarted", { detail: { wasm } }));
