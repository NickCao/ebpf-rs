# clang

## build
instead of manually crafting eBPF programs, we can use clang to compile a subset of c code into eBPF programs  

    clang -target bpf -Werror -O2 -c ebpf.c -o ebpf.o

notice that `-O2` MUST NOT be omitted, otherwise clang would generate pseudo instructions out of the eBPF specification  

## disassemble

    llvm-objdump --triple=bpf -S ebpf.o
