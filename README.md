# Rocks DB

This project has 3 parts.

1. The `librocks` library, written in Rust.
2. The CLI interface, written in Rust.
3. The GUI interface, written in Qt/C++ (using Rust FFI bindings).


## Building the library

### For Linux (CLI only)

```sh
cd rocks-db/cli

# Build
cargo build --release

# Dev run
cargo run
```

### For Windows (CLI and GUI)

Note, this guide is written for Windows XP. These instructions intentionally use an older/outdated version of Qt (5.3)

The Rust compiler doesn't work on XP, but the build can be done on a newer OS (7, 8, 10).

> It's probably also possible to cross-compile from Linux.
> Rust makes this easy with `cargo build --target i686-pc-windows-gnu`.
> I've never tried Qt cross-compiling, YMMV.

#### CLI

```bat
cd rocks-db\cli

" Build
cargo build --release

" Dev run
cargo run
```

#### GUI

- Install Git for Windows [here](https://github.com/git-for-windows/git/releases/download/v2.22.0.windows.1/Git-2.22.0-32-bit.exe)
- Install Rust (GCC 32bit) [here](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)
- Install Qt 5.3 [here](http://mirrors.ocf.berkeley.edu/qt/archive/qt/5.3/5.3.2/qt-opensource-windows-x86-mingw482_opengl-5.3.2.exe)


```bat
" build librocks
cd rocks-db\lib
cargo build --release

" build qt gui
cd ..\qt
mkdir build
cd build

qmake
mingw32-make

makensis
" Or right-click on install.nsi -> Compile NSIS script 

```


## Authors
- Anish Phillips
- Gwilyn Saunders


## License
See `LICENSE.md`
