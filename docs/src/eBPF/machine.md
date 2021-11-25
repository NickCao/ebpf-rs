# machine

## registers
10 general purpose 64 bit registers numbered from r0 to r9, and 1 read-only frame pointer numbered r10

## calling convention
- r0         return value
- r1 - r5    arguments (more than 5 arguments is not supported)
- r6 - r9    callee saved

## memory
a fixed size stack of 512 bytes and arbitrary access to other memory regions
