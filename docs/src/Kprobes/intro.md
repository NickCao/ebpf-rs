# intro to Kprobes
Kprobes enables you to dynamically insert breakpoint into nearly any kernel routine and specifying a handler routine to be invoked when the breakpoint is hit. it does so by replacing the instruction to trap with `INT3` on x86 (or equivalent instructions on other platforms) and single step the replaced instruction in breakpointe handler after executing user specified handler routines.

A fork of rCore with Kprobes implements can be found at [https://github.com/NickCao/rCore](https://github.com/NickCao/rCore)

## single step
single stepping the replaced instruction is trickier than it seems. only if the instruction would not produce exceptions or results related to pc or privilege levels can we safely run it out of the original context. otherwise we must take into consideration of all possible side effects and remediate them.

## simplified implementation
hereby we propose a simplified implementation of Kprobe that is easier to implement, while introducing some limitations. instead of allowing trapping at any point in any traced function, we only trace functions that eventually returns to its caller without recursions, and limit the breakpoint to be the entry point of the functions.

the way to insert the breakpoint remains the same, but when the breakpoint is hit, we replace the content of `ra` register with our own handler, restore the original instruction, run user defined handlers, then jump back to the entry point of the function. when the function returns (to our handler), we insert the breakpoint again and wait for it to be hit again.  

while this implementation saves us the cost of single stepping the trapped instruction, it imposes much restriction on the function to be traced and generally won't work on SMP systems.

## run handler on function return
when probing functions, both the entry point and exit of functions are interested, thus we need to redirect the return address of the function to an address we control in order to run custom handlers on function return. Since recursive functions and on SMP systems, a single function may be called multiple times simultaneously, we cannot set ra to a static value, instead we need a trampoline, a contiguous region of ebreak instructions, and set ra to an unused address within that range, in order to distinguish between multiple invocations. A trampoline inplemented as such may not be able to handle functions on hot paths due to it's limited size, thus measures to dynamically increase it's size should be implemented, and don't forget to flush icache when doing so.
