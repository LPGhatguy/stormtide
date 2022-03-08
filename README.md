# MTG3

Let's write yet another MtG rules engine!

This one has its rules implementation in Rust with a React.js frontend, glued together with WebAssembly.

## Requirements

- Rust 1.58.0 or newer
- Node.js 14.17 or newer
- wasm-pack 0.10.2

## Getting Started

To run tests:

```bash
cargo test
```

To run the web client:

```bash
npm install
npm start
```

To build a release build of the web client:

```bash
npm run build
```

## References

Implementation is currently based on comprehensive rules from August 7, 2020 — the release of Double Masters.

[Comprehensive Rules Summary](https://mtg.gamepedia.com/Comprehensive_Rules)
