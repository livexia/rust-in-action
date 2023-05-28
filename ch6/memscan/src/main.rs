use libc::{c_int, pid_t};
use mach2::vm::mach_vm_read;
use std::io;

use mach2::kern_return::{kern_return_t, KERN_SUCCESS};
use mach2::port::{mach_port_name_t, MACH_PORT_NULL};

type Pid = pid_t;

fn main() {
    let task = task_for_pid(70793).unwrap();
    println!("task: {task}");
    let base_addr = 0x16d0ce534;
    let size = 4;
    let raw_data = vm_read(task, base_addr, size).unwrap();
    let data: u32 =
        unsafe { std::mem::transmute::<[u8; 4], u32>(raw_data[..4].try_into().unwrap()) };
    println!("data at 0x{base_addr:0x} is {data}");
}

pub fn vm_read(task: mach_port_name_t, base_addr: usize, size: usize) -> io::Result<Vec<u8>> {
    let mut buf = vec![0u8; size];
    let mut read_len = 0;

    let result = unsafe {
        mach_vm_read(
            task,
            base_addr as _,
            size as _,
            buf.as_mut_ptr() as _,
            &mut read_len,
        )
    };
    if result != KERN_SUCCESS {
        return Err(io::Error::last_os_error());
    }
    println!("{:?}", buf);
    Ok(buf)
}

//
// Referenced from <https://github.com/rbspy/proc-maps/blob/master/src/mac_maps/mod.rs> (MIT)
// Copyright (c) 2016 Julia Evans, Kamal Marhubi Portions (continuous integration setup) Copyright (c) 2016 Jorge Aparicio
//
pub fn task_for_pid(pid: Pid) -> io::Result<mach_port_name_t> {
    let mut task: mach_port_name_t = MACH_PORT_NULL;
    // sleep for 10ms to make sure we don't get into a race between `task_for_pid` and execing a new
    // process. Races here can freeze the OS because of a Mac kernel bug on High Sierra.
    // See https://jvns.ca/blog/2018/01/28/mac-freeze/ for more.
    std::thread::sleep(std::time::Duration::from_millis(10));
    unsafe {
        let result =
            mach2::traps::task_for_pid(mach2::traps::mach_task_self(), pid as c_int, &mut task);
        if result != KERN_SUCCESS {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(task)
}
