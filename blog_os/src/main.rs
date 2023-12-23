#![no_std]//不链接Rust标准库
#![no_main]//禁用所有Rust层级的入口点
use core::panic::PanicInfo;
mod vga_buffer;
//这个函数将在panic时被调用

static HELLO:&[u8] = b"Hello World!";

#[no_mangle]//不重整函数名
pub extern "C" fn _start()->!{
    // let vga_buffer = 0xb8000 as *mut u8;

    // for(i,&byte) in HELLO.iter().enumerate(){
    //     unsafe{
    //         *vga_buffer.offset(i as isize*2) = byte;
    //         *vga_buffer.offset(i as isize*2+1) = 0xb;
    //     }
    // }
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(),", some numbers: {} {}",42,1.337).unwrap();
    println!("Welcome to ZcoreOS {}","!");
    loop{}
}

#[panic_handler]
fn panic(info:&PanicInfo)->!{
    println!("{}",info);
    loop{}
}

//为我们的内核实现自定义测试框架
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]

// //我们的runner会打印一个简短的debug信息
// #[cfg(test)]
// fn test_runner(tests:&[&dyn Fn()]){
//     println!("Running {} tests",tests.len());
//     for test in tests{
//         test();
//     }
// }