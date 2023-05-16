<div align="center">

  <h1><code>Parse wasm file to wat</code></h1>

  <p>
    <a href="https://github.com/axetroy/wasm-printer/actions/workflows/rust.yml"><img src="https://github.com/axetroy/wasm-printer/actions/workflows/rust.yml/badge.svg" alt="Build Status" /></a>
  </p>

<sub>Built with 🦀🕸

</div>

## Usage

```js
import("@axetroy/wasm-printer").then((parse_wat) => {
  fetch("/path/to/demo.wasm")
    .then((resp) => resp.arrayBuffer())
    .then((bytes) => {
      const str = parse_wat(new Uint8Array(bytes));

      console.log(str);
    });
});
```

## 🚴 Installation

1. if you are using Bundler(Which build-in support to importing wasm module, eg. Webpack^5/Vite)

```bash
npm install @axetroy/wasm-printer
```

2. If you are using in Native browser

### 🛠️ Build from source

Make sure you have install [rust^1.69](https://www.rust-lang.org/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
make
```

## License

[Anti-996](License)
