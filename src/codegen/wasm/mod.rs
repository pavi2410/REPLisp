/// WebAssembly code generation backend
///
/// This module contains all the components needed to compile REPLisp to WebAssembly.

pub mod module;
pub mod encoder;
pub mod compiler;

pub use compiler::Compiler as WasmCompiler;
pub use module::WasmModule;
