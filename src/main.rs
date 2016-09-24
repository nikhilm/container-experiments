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

fn gethostname() -> String {
    let mut hostname = Vec::with_capacity(256);
    let cs = unsafe { CString::from_vec_unchecked(hostname) };
    let ptr = cs.into_raw();
    let r = unsafe { libc::gethostname(ptr, 256) };
    if r == -1 {
        perror("gethostname");
    }

    // retake ownership
    let own = unsafe { CString::from_raw(ptr) };

    let s = own.into_string().unwrap();
    return s;
}

extern "C" fn enter(_: *mut libc::c_void) -> libc::c_int {
    let mypid = unsafe { libc::getpid() };
    println!("I'm called from clone. Pid {}", mypid);

    // let mut uts = Box::new(libc::utsname {
    //    sysname: [0; 65],
    //    nodename: [0; 65],
    //    release: [0; 65],
    //    version: [0; 65],
    //    machine: [0; 65],
    //    domainname: [0; 65],
    // });
    // let r = unsafe { libc::uname(&mut *uts) };
    // if r == -1 {
    //    perror("uname");
    // }

    // let nn = unsafe { CStr::from_ptr(uts.nodename.as_ptr()) };
    // println!("Node name {:?}", nn);

    println!("Host name in child {}", gethostname());

    std::thread::sleep(std::time::Duration::from_secs(30));
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
    let child_pid = unsafe {
        clone(enter,
              stack_top,
              libc::CLONE_NEWUTS | libc::SIGCHLD,
              bytesfoo)
    };
    if child_pid == -1 {
        perror("clone");
    }
    println!("Clone ret {}", child_pid);

    let wait_r = unsafe { libc::waitpid(child_pid, std::ptr::null_mut(), 0) };
    if wait_r == -1 {
        perror("waitpid");
    }
}

fn main() {
    println!("Hello");
    child_in_new_uts();
}
