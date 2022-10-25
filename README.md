<div align="center">

  <h1>PASETO rust WASM (<code>pasetors-wasm</code>)</h1>
Enable the use of PASETO in the JavaScript browser with the help of <a href="https://github.com/rustwasm/wasm-pack">WebAssembly (WASM)</a>.</strong>

  <!-- <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>
 -->
  <!-- <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3> -->

  <sub>Built with 🦀🕸 by <a href="https://github.com/achmadk">Achmad Kurnianto</a></sub>
</div>

## :mountain: Background

PASETO is everything you love about JOSE (JWT, JWE, JWS) without any of the many design deficits that plague the JOSE standards. 
According to the [paseto.io](https://paseto.io) website, there is no PASETO library supported for JavaScript browsers yet. Library paseto from Filip Skokan only supports nodejs, and paseto.js from Samuel Judson from now uses deprecated versions 1 and 2 implementations of PASETO.

Luckily, there are PASETO libraries implemented with the rust programming language, which can be compiled into WebAssembly. One of them is [`pasetors`](https://github.com/brycx/pasetors) from [Johannes](https://github.com/brycx), which has WASM support [but is not fully tested](https://github.com/brycx/pasetors/issues/75#issuecomment-1281376534). 

The creator of `pasetors` also lacks experience using JavaScript programming language and publishing it to the npm registry. I hope this crate will help him and improve WASM support of `pasetors`.

## 🚴 Usages

### Installation

This crate will produce 2 separate npm libraries each called `@achmadk/pasetors-wasm` (support JavaScript browser) and `@achmadk/pasetors-wasm-node` (support nodejs).

```sh
yarn add @achmadk/pasetors-wasm # for browser, or
yarn add @achmadk/pasetors-wasm-node # for nodejs
```

### Methods (WIP)

- `v4.localToken.encrypt`
- `v4.localToken.decrypt`
- `v4.publicToken.sign`
- `v4.publicToken.verify`

## :pen: Contributing

### 🛠️ Build with `wasm-pack build`

```sh
wasm-pack build --release -t web -s achmadk --out-dir pkg/pasetors-wasm --out-name pasetors-wasm # for browser
wasm-pack build --release -t nodejs -s achmadk --out-dir pkg/pasetors-wasm-node --out-name pasetors-wasm-node # for nodejs
```

### 🔬 Test in either Browsers or nodejs

```
wasm-pack test --headless
wasm-pack test --chrome --firefox
wasm-pack test --node
```

<!-- ### 🎁 Publish to NPM

```
wasm-pack publish
``` -->

For more information about contributing, please read [this file](CONTRIBUTING.md)

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook) for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized for small code size.
* [`pasetors`](https://github.com/brycx/pasetors) for implementing PASETO with rust programming language.
