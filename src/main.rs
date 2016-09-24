extern crate libc;

use libc::clone;
use std::ffi::CString;

extern "C" fn enter(_: *mut libc::c_void) -> libc::c_int {
    let mypid = unsafe { libc::getpid() };
    println!("I'm called from clone. Pid {}", mypid);
    return 0;
}

fn child_in_new_uts() {
    let mypid = unsafe { libc::getpid() };
    println!("Original program Pid {}", mypid);

    const STACK_SIZE: usize = 1024 * 1024;
    let mut stack = Vec::with_capacity(STACK_SIZE);
    let stack_top = unsafe { stack.as_mut_ptr().offset(STACK_SIZE as isize) };
    let arg = CString::new("/bin/bash").unwrap();
    let bytes = arg.into_raw();
    let bytesfoo: *mut libc::c_void = bytes as *mut libc::c_void;
    let r = unsafe { clone(enter, stack_top, libc::SIGCHLD, bytesfoo) };
    if r == -1 {
        let p = CString::new("clone").unwrap();
        unsafe {
            libc::perror(p.as_ptr());
        }
    }
    println!("Clone ret {}", r);
}

fn main() {
    println!("Hello");
    child_in_new_uts();
}
