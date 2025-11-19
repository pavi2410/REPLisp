/// Backend trait for code generation
///
/// This trait defines the interface that all code generation backends must implement.
/// It provides a common abstraction for compiling REPLisp AST to different target
/// platforms (JVM, WASM, etc.).

use crate::parser::Expr;

/// Common interface for all code generation backends
pub trait CodegenBackend {
    /// Create a new backend instance with the given module/class name
    ///
    /// # Arguments
    /// * `name` - The name of the output module/class
    ///
    /// # Returns
    /// A new instance of the backend
    fn new(name: String) -> Self where Self: Sized;

    /// Compile a list of expressions into the target format
    ///
    /// This is the main compilation entry point. It should:
    /// 1. Perform any necessary preprocessing (e.g., collecting function definitions)
    /// 2. Compile all expressions in order
    /// 3. Generate any necessary wrapper code (e.g., main function)
    /// 4. Finalize the output
    ///
    /// # Arguments
    /// * `exprs` - The list of AST expressions to compile
    ///
    /// # Returns
    /// The compiled bytecode/binary as a Vec<u8>, or an error message
    fn compile(&mut self, exprs: &[Expr]) -> Result<Vec<u8>, String>;

    /// Get the file extension for the output file
    ///
    /// # Returns
    /// The file extension without the dot (e.g., "class", "wasm")
    fn file_extension(&self) -> &str;

    /// Get a human-readable name for this backend
    ///
    /// # Returns
    /// The backend name (e.g., "JVM", "WASM")
    fn backend_name(&self) -> &str;
}

/// Compile the given expressions using the specified backend
///
/// This is a convenience function that creates a backend instance and compiles
/// the expressions in one step.
///
/// # Arguments
/// * `name` - The name of the output module/class
/// * `exprs` - The list of AST expressions to compile
///
/// # Returns
/// The compiled bytecode/binary as a Vec<u8>, or an error message
pub fn compile_with_backend<B: CodegenBackend>(name: String, exprs: &[Expr]) -> Result<Vec<u8>, String> {
    let mut backend = B::new(name);
    backend.compile(exprs)
}
