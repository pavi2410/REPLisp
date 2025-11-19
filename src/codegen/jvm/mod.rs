/// JVM bytecode generation backend
///
/// This module contains all the components needed to compile REPLisp to JVM bytecode.

pub mod classfile;
pub mod bytecode;
pub mod compiler;

pub use compiler::Compiler as JvmCompiler;
pub use classfile::ClassFile;
pub use bytecode::Opcode;
