# sqexpatch-sys

Incomplete Rust bindings for `sqexPatch.dll`.

## Building

Since `sqexPatch.dll` is 32-bit, make sure to use the i686 target. For example, when cross-compiling on Linux:

```shell
cargo build --target i686-pc-windows-gnu  
```

An example application is provided with the `basic` example to test if it's working.

## License

This project is licensed under the MIT license.
