# minifb multiplatform (webassembly too!) example

This project can be compiled to webassembly and desktop.

There is only one thing to keep in mind:
* desktop app expects the pixels to be in ARGB
* web app expects the pixels to be in ABGR

Check the game_step() function to see an example of
conditional compilation.

## How to run in a browser

* make sure you have wasm-pack installed: cargo install wasm-pack
* Run ./build_web.sh on linux to compile and start a demo server.

(the demo server is written in Python 3, so make is installed)

## How to run in a desktop

* cargo run --release
