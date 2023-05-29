#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


// #[cfg(test)]
// fn run() {
//     unsafe {
//         let res = libsrt_sys::srt_startup();
//         println!("srt_startup: {}", res);
//
//         let res = libsrt_sys::srt_cleanup();
//         println!("srt_cleanup: {}", res);
//     }
// }

