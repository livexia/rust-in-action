use libc::{c_int, pid_t};
use mach2::message::mach_msg_type_number_t;
use mach2::vm::{mach_vm_read_overwrite, mach_vm_region, mach_vm_write};
use mach2::vm_region::{
    vm_region_basic_info_data_64_t, vm_region_basic_info_data_t, vm_region_info_t,
    VM_REGION_BASIC_INFO,
};
use mach2::vm_types::{mach_vm_address_t, mach_vm_size_t};
use std::error::Error;
use std::io;
use std::result;

use mach2::kern_return::{kern_return_t, KERN_SUCCESS};
use mach2::port::{mach_port_name_t, MACH_PORT_NULL};

type Pid = pid_t;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) };
}

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub fn search_value(
    task: mach_port_name_t,
    region_address: usize,
    region_size: usize,
    pattern: &[u8],
) -> Result<Vec<usize>> {
    let mut result = vec![];
    let data_size = pattern.len();
    for addr in (region_address..region_address + region_size).step_by(data_size) {
        let raw_data = vm_read_overwrite(task, addr, data_size)?;
        if raw_data == pattern {
            result.push(addr);
            println!("Found value {raw_data:?} at: 0x{addr:0x}");
        }
    }

    Ok(result)
}

pub fn vm_region(task: mach_port_name_t, mut address: mach_vm_address_t) -> Result<(usize, usize)> {
    let mut size: mach_vm_size_t = unsafe { std::mem::zeroed() };
    let mut info: vm_region_basic_info_data_64_t = unsafe { std::mem::zeroed() };
    let mut info_cnt =
        std::mem::size_of::<vm_region_basic_info_data_64_t>() as mach_msg_type_number_t;
    let mut object_name = 0;
    let result = unsafe {
        mach_vm_region(
            task,
            &mut address,
            &mut size,
            VM_REGION_BASIC_INFO,
            &mut info as *mut vm_region_basic_info_data_64_t as vm_region_info_t,
            &mut info_cnt,
            &mut object_name,
        )
    };

    if result != KERN_SUCCESS {
        return Err(Box::new(io::Error::last_os_error()));
    }
    Ok((address as usize, size as usize))
}

pub fn vm_read_overwrite(task: mach_port_name_t, address: usize, size: usize) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; size];
    let mut read_len = 0;

    let result = unsafe {
        mach_vm_read_overwrite(
            task,
            address as _,
            size as _,
            buf.as_mut_ptr() as _,
            &mut read_len,
        )
    };
    if result != KERN_SUCCESS {
        return Err(Box::new(io::Error::last_os_error()));
    }
    Ok(buf)
}

pub fn vm_write(task: mach_port_name_t, address: usize, buffer: &[u8], size: usize) -> Result<()> {
    let result = unsafe { mach_vm_write(task, address as _, buffer.as_ptr() as _, size as _) };
    if result != KERN_SUCCESS {
        return Err(Box::new(io::Error::last_os_error()));
    }
    Ok(())
}

//
// Referenced from <https://github.com/rbspy/proc-maps/blob/master/src/mac_maps/mod.rs> (MIT)
// Copyright (c) 2016 Julia Evans, Kamal Marhubi Portions (continuous integration setup) Copyright (c) 2016 Jorge Aparicio
//
pub fn task_for_pid(pid: Pid) -> Result<mach_port_name_t> {
    let mut task: mach_port_name_t = MACH_PORT_NULL;
    // sleep for 10ms to make sure we don't get into a race between `task_for_pid` and execing a new
    // process. Races here can freeze the OS because of a Mac kernel bug on High Sierra.
    // See https://jvns.ca/blog/2018/01/28/mac-freeze/ for more.
    std::thread::sleep(std::time::Duration::from_millis(10));
    unsafe {
        let result =
            mach2::traps::task_for_pid(mach2::traps::mach_task_self(), pid as c_int, &mut task);
        if result != KERN_SUCCESS {
            return Err(Box::new(io::Error::last_os_error()));
        }
    }
    Ok(task)
}

fn main() -> Result<()> {
    let pid = 85555;
    let task = task_for_pid(pid)?;
    println!("pid: {pid}, task: {task}");

    let mut address = 0;
    let mut size = 0;
    while address < usize::MAX {
        (address, size) = vm_region(task, (address + size) as _)?;
        println!("address: 0x{address:0x} size: {size}");
        if let Ok(result) = search_value(task, address, size, &900usize.to_le_bytes()) {
        } else {
            println!("Err");
        };
    }

    vm_write(
        task,
        0x16d0ce540,
        &[255, 255, 255, 255, 255, 255, 255, 255],
        8,
    )?;

    Ok(())
}
