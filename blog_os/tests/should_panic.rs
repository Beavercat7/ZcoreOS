#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::{QemuExitCode,exit_qemu,serial_println};
use blog_os::serial_print;

#[no_mangle]
pub extern "C" fn _start() -> !{
    //test_main();
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop{}
}
/*
对于只有单个测试函数的集成测试来说例如:should_panic


*/
// pub fn test_runner(tests:&[&dyn Fn()]){
//     serial_println!("Running {} tests",tests.len());
//     for test in tests{
//         test();
//         serial_println!("[test did not panic]");
//         exit_qemu(QemuExitCode::Failed);
//     }
//     exit_qemu(QemuExitCode::Success);
// }

//#[test_case]
fn should_fail(){
    serial_print!("should_fail... ");
    assert_eq!(1,1);
}

#[panic_handler]
fn panic(_info:&PanicInfo)->!{
    serial_println!("[Ok]");
    exit_qemu(QemuExitCode::Success);
    loop{}
}