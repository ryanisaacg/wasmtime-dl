use libloading::{Library, Symbol};
use thiserror::Error;
use wasmparser::{BinaryReaderError, Parser, Payload};
use wasmtime::{AsContextMut, Caller, Engine, Extern, FuncType, Linker, ValRaw, ValType};

mod bind;

#[derive(Debug, Error)]
pub enum Error {
    #[error("wasm binary error: {0}")]
    BinaryWasmError(#[from] BinaryReaderError),
}

pub fn discover_imports(binary_module: &[u8]) -> Result<(), Error> {
    for payload in Parser::new(0).parse_all(binary_module) {
        match payload? {
            Payload::ImportSection(s) => {
                for import in s {
                    let import = import?;
                }
            }
            Payload::TypeSection(s) => {
                for ty in s {
                    let ty = ty?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ParamType {
    I32,
    F32,
    I64,
    F64,
    Pointer,
}

impl ParamType {
    fn val(self) -> ValType {
        match self {
            ParamType::I32 | ParamType::Pointer => ValType::I32,
            ParamType::F32 => ValType::F32,
            ParamType::I64 => ValType::I64,
            ParamType::F64 => ValType::F64,
        }
    }
}

pub unsafe fn bind(
    linker: &mut Linker<()>,
    module: &str,
    name: &str,
    lib: &'static Library,
    lib_name: &[u8],
    params: &[ParamType],
    returns: Option<ParamType>,
) {
    let ty = FuncType::new(linker.engine(), params.iter().map(|p| p.val()), []);
    bind::bind(linker, module, name, lib, lib_name, ty, params, returns);
}
