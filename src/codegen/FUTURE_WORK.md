# Codegen Future Work

## Option B: IR-Based Architecture

This document describes a potential future enhancement to the codegen architecture using an Intermediate Representation (IR) layer.

### Current Architecture (Option A - Trait-Based)

```
AST (Expr) → CodegenBackend Trait → JVM/WASM Backend
```

The current implementation uses a trait-based approach where each backend directly compiles AST expressions to target-specific bytecode/instructions. This works well for straightforward compilation but has some limitations for advanced features.

### Proposed: IR-Based Architecture

```
AST (Expr) → IR → Optimization Passes → JVM/WASM Backend
```

### IR Design

Create an intermediate representation that is:
- **Target-independent**: Abstract away JVM/WASM specifics
- **SSA-based**: Single Static Assignment for optimization
- **Type-explicit**: Include type information for each value

```rust
pub enum IR {
    // Values
    LoadConst(Constant),
    LoadVar(VarId),
    StoreVar(VarId, Box<IR>),

    // Arithmetic
    Add(Box<IR>, Box<IR>),
    Sub(Box<IR>, Box<IR>),
    Mul(Box<IR>, Box<IR>),
    Div(Box<IR>, Box<IR>),

    // Comparisons
    Eq(Box<IR>, Box<IR>),
    Lt(Box<IR>, Box<IR>),
    Gt(Box<IR>, Box<IR>),

    // Control flow
    If(Box<IR>, Block, Block),
    Loop(Block),
    Break,
    Continue,

    // Functions
    Call(FuncId, Vec<IR>),
    Return(Box<IR>),

    // Memory/Data structures
    AllocList(Vec<IR>),
    IndexList(Box<IR>, Box<IR>),

    // Blocks
    Block(Vec<IR>),
}

pub struct Constant {
    pub kind: ConstantKind,
    pub value: ConstantValue,
}

pub enum ConstantKind {
    Number,
    String,
    Boolean,
    Nil,
}

pub type VarId = usize;
pub type FuncId = usize;
pub type Block = Vec<IR>;
```

### Compilation Pipeline

1. **AST → IR Translation**
   ```rust
   pub fn ast_to_ir(exprs: &[Expr]) -> Result<Vec<IR>, CompileError>;
   ```
   - Convert high-level Lisp constructs to IR
   - Resolve all symbols to VarId/FuncId
   - Desugar special forms

2. **Optimization Passes** (Optional)
   ```rust
   pub trait OptimizationPass {
       fn optimize(&self, ir: Vec<IR>) -> Vec<IR>;
   }
   ```

   Possible optimizations:
   - **Constant folding**: `(+ 2 3)` → `5`
   - **Dead code elimination**: Remove unreachable code
   - **Tail call optimization**: Convert tail calls to loops
   - **Inlining**: Inline small functions
   - **Common subexpression elimination**

3. **IR → Target Backend**
   ```rust
   pub trait IRBackend {
       fn compile_ir(&mut self, ir: &[IR]) -> Result<Vec<u8>, CompileError>;
   }
   ```
   - Pure translation from IR to target bytecode
   - No semantic analysis needed at this stage
   - Backend becomes much simpler

### Benefits

1. **Optimization**: Common optimization passes work for all targets
2. **Simplicity**: Backends become pure translators
3. **Debugging**: Can inspect/dump IR for debugging
4. **Testing**: Easier to test IR generation vs bytecode
5. **New Backends**: Much easier to add new targets
6. **Advanced Features**: Easier to implement:
   - Tail call optimization
   - Closures
   - Continuations
   - Coroutines

### Drawbacks

1. **Complexity**: More layers of abstraction
2. **Development Time**: Significant upfront investment
3. **Performance**: Extra translation step (likely negligible)
4. **Maintenance**: More code to maintain

### When to Implement

Consider implementing IR-based architecture when:

1. **Adding 3+ backends**: Shared optimization becomes worthwhile
2. **Performance matters**: Need sophisticated optimizations
3. **Complex features**: Implementing closures, continuations, etc.
4. **Debugging tools**: Want to provide IR-level debugging
5. **Multiple frontends**: If adding other source languages

### Implementation Strategy

If pursuing this approach:

1. **Phase 1**: Design IR format
   - Define IR enum
   - Design type system
   - Plan memory model

2. **Phase 2**: AST → IR translation
   - Implement `ast_to_ir()`
   - Handle all expression types
   - Test IR generation

3. **Phase 3**: Basic optimization passes
   - Constant folding
   - Dead code elimination
   - Simple peephole optimizations

4. **Phase 4**: IR → JVM backend
   - Refactor JVM backend to consume IR
   - Test parity with direct compilation

5. **Phase 5**: IR → WASM backend
   - Refactor WASM backend to consume IR
   - Verify correctness

6. **Phase 6**: Advanced optimizations
   - Tail call optimization
   - Inlining
   - More sophisticated passes

### References

For inspiration, study these IR designs:

- **LLVM IR**: Industry standard, SSA-based
- **Cranelift IR**: WebAssembly-focused, modern design
- **MIR (Rust)**: Mid-level IR for high-level languages
- **CIL (.NET)**: Stack-based IR similar to our needs
- **JVM bytecode**: Can serve as IR itself

### Related Work

Some existing Rust projects with IR:

- `cranelift-codegen`: Code generator with IR
- `inkwell`: LLVM bindings for Rust
- `walrus`: WebAssembly manipulation library

---

**Status**: Future consideration
**Priority**: Low (current trait-based approach is sufficient)
**Estimated Effort**: 2-3 weeks of development
**Created**: 2025-11-19
