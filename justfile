build:
  #!/bin/bash
  rm -rf dist/
  mkdir -p dist/

  # Compile to WebAssembly
  # Note: If this fails, you are missing the wasm32 target.
  cargo build --target wasm32-unknown-unknown --release

  # Check if wasm-bindgen-cli is installed
  if ! command -v wasm-bindgen &> /dev/null
  then
      echo "wasm-bindgen-cli not found. Installing..."
      cargo install wasm-bindgen-cli
  fi

  # Generate the static files into a folder named 'dist'
  wasm-bindgen --target web \
               --out-dir ./dist \
               target/wasm32-unknown-unknown/release/metronomo.wasm

  # 4. Copy your index.html into the dist folder
  cp index.html dist/
  cp index.css dist/


serve:
  cd dist && python3 -m http.server 9996
