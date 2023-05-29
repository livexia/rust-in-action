use libc::{c_int, pid_t};
use mach2::message::mach_msg_type_number_t;
use mach2::vm::{mach_vm_read_overwrite, mach_vm_region};
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

fn main() -> Result<()> {
    let task = task_for_pid(70793).unwrap();
    println!("task: {task}");
    let base_addr = 0x16d0ce534;
    let size = 4;
    let raw_data = vm_read(task, base_addr, size).unwrap();
    let data: u32 = u32::from_le_bytes(raw_data[..4].try_into().unwrap());
    println!("raw data: {:?}", raw_data);
    println!("data at 0x{base_addr:0x} is {data}");

    let result = search_value(task, base_addr, &12999usize.to_le_bytes())?;
    Ok(())
}

pub fn search_value(task: mach_port_name_t, address: usize, pattern: &[u8]) -> Result<Vec<usize>> {
    let mut result = vec![];
    let data_size = pattern.len();
    let (address, size) = vm_region(task, address as _)?;
    for addr in (address..address + size).step_by(data_size) {
        let raw_data = vm_read(task, addr, data_size).unwrap();
        if raw_data == pattern {
            result.push(addr);
            println!("Found value {raw_data:?} at: 0x{addr:0x}");
        }
    }

    Ok(result)
}

pub fn vm_region(task: mach_port_name_t, mut address: mach_vm_address_t) -> Result<(usize, usize)> {
    let mut size: mach_vm_size_t = unsafe { std::mem::zeroed() };
    let mut info: vm_region_basic_info_data_t = unsafe { std::mem::zeroed() };
    let mut info_cnt =
        std::mem::size_of::<vm_region_basic_info_data_64_t>() as mach_msg_type_number_t;
    let mut object_name = 0;
    let result = unsafe {
        mach_vm_region(
            task,
            &mut address,
            &mut size,
            VM_REGION_BASIC_INFO,
            &mut info as *mut vm_region_basic_info_data_t as vm_region_info_t,
            &mut info_cnt,
            &mut object_name,
        )
    };

    if result != KERN_SUCCESS {
        return Err(Box::new(io::Error::last_os_error()));
    }
    Ok((address as usize, size as usize))
}

pub fn vm_read(task: mach_port_name_t, address: usize, size: usize) -> Result<Vec<u8>> {
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
