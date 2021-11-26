# eBPF function

eBPF functions are called with specialized instruction `CALL`, by a predefined function index encoded in immediate. functions are normally provided as `helpers` by the eBPF runtime to augment the restricted functionalities of eBPF programs. some eBPF runtimes allow eBPF programs to call each other following the same calling convention with dynamically function index assigned at load time. 

## map
as the eBPF machine only has a stack of limited size without heap allocation, most eBPF runtimes provide hashmap-like data stuctures in the form of helper functions. maps are normally created when eBPF programs are being loaded into kernel, by a userland helper, and the parameters for maps such as data type or max size can be specified in special ELF sections. 
