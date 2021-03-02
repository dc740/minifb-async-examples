#!/usr/bin/env bash
rm demo_server/web_app/req_anim_frame.wasm demo_server/web_app/req_anim_frame.js
cd req_anim_frame
wasm-pack build --target web --out-name req_anim_frame.wasm
cp pkg/*.js ../demo_server/web_app/

if command -v wasm-opt > /dev/null 2>&1; then
  wasm-opt pkg/req_anim_frame.wasm -o ../demo_server/web_app/req_anim_frame.wasm -O4 -all -ffm --enable-simd
else
    cp pkg/req_anim_frame.wasm demo_server/web_app/req_anim_frame.wasm
  echo WARNING: wasm-opt is not available. You can install it globally, or
  echo enable it in Cargo.toml if you are running on an x64 linux system.
  echo Cargo auto-installation for this particular command is broken
  echo because there are no precompiled packages.
fi

cd -
echo COMPILATION DONE. OPENING DEV SERVER:
python3 demo_server/server.py
