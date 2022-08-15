# Tower Defense RUST

Works both in-browser (thanks to WASM) and as a desktop version (OpenGL) on Mac, Windows and Linux.

## Web version

Install wasmpack if not installed: https://rustwasm.github.io/wasm-pack/installer/.

```
cd rust
cargo build --workspace --exclude desktop
wasm-pack build wasm
cd wasm
yarn
yarn start
```

## Desktop

### Mac

Install required dependencies:

```
brew install sdl2 sdl2_image sdl2_ttf sdl2_gfx
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

Build and start project:

```
cd rust
cargo build
./target/debug/desktop
```

### Linux & Windows

[Desktop version can be also tested on Windows with Windows-Subsystem-For-Linux.](https://docs.microsoft.com/en-us/windows/wsl/tutorials/gui-apps)

#### Start

```
cd rust
cargo build
./target/debug/desktop
```

<img src="./demo-screenshot-desktop.png"/>
<img src="./demo-screenshot-web.png"/>
