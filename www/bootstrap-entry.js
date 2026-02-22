// Keep the async wasm import in a dedicated source entry file.
// The built bundle is written to bootstrap.js, which should never be used as source.
import("./index.js").catch((e) =>
  console.error("Error importing `index.js`:", e)
);
