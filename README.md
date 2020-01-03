# Rocks DB

This project has 3 parts.

1. The `librocks` library, written in Rust.
2. The CLI interface, written in Rust.
3. The GUI interface, written in Qt/C++ (using Rust FFI bindings).


## Building the library

### For Linux

```sh
# Build
cargo build --release

# Dev run
cargo run --bin cli
```

### For Windows

Note, this guide is written for Windows XP. These instructions intentionally use an older/outdated version of Qt (5.2)

- Install Git for Windows [here](https://github.com/git-for-windows/git/releases/download/v2.22.0.windows.1/Git-2.22.0-32-bit.exe)
- Install Rust (32bit) [here](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe)
- Install VS 2010 Express [here](https://my.visualstudio.com/Downloads?q=visual%20studio%202010&wt.mc_id=o~msft~vscom~older-downloads)
- Install Windows SDK 7.1 [here](https://www.microsoft.com/en-us/download/details.aspx?id=8279)
  - Select only "Debugging Tools"
  - Tip: Uninstall the VS 2010 redistributable package if it failed to install
- Install Qt 5.2 [here](http://mirrors.ocf.berkeley.edu/qt/archive/qt/5.2/5.2.1/)
  - Choose MSVC 2010


```bat
" build
cargo build --release --target i586-pc-window-msvc

mkdir dist
copy description-database.txt dist
copy target/release/*.exe dist
copy target/release/build/ui-sys-*s/out/build/out/libui.dll dist

cargo rustc --release --target i686-pc-windows-msvc -- -Clink-args="/SUBSYSTEM:CONSOLE,5.01" -Ctarget-feature=+crt-static

```

Package up the `dist` folder into a zip file.


## Authors
- Anish Phillips
- Gwilyn Saunders


## License
See `LICENSE.md`
