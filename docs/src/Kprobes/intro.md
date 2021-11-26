# intro to Kprobes
Kprobes enables you to dynamically insert breakpoint into nearly any kernel routine and specifying a handler routine to be invoked when the breakpoint is hit. it does so by replacing the instruction to trap with `INT3` on x86 (or equivalent instructions on other platforms) and single step the replaced instruction in breakpointe handler after executing user specified handler routines.

## single step
single stepping the replaced instruction is trickier than it seems. only if the instruction would not produce exceptions or results related to pc or privilege levels can we safely run it out of the original context. otherwise we must take into consideration of all possible side effects and remediate them.

## simplified implementation
hereby we propose a simplified implementation of Kprobe that is easier to implement, while introducing some limitations. instead of allowing trapping at any point in any traced function, we only trace functions that eventually returns to its caller without recursions, and limit the breakpoint to be the entry point of the functions.

the way to insert the breakpoint remains the same, but when the breakpoint is hit, we replace the content of `ra` register with our own handler, restore the original instruction, run user defined handlers, then jump back to the entry point of the function. when the function returns (to our handler), we insert the breakpoint again and wait for it to be hit again.  

while this implementation saves us the cost of single stepping the trapped instruction, it imposes much restriction on the function to be traced and generally won't work on SMP systems.
