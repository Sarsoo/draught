Draught
===============

![gof-ci](https://github.com/sarsoo/draught/actions/workflows/test.yml/badge.svg)

## [Try it Out!](https://draught.sarsoo.xyz)

WebAssembly-based checkers game with a minimax-based AI player.

Rust WASM module for game logic with a JS frontend for rendering and processing user input.

Read the docs [here](https://draught.sarsoo.xyz/doc/draught/). Read the blog post [here](https://sarsoo.xyz/posts/draught/).

![Screenshot](./draught_web/docs/screenshot.png)

## Building

1. Setup a Rust + wasm-pack environment and a Node environment
2. Build the Rust library into a WASM module 
    - `wasm-pack build`
3. Move to the Js workspace 
    - `cd www`
4. Install the Js dependencies
    - `npm install`
5. Build the Js frontend with Rust WASM module
    - `npm run build`