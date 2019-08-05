# Rocks DB

## Building (linux)

```sh
apt install xorg-dev cmake libgtk-3-dev
cargo build --release
# binaries are:
# target/release/gui
# target/release/cli

# Dev
cargo run --bin cli
```

## Building (windows)

- Install Git for Windows [here](https://github.com/git-for-windows/git/releases/download/v2.22.0.windows.1/Git-2.22.0-32-bit.exe)
- Install Rust (32bit) [here](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe)
- Install VS Build tools [here](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019)
  - Tick VS2019, Cmake, Windows 10 SDK

*Tip: Always tick 'include in PATH'*

```bat
git clone https://git.gwillz.com/gwillz/rocks-db
cd rocks-db

set path=%path%;c:\program files (x86)\Microsoft Visual Studio\2019\BuildTools\Common7\IDE\CommonExtenstions\Microsoft\CMake\CMake\bin

cargo build --release

mkdir dist
copy description-database.txt dist
copy target/release/*.exe dist
copy target/release/build/ui-sys-*s/out/build/out/libui.dll dist
```

Package up the `dist` folder into a zip file.


## Authors
- Anish Phillips
- Gwilyn Saunders


## License
See `LICENSE.md`
