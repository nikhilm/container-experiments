extern crate libc;

use libc::clone;
use std::ffi::CString;
use std::ffi::CStr;

fn perror(a: &str) {
    let p = CString::new(a).unwrap();
    unsafe {
        libc::perror(p.as_ptr());
    }
}

fn sethostname(new: &str) {
    let cs = CString::new(new).unwrap();
    let ptr = cs.as_ptr();
    let r = unsafe { libc::sethostname(ptr, new.len()) };
    if r == -1 {
        perror("sethostname");
    }
}

fn gethostname() -> String {
    let mut hostname: Vec<u8> = Vec::with_capacity(255);
    let ptr = hostname.as_mut_ptr();
    let r = unsafe { libc::gethostname(ptr as *mut libc::c_char, 255) };
    if r == -1 {
        perror("gethostname");
        return String::new();
    }

    unsafe {
        let len = CStr::from_ptr(ptr as *const libc::c_char).to_bytes().len();
        hostname.set_len(len);
    }
    let s = String::from_utf8(hostname).unwrap();
    return s;
}

extern "C" fn enter(_: *mut libc::c_void) -> libc::c_int {
    let mypid = unsafe { libc::getpid() };
    println!("Child pid {}", mypid);
    let s = gethostname();
    {
        println!("Original host name in child {}", s);
    }
    sethostname("temporary-only-in-child");
    println!("Host name in child {}", gethostname());

    println!("Sleeping for 10 seconds");
    std::thread::sleep(std::time::Duration::from_secs(10));
    return 0;
}

fn child_in_new_uts() {
    let mypid = unsafe { libc::getpid() };
    println!("Parent pid {}", mypid);
    println!("Original host name in parent {}", gethostname());

    const STACK_SIZE: usize = 1024 * 1024;
    let mut stack = Vec::with_capacity(STACK_SIZE);
    let stack_top = unsafe { stack.as_mut_ptr().offset(STACK_SIZE as isize) };
    let child_pid = unsafe {
        clone(enter,
              stack_top,
              libc::CLONE_NEWUTS | libc::SIGCHLD,
              std::ptr::null_mut())
    };
    if child_pid == -1 {
        perror("clone");
    }

    let wait_r = unsafe { libc::waitpid(child_pid, std::ptr::null_mut(), 0) };
    if wait_r == -1 {
        perror("waitpid");
    }

    println!("Host name in parent {}", gethostname());
}

fn main() {
    child_in_new_uts();
}
