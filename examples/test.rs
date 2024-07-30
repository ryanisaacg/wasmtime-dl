use libloading::Library;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_dl::{bind, ParamType};

fn main() {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let mut linker = Linker::new(&engine);
    let library =
        Box::leak(Box::new(unsafe { Library::new("testinglib").unwrap() })) as &'static Library;
    unsafe {
        bind(
            &mut linker,
            "bindings",
            "print",
            library,
            "print".as_bytes(),
            &[ParamType::F32],
            None,
        )
    };
    unsafe {
        bind(
            &mut linker,
            "bindings",
            "incr",
            library,
            "incr".as_bytes(),
            &[ParamType::Pointer],
            None,
        )
    };
    let binary = std::fs::read("out.wasm").unwrap();
    let module = Module::from_binary(&engine, binary.as_slice()).unwrap();
    linker.instantiate(&mut store, &module).unwrap();
}
