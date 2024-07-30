use libloading::Library;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_dl::bind;

fn main() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let mut linker = Linker::new(&engine);
    let library =
        Box::leak(Box::new(unsafe { Library::new("printtest").unwrap() })) as &'static Library;
    unsafe { bind(&engine, &mut linker, "bindings", "print", library) };
    let binary = std::fs::read("out.wasm").unwrap();
    let module = Module::from_binary(&engine, binary.as_slice()).unwrap();
    linker.instantiate(&mut store, &module).unwrap();
}
