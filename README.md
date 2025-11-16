# jvm-rs

A Java Virtual Machine (JVM) implementation written in Rust, capable of parsing and executing Java class files and bytecode.

## Overview

jvm-rs is an educational/experimental JVM implementation that demonstrates the core components and execution model of the Java Virtual Machine. The project uses Rust's type safety and modern tooling to build a simplified but functional JVM interpreter.

## Features

### Currently Implemented

- **Class File Parsing**: Full support for reading and deserializing binary Java class files (`.class`)
- **Constant Pool Management**: Handles 20+ constant pool entry types (UTF8, Integer, Float, Long, Double, Class, String, MethodRef, FieldRef, etc.)
- **Dynamic Class Loading**: On-demand class loading with superclass and interface resolution
- **Class Initialization**: Automatic `<clinit>` method execution during class initialization
- **Bytecode Execution**: Interpreter for JVM bytecode instructions including:
  - Constant loading (`ldc`, `ldc2_w`)
  - Load/store operations (`fload`, `dload`, `fstore`, `dstore`, etc.)
  - Type conversions (`f2d`)
  - Arithmetic operations (`dadd`)
  - Field access (`getstatic`)
  - Method invocation (`invokevirtual`, `invokestatic`)
  - Control flow (`return_void`)
- **Module System**: Support for loading classes from 7z binary image archives
- **Frame-based Execution**: Proper execution context with program counter, operand stack, and local variables

### In Development

Many bytecode instructions and JVM features are planned but not yet implemented:
- Most bytecode operations (array operations, object creation, exception handling, etc.)
- Complete object model and field access
- Method resolution and invoke dynamic
- Exception handling and throw/catch
- Type checking and validation
- Garbage collection
- JNI/Native interface support

## Architecture

### Core Components

- **`Vm`** (`vm.rs`): Main virtual machine controller managing threads and class loader
- **`VmThread`** (`thread.rs`): Thread of execution managing the frame stack and method invocation
- **`Frame`** (`lib.rs`): Execution context for a method with PC, operand stack, and local variables
- **`ClassLoader`** (`class_loader.rs`): Handles dynamic class loading, linking, and initialization
- **`RuntimeClass`** (`class.rs`): Runtime representation of a loaded class
- **`ClassFile`** (`class_file/`): Binary parser for Java class files using the `deku` library
- **`ConstantPool`** (`class_file/constant_pool.rs`): Constant pool resolution and management
- **`Object`** (`object.rs`): Runtime representation of Java objects

### Execution Flow

1. **Loading**: `ClassFile::from_bytes()` parses binary class file data
2. **Resolution**: `ClassLoader` converts `ClassFile` to `RuntimeClass`, resolving dependencies
3. **Execution**: `VmThread` invokes the main method, creating a `Frame`
4. **Interpretation**: `Frame` iterates through bytecode operations, executing each instruction
5. **Stack Operations**: Instructions manipulate the operand stack and local variables

## Usage

```rust
use jvm_rs::{Vm, VmError};

fn main() -> Result<(), VmError> {
    // Initialize the VM
    let vm = Vm::new();

    // Execute a main method
    vm.run_main("path/to/ClassFile.class")?;

    Ok(())
}
```

## Dependencies

- **`deku`**: Binary parsing and serialization for class files
- **`log`** / **`env_logger`**: Logging infrastructure
- **`itertools`**: Iterator utilities
- **`sevenz-rust2`**: 7z archive reading for module system support

## Project Structure

```
jvm-rs/
├── src/
│   ├── lib.rs                 # Frame and Value definitions
│   ├── main.rs                # Entry point
│   ├── vm.rs                  # Virtual Machine implementation
│   ├── thread.rs              # Thread execution management
│   ├── class.rs               # RuntimeClass definition
│   ├── class_loader.rs        # ClassLoader implementation
│   ├── class_file/
│   │   ├── class_file.rs      # ClassFile parser (magic 0xCAFEBABE)
│   │   └── constant_pool.rs   # Constant pool types
│   ├── attributes.rs          # Bytecode operations and attributes
│   ├── macros.rs              # Helper macros
│   ├── object.rs              # Object representation
│   ├── bimage.rs              # Binary image (7z) reader
│   └── rng.rs                 # RNG utilities
├── data/                      # Test class files
└── lib/                       # Module library (7z archives)
```

## Building

```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## Current Status

This project is in early development (v0.1.0). The core infrastructure for class loading and basic bytecode execution is in place, but many JVM features remain unimplemented. Contributions and experimentation are welcome!

## License

[Add your license here]

## References

- [JVM Specification](https://docs.oracle.com/javase/specs/jvms/se21/html/index.html)
- [Java Class File Format](https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html)