/// Code generation modules
///
/// This module provides code generation capabilities for multiple target platforms.
/// Currently supported backends:
/// - JVM: Compile to Java bytecode (.class files)
/// - WASM: Compile to WebAssembly (.wasm files)

pub mod backend;
pub mod common;
pub mod jvm;
pub mod wasm;

// Re-export commonly used types
pub use backend::{CodegenBackend, compile_with_backend};
pub use jvm::JvmCompiler;
pub use wasm::WasmCompiler;

// For backwards compatibility, re-export the JVM compiler as "Compiler"
pub use jvm::JvmCompiler as Compiler;
