# Tower Defense RUST

Works both in-browser (thanks to WASM) on Mac, Windows, Linux and as a desktop version (OpenGL) on Windows and Linux.

## Mac

Install wasmpack if not installed: https://rustwasm.github.io/wasm-pack/installer/.

### Web version

```
cd rust
cargo build --workspace --exclude desktop
wasm-pack build wasm
cd wasm
yarn
yarn start
```

## Windows and Linux

### Desktop

Desktop version can be tested on Windows with Windows-Subsystem-For-Linux and XLaunch (Disable native OpenGL).

#### Start

```
cd rust
cargo build
./target/debug/desktop
```

<img src="./demo-screenshot-desktop.png"/>

### Web version

#### Start

```
wasm-pack build rust/wasm
cd rust/wasm
yarn start
```

<img src="./demo-screenshot-web.png"/>
