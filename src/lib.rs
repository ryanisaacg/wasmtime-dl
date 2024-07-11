use std::ffi::OsStr;

use libloader::libloading::{Library, Symbol};
use wasmtime::Linker;

#[derive(Copy, Clone)]
pub enum ValType {
    I32,
    F32,
    I64,
    F64,
}

pub struct WasmFuncImport<'a> {
    pub module: &'a str,
    pub name: &'a str,
    pub params: &'a [ValType],
    pub returns: Option<ValType>,
}

#[allow(clippy::type_complexity)]
pub unsafe fn link(
    linker: &mut Linker<()>,
    dynamic_lib: impl AsRef<OsStr>,
    imports: &[WasmFuncImport<'_>],
) {
    // TODO: Use an Arc instead of leaking the DLL?
    let lib = unsafe { Library::new(dynamic_lib).unwrap() };
    let lib = Box::leak(Box::new(lib)) as &'static Library;

    for import in imports {
        match (import.params, import.returns) {
            ([], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn() -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move || func())
                    .unwrap();
            }
            ([], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn() -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move || func())
                    .unwrap();
            }
            ([], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn() -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move || func())
                    .unwrap();
            }
            ([], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn() -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move || func())
                    .unwrap();
            }
            ([], None) => {
                let func: Symbol<unsafe extern "C" fn()> = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move || func())
                    .unwrap();
            }
            ([ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0| func(p0))
                    .unwrap();
            }
            ([ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1| func(p0, p1))
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2| {
                        func(p0, p1, p2)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F32, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f32, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I64, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i64, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::F64, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, f64, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I32, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i32, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F32, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f32, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::I64, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, i64, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F32, ValType::F64, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f32, f64, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I32, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i32, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F32, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f32, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::I64, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, i64, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I64, ValType::F64, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(i64, f64, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I32, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i32, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F32, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f32, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::I64, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, i64, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F32, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f32, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::I64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, i64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::I32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::I32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, i32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::I32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, i32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::I32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, i32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::F32], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, f32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::F32], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, f32) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::F32], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, f32) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::F32], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, f32) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::F32], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, f32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::I64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, i64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::I64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, i64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::I64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, i64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::I64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, i64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::I64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, i64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::F64], Some(ValType::I32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, f64) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::F64], Some(ValType::F32)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, f64) -> f32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::F64], Some(ValType::I64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, f64) -> i64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::F64], Some(ValType::F64)) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, f64) -> f64> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::F64, ValType::F64, ValType::F64, ValType::F64], None) => {
                let func: Symbol<unsafe extern "C" fn(f64, f64, f64, f64)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3| {
                        func(p0, p1, p2, p3)
                    })
                    .unwrap();
            }
            ([ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32], None) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3, p4| {
                        func(p0, p1, p2, p3, p4)
                    })
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                Some(ValType::I32),
            ) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3, p4| {
                        func(p0, p1, p2, p3, p4)
                    })
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                None,
            ) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3, p4, p5| {
                        func(p0, p1, p2, p3, p4, p5)
                    })
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                Some(ValType::I32),
            ) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(import.module, import.name, move |p0, p1, p2, p3, p4, p5| {
                        func(p0, p1, p2, p3, p4, p5)
                    })
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                None,
            ) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6| func(p0, p1, p2, p3, p4, p5, p6),
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                Some(ValType::I32),
            ) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32) -> i32> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6| func(p0, p1, p2, p3, p4, p5, p6),
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                None,
            ) => {
                let func: Symbol<unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32)> =
                    lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7| func(p0, p1, p2, p3, p4, p5, p6, p7),
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                Some(ValType::I32),
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32) -> i32,
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7| func(p0, p1, p2, p3, p4, p5, p6, p7),
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                None,
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32, i32),
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8| {
                            func(p0, p1, p2, p3, p4, p5, p6, p7, p8)
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                Some(ValType::I32),
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32, i32) -> i32,
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8| {
                            func(p0, p1, p2, p3, p4, p5, p6, p7, p8)
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                None,
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32),
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8, p9| {
                            func(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                Some(ValType::I32),
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) -> i32,
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8, p9| {
                            func(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                None,
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32),
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10| {
                            func(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10)
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                Some(ValType::I32),
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ) -> i32,
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10| {
                            func(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10)
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                None,
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ),
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11| {
                            func(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11)
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                Some(ValType::I32),
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ) -> i32,
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11| {
                            func(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11)
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                None,
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ),
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12| {
                            func(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12)
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                Some(ValType::I32),
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ) -> i32,
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12| {
                            func(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12)
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                None,
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ),
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13| {
                            func(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13)
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                Some(ValType::I32),
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ) -> i32,
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13| {
                            func(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13)
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                None,
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ),
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14| {
                            func(
                                p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14,
                            )
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                Some(ValType::I32),
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ) -> i32,
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14| {
                            func(
                                p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14,
                            )
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                None,
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ),
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0,
                              p1,
                              p2,
                              p3,
                              p4,
                              p5,
                              p6,
                              p7,
                              p8,
                              p9,
                              p10,
                              p11,
                              p12,
                              p13,
                              p14,
                              p15| {
                            func(
                                p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14,
                                p15,
                            )
                        },
                    )
                    .unwrap();
            }
            (
                [ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32, ValType::I32],
                Some(ValType::I32),
            ) => {
                let func: Symbol<
                    unsafe extern "C" fn(
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                        i32,
                    ) -> i32,
                > = lib.get(import.name.as_bytes()).unwrap();
                linker
                    .func_wrap(
                        import.module,
                        import.name,
                        move |p0,
                              p1,
                              p2,
                              p3,
                              p4,
                              p5,
                              p6,
                              p7,
                              p8,
                              p9,
                              p10,
                              p11,
                              p12,
                              p13,
                              p14,
                              p15| {
                            func(
                                p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14,
                                p15,
                            )
                        },
                    )
                    .unwrap();
            }
            ([_p0, _p1, _p2, _p3, ..], _) => todo!(),
        }
    }
}
