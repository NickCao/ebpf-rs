# ebpf-rs
A verifier, interpreter, JIT compiler for eBPF and corresponding facilities to hook them into [rCore](https://github.com/rcore-os/rCore) written in rust.

## relevant works
[rbpf](https://github.com/qmonnet/rbpf) Rust (user-space) virtual machine for eBPF  
[redbpf](https://github.com/foniod/redbpf) Rust library for building and running BPF/eBPF modules  
[aya](https://github.com/aya-rs/aya) Aya is an eBPF library for the Rust programming language  

## references
[linux bpf uapi](https://elixir.bootlin.com/linux/v5.14.12/source/include/uapi/linux/bpf.h) include/uapi/linux/bpf.h  
[linux bpf impl](https://elixir.bootlin.com/linux/v5.14.12/source/kernel/bpf) kernel/bpf  
[BPF and XDP Reference Guide](https://docs.cilium.io/en/stable/bpf/) from cilium documentation  
[BPF Internals](https://www.usenix.org/conference/lisa21/presentation/gregg-bpf) from usenix presentation  
