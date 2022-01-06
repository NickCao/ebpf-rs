## symbols in Rust programs
Rust follows an obscure symbol name mangling scheme, making it impossible to recover the original type name from the symbol table, without further information from the compiler.

However a new mangling scheme is proposed, [Rust Symbol Mangling (v0)](https://rust-lang.github.io/rfcs/2603-rust-symbol-name-mangling-v0.html), which makes the mangled names more machine readable. Now that support for this mangling scheme is already implemented in both rustc and upstream toolchains, we can use it for better visibility into rust programs. 

```toml
# .cargo/config.toml
[build]
rustflags = ["-Zsymbol-mangling-version=v0"]
```

both gdb and objdump should support this scheme out of the box, for other tools, you can still use `rustfilt` manually to demangle the names.
