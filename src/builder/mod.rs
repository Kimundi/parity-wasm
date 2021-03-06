//! Various builders to generate/alter wasm components

mod invoke;
mod module;
mod code;
mod misc;
mod import;
mod memory;
mod table;
mod export;
mod global;
mod data;

pub use self::module::{module, from_module, ModuleBuilder};
pub use self::code::{signatures, signature, function, FunctionBuilder, SignatureBuilder};
pub use self::import::import;
pub use self::export::{export, ExportBuilder, ExportInternalBuilder};
pub use self::global::global;
pub use self::invoke::{Identity, Invoke};
