use libc::{c_char, c_int, c_void};
use std::mem::ManuallyDrop;

fn hello() {
    println!("Hello from Rust!");
}

#[repr(C)]
pub struct Module {
    node: Hashnode,
    u: U,
    autoloads: LinkList,
    deps: LinkList,
    wrapper: c_int,
}
#[repr(C)]
pub struct Hashnode {
    next: *mut Hashnode,
    nam: *mut c_char,
    flags: c_int,
}

#[repr(C)]
pub union U {
    handle: *mut c_void,
    linked: ManuallyDrop<LinkedMod>,
    alias: *mut c_char,
}

#[repr(C)]
pub struct LinkList {
    first: *mut LinkNode,
    last: *mut LinkNode,
    flags: c_int,
}

#[repr(C)]
pub struct LinkNode {
    first: *mut LinkNode,
    last: *mut LinkNode,
    dat: *mut c_void,
}

#[repr(C)]
pub union LinkRoot {
    list: ManuallyDrop<LinkList>,
    node: ManuallyDrop<LinkNode>,
}
#[repr(C)]
pub struct LinkedMod {
    name: *mut c_char,
    setup: extern "C" fn(Module) -> c_int,
    features: extern "C" fn(Module, *mut *mut *mut c_char) -> c_int,
    enables: extern "C" fn(Module, *mut *mut c_int) -> c_int,
    boot: extern "C" fn(Module) -> c_int,
    cleanup: extern "C" fn(Module) -> c_int,
    finish: extern "C" fn(Module) -> c_int,
}

#[no_mangle]
pub extern "C" fn setup_(_m: Module) -> c_int {
    hello();
    0
}

#[no_mangle]
pub extern "C" fn features_(_m: Module, _features: *mut *mut *mut c_char) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn enables_(_m: Module, _enables: *mut *mut c_int) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn boot_(_m: Module) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_(_m: Module) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn finish_(_m: Module) -> c_int {
    0
}
