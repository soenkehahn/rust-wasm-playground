serve:
  miniserve ./static --index index.html

bundle:
  wasm-pack build --target web --out-name wasm --out-dir ./static
