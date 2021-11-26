# eBPF machine
the eBPF machine is a traditional RISC register machine with a total of 11 64-bit registers, a program counter and a 512 byte fixed size stack. 

## registers allocation
10 general purpose read-write registers numbered from r0 to r9, and 1 read-only frame pointer numbered r10. The pc register is implicit and cannot be read.

## calling convention
- r0: return value
- r1-r5: arguments (more than 5 arguments is not supported)
- r6-r9: callee saved
