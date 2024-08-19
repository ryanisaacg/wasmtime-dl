# `wasmtime-dl`

This crate is an experiment that aims to combine dynamic libraries with wasmtime. The idea is to take a WASM module's imports and satisfy them with functions from an external dynamic library, which would allow access to libraries like SDL2 without recompiling the host program.

Ultimately it has some major flaws, namely:

1. Incredibly long compile times (there's a large amount of generated code)
2. Strict limits (it only supports up to 8 parameters for functions)
3. WASM ABI only (pointers and `i32/f32/i64/f64` are supported, but not any kind of record type)

I wouldn't recommend using it for any kind of production software, but maybe it can serve as inspiration for a better way to compile desktop apps to WASM.
