use crate::Result;
use libc::c_void;
use std::ptr;
use std::slice::from_raw_parts_mut;
//use std::mem;
//use std::error;

// const CTL_HW: i32 = 6;

// const HW_SENSORS: i32 = 11;
// const HW_MODEL: i32 = 2;

// const SENSOR_MAX_TYPES: usize = 23;

// #[repr(C)]
// #[derive(Default)]
// struct SensorDev {
// num: i32,        /* sensordev number */
// xname: [u8; 16], /* unix device name */
// maxnumt: [i32; SENSOR_MAX_TYPES],
// sensors_count: i32,
// }

// T must match requirements for from_raw_parts_mut.
pub fn as_bytes_ref<'a, T: ?Sized>(data: &mut T, len: usize) -> &'a mut [u8] {
    unsafe { from_raw_parts_mut(ptr::from_mut(data) as *mut u8, len) }
}

pub fn sysctl(
    name: &[i32],
    namelen: u32,
    old: Option<&mut [u8]>,
    new: Option<&mut [u8]>,
) -> Result<usize> {
    let mib: *const i32 = name.as_ptr();
    let mut oldlenp: usize = 0;
    let oldp: *mut c_void = match old {
        Some(o) => {
            oldlenp = o.len();
            o.as_mut_ptr() as *mut c_void
        }
        None => ptr::null_mut(),
    };
    let mut newlen: usize = 0;
    let newp: *mut c_void = match new {
        Some(n) => {
            newlen = n.len();
            n.as_mut_ptr() as *mut c_void
        }
        None => ptr::null_mut(),
    };

    unsafe {
        if libc::sysctl(
            mib,
            namelen,
            oldp,
            ptr::from_mut(&mut oldlenp),
            newp,
            newlen,
        ) == -1
        {
            return Err(std::io::Error::last_os_error().into());
        }
    }

    Ok(oldlenp)
}

// fn test1() {
//     let mib = [CTL_HW, HW_MODEL, 0, 0, 0];
//     let mut model: Vec<u8>;

//     let len = sysctl(&mib, 2, None, None).unwrap();

//     model = Vec::<u8>::with_capacity(len);
//     model.resize(len, 0);

//     sysctl(&mib, 2, Some(as_bytes_ref(&mut model[0], len)), None).unwrap();

//     println!("{} {}", len, String::from_utf8_lossy(model.as_slice()));
//     println!("Done");
// }

// fn test2() {
//     let mut mib = [CTL_HW, HW_SENSORS, 0, 0, 0];
//     let mut device = SensorDev::default();

//     while let Ok(_) = sysctl(
//         &mib,
//         3,
//         Some(as_bytes_ref(&mut device, mem::size_of::<SensorDev>())),
//         None,
//     ) {
//         println!("{}", String::from_utf8_lossy(&device.xname));
//         mib[2] += 1;
//     }
//     println!("Done");
// }

// fn test3() {
//     let mib = [CTL_HW, HW_MODEL, 0, 0, 0];
//     let mut model: Vec<u8>;

//     let len = sysctl(&mib, 2, None, None).unwrap();

//     model = Vec::<u8>::with_capacity(len);
//     model.resize(len, 0);

//     sysctl(&mib, 2, Some(model.as_mut_slice()), None).unwrap();

//     println!("{} {}", len, String::from_utf8_lossy(model.as_slice()));
//     println!("Done");
// }

// fn main() {
//     test1();
//     test2();
//     test3();
// }
