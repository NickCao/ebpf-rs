#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

pub mod btf {
    include!(concat!(env!("OUT_DIR"), "/btf.rs"));
}
pub mod bpf {
    include!(concat!(env!("OUT_DIR"), "/bpf.rs"));
}
