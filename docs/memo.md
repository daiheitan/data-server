# Preparation for Windows

- Install [Cmder](http://cmder.net/): Download source, put it into `Program Files` folder, run. Cmder will load `$PATH` automatically.
- Install [Rust](http://rust-lang.org/): It ships `Cargo` by default.

> Install `Cargo-edit`:

> `cargo-edit` is an extension for cargo to install project dependencies like `npm`.
However, it is currently not available for windows due to it's heavy rely on `libcurl`

- Install [VSCode](http://github.com/microsoft/vscode).
    - RustyCode
    - Eslint
- Install [Racer](https://github.com/phildawes/racer) for Rust code complemention.
    - Download Rust src, `set RUST_SRC_PATH=<your path>`
    - Run `racer` to check that everything is OK

- Install [MinGW-Builds](http://mingw-w64.org/doku.php/download/mingw-builds)
> `MinGW` is a Windows port for gcc which only supports win32. `MinGW-W64` is a project supporting
win64 with only the source code. `MinGW-Builds` is a ready-to-use compiler for it.

    > `Win-builds` is an GUI program shipped with `yypkg` as the package manager, it could
install many windows-version libs. However it is relatively new and not stable. We will not use
it here.

    - Add Path: `E:\Program Files\mingw-w64\x86_64-5.3.0-posix-seh-rt_v4-rev0\mingw64\bin`
    - Download the openssl libs for windows
    - Extract them to `<minGw dir>/x86_64-w64-mingw32/include` folder

- Install [openssl](http://slproweb.com/products/Win32OpenSSL.html)
    - If you DO NOT choose `install openssl bin to windows folder`, `cargo run` will fail with DLL
missing error
    - Copy `libeay32.dll`, `libssl32.dll`, `ssleay32.dll` to `<minGw dir>/lib/gcc`
        - or, create a new env variable `OPENSSL_LIB_DIR`, point to openssl installation folder and
restart bash
    - `cargo clean`

- `cargo build` will now success
