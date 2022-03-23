# Stormtide

Rules engine for a card game using Rust, React.js, and WebAssembly.

## Requirements

- Rust 1.58.0 or newer
- Node.js 14.17 or newer
- wasm-pack 0.10.2

## Getting Started

This project is split between two crates: `mtg_engine` where the core rules are implemented, and `mtg`, which is a React-based frontend consuming `mtg_engine` via WebAssembly.

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

## License

This project is available under the terms of the Mozilla Public License, Version 2.0. Details are available in [LICENSE.txt](LICENSE.txt) or at <https://www.mozilla.org/en-US/MPL/>.
