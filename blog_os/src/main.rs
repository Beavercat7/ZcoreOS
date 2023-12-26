#![no_std]//不链接Rust标准库
#![no_main]//禁用所有Rust层级的入口点
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
use core::panic::PanicInfo;
use blog_os::println;
//这个函数将在panic时被调用
//为我们的内核实现自定义测试框架


//我们的runner会打印一个简短的debug信息

//static HELLO:&[u8] = b"Hello World!";


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
    println!("Welcome to Zcore {}\n","!");
    //panic!("Some panic message");
    #[cfg(test)]
    test_main();
    
    //println!("test over!");
    loop{}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info:&PanicInfo)->!{
    println!("{}",info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info:&PanicInfo)->!{
   blog_os::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

//验证打印的几行字符是否真的出现在屏幕上
// #[test_case]
// fn test_println_output(){
//     let s = "Some test string that fits on a single line";
//     println!("{}", s);
//     for (i,c) in s.chars().enumerate(){
//         let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT-2][i].read();
//         assert_eq!(char::from(screen_char.ascii_character),c);
//     }
// }


