# roast-vm

A Java Virtual Machine (JVM) implementation written in Rust, capable of parsing and executing Java class files and bytecode.

## Overview

roast-vm is an educational/experimental JVM implementation that demonstrates the core components and execution model of the Java Virtual Machine. The project uses Rust's type safety and modern tooling to build a simplified but functional JVM interpreter.

## Features

### Currently Implemented

- **Class File Parsing**: Full support for reading and deserializing binary Java class files (`.class`) with magic number `0xCAFEBABE`
- **Constant Pool Management**: Handles 20+ constant pool entry types (UTF8, Integer, Float, Long, Double, Class, String, MethodRef, FieldRef, InterfaceMethodRef, NameAndType, MethodHandle, MethodType, InvokeDynamic, etc.)
- **Dynamic Class Loading**: On-demand class loading with superclass and interface resolution, caching via DashMap
- **Class Initialization**: Automatic `<clinit>` method execution following JVM Spec 5.5, with recursive initialization tracking
- **Bytecode Execution**: Interpreter for 50+ JVM bytecode instructions including:
  - Constants: `aconst_null`, `iconst_*`, `lconst_*`, `fconst_*`, `dconst_*`, `bipush`, `sipush`, `ldc`, `ldc_w`, `ldc2_w`
  - Load/Store: `iload`, `lload`, `fload`, `dload`, `aload`, `istore`, `lstore`, `fstore`, `dstore`, `astore` (including `_0-3` variants)
  - Array operations: `iaload`, `laload`, `faload`, `daload`, `aaload`, `baload`, `caload`, `saload`, `iastore`, `lastore`, `fastore`, `dastore`, `aastore`, `bastore`, `castore`, `sastore`, `arraylength`
  - Stack manipulation: `pop`, `pop2`, `dup`, `dup_x1`, `dup_x2`, `dup2`, `dup2_x1`, `dup2_x2`, `swap`
  - Arithmetic: `iadd`, `ladd`, `fadd`, `dadd`, `isub`, `lsub`, `fsub`, `dsub`, `imul`, `lmul`, `fmul`, `dmul`, `idiv`, `ldiv`, `fdiv`, `ddiv`, `irem`, `lrem`, `frem`, `drem`, `ineg`, `lneg`, `fneg`, `dneg`
  - Bitwise: `ishl`, `lshl`, `ishr`, `lshr`, `iushr`, `lushr`, `iand`, `land`, `ior`, `lor`, `ixor`, `lxor`
  - Type conversions: `i2l`, `i2f`, `i2d`, `l2i`, `l2f`, `l2d`, `f2i`, `f2l`, `f2d`, `d2i`, `d2l`, `d2f`, `i2b`, `i2c`, `i2s`
  - Comparisons: `lcmp`, `fcmpl`, `fcmpg`, `dcmpl`, `dcmpg`
  - Control flow: `ifeq`, `ifne`, `iflt`, `ifge`, `ifgt`, `ifle`, `if_icmp*`, `if_acmp*`, `goto`, `ifnull`, `ifnonnull`
  - Object operations: `new`, `newarray`, `anewarray`, `multianewarray`, `checkcast`, `instanceof`
  - Field access: `getstatic`, `putstatic`, `getfield`, `putfield`
  - Method invocation: `invokevirtual`, `invokespecial`, `invokestatic`, `invokeinterface`
  - Returns: `ireturn`, `lreturn`, `freturn`, `dreturn`, `areturn`, `return`
- **Object Model**: Full object creation, field storage, and array support (primitive and reference arrays)
- **JNI Support**: Implementation of 80+ JNI functions for native method integration
- **Native Library Loading**: Dynamic loading of native libraries (DLLs on Windows)
- **Stack Traces**: Detailed stack trace generation with line number mapping from class file attributes
- **Module System**: Support for loading classes from 7z binary image archives (JDK modules)
- **Frame-based Execution**: Proper execution context with program counter, operand stack, and local variables

### In Development

- Additional bytecode instructions (`tableswitch`, `lookupswitch`, `monitorenter`, `monitorexit`, etc.)
- Exception handling (`athrow`, try/catch blocks)
- Garbage collection (basic object manager exists)
- Reflection API
- Multi-threading support
- Method handles and `invokedynamic`

## Architecture

### Core Components

- **`Vm`** (`vm.rs`): Main virtual machine controller managing threads, class loader, and native library loading
- **`VmThread`** (`thread.rs`): Thread of execution managing the frame stack and method invocation
- **`Frame`** (`lib.rs`): Execution context for a method with PC, operand stack, and local variables
- **`ClassLoader`** (`class_loader.rs`): Handles dynamic class loading, linking, and initialization
- **`RuntimeClass`** (`class.rs`): Runtime representation of a loaded class with initialization state tracking
- **`ClassFile`** (`class_file/`): Binary parser for Java class files using the `deku` library
- **`ConstantPool`** (`class_file/constant_pool.rs`): Constant pool resolution and management
- **`ObjectManager`** (`objects/object_manager.rs`): Object allocation and garbage collection management
- **`JNI`** (`jni.rs`): Java Native Interface implementation

### Execution Flow

1. **Loading**: `ClassFile::from_bytes()` parses binary class file data
2. **Resolution**: `ClassLoader` converts `ClassFile` to `RuntimeClass`, resolving dependencies
3. **Initialization**: Class initializers (`<clinit>`) execute per JVM Spec 5.5
4. **Execution**: `VmThread` invokes the main method, creating a `Frame`
5. **Interpretation**: `Frame` iterates through bytecode operations, executing each instruction
6. **Stack Operations**: Instructions manipulate the operand stack and local variables

## Project Structure

```
roast-vm/
├── Cargo.toml                      # Workspace configuration
└── crates/
    ├── core/                       # Main JVM implementation (roast-vm-core)
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs             # Entry point (binary: roast)
    │       ├── lib.rs              # Frame and bytecode execution
    │       ├── vm.rs               # Virtual Machine controller
    │       ├── thread.rs           # Thread execution management
    │       ├── class.rs            # RuntimeClass definition
    │       ├── class_loader.rs     # ClassLoader implementation
    │       ├── class_file/         # Binary class file parser
    │       │   ├── class_file.rs   # ClassFile parser (magic 0xCAFEBABE)
    │       │   └── constant_pool.rs
    │       ├── objects/            # Object model
    │       │   ├── object.rs       # Object representation
    │       │   ├── array.rs        # Array support
    │       │   └── object_manager.rs
    │       ├── jni.rs              # JNI implementation
    │       ├── instructions.rs     # Bytecode opcode definitions
    │       ├── attributes.rs       # Class file attributes
    │       ├── value.rs            # Value and stack types
    │       ├── error.rs            # Error handling and stack traces
    │       ├── native_libraries.rs # Native library management
    │       └── bimage.rs           # Binary image (7z) reader
    │
    └── roast-vm-sys/               # Native methods bridge (cdylib)
        ├── Cargo.toml
        └── src/
            ├── lib.rs              # Native method implementations
            ├── system.rs           # System native calls
            ├── class.rs            # Class native operations
            └── object.rs           # Object native operations
```

## Dependencies

- **`deku`**: Binary parsing and serialization for class files
- **`dashmap`**: Concurrent HashMap for class and object storage
- **`jni`**: Java Native Interface bindings
- **`libloading`**: Dynamic library loading
- **`libffi`**: Foreign function interface for native calls
- **`sevenz-rust2`**: 7z archive reading for module system support
- **`log`** / **`env_logger`**: Logging infrastructure
- **`itertools`**: Iterator utilities
- **`colored`**: Colored console output

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

This project is in early development (v0.1.0). The core infrastructure for class loading, bytecode execution, object creation, JNI support, and stack traces is functional. Many JVM features remain in development.



## References

- [JVM Specification](https://docs.oracle.com/javase/specs/jvms/se25/html/index.html)
- [Java Class File Format](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html)