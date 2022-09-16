# Lurk web REPL example

This example REPL is largely based on the following crates:

https://github.com/rustwasm/rust-webpack-template
https://github.com/segeljakt/xterm-js-rs
https://github.com/jlogelin/lurk-web-component

Useful docs:
https://rustwasm.github.io/docs/wasm-pack/tutorials/hybrid-applications-with-webpack/index.html
https://rustwasm.github.io/docs/wasm-bindgen/

### Install prerequisites

- wasm32 target:
```
rustup target add wasm32-unknown-unknown
```
- wasm-pack:
```
cargo install wasm-pack
```
- wasm-ld linker:
```
# Ubuntu
sudo apt install llvm lld-14
# Mac
brew install llvm
# Add llvm to homebrew's PATH variable, e.g. one of the following
export PATH=/usr/local/opt/llvm/bin:$PATH
export PATH=/opt/homebrew/Cellar/llvm/13.0.1_1/bin:$PATH
# Verify installation
llc --version
```
- [clang](https://clang.llvm.org/get_started.html)
- [yarn](https://classic.yarnpkg.com/lang/en/docs/install/#mac-stable) or [npm](https://nodejs.org/en/download/package-manager/)
- [webpack](https://webpack.js.org/guides/installation/)

## How to install

```sh
npm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## How to run unit tests

```sh
# Runs tests in Firefox
npm test -- --firefox

# Runs tests in Chrome
npm test -- --chrome

# Runs tests in Safari
npm test -- --safari
```

## What does each file do?

* `Cargo.toml` contains the standard Rust metadata. You put your Rust dependencies in here. You must change this file with your details (name, description, version, authors, categories)

* `package.json` contains the standard npm metadata. You put your JavaScript dependencies in here. You must change this file with your details (author, name, version)

* `webpack.config.js` contains the Webpack configuration. You shouldn't need to change this, unless you have very special needs.

* The `js` folder contains your JavaScript code (`index.js` is used to hook everything into Webpack, you don't need to change it).

* The `src` folder contains your Rust code.

* The `static` folder contains any files that you want copied as-is into the final build. It contains an `index.html` file which loads the `index.js` file.

* The `tests` folder contains your Rust unit tests.

