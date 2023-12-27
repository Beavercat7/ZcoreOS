use x86_64::structures::idt::{InterruptDescriptorTable,InterruptStackFrame};
use crate::println;  
use lazy_static::lazy_static;
lazy_static!{
    static ref IDT:InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}
//static mut idt = InterruptDescriptorTable::new();
pub fn init_idt(){
    // unsafe{
    // idt.breakpoint.set_handler_fn(breakpoint_handler);
    // //要让CPU使用新的中断描述符表
    // //需要使用lidt指令来装载一下
    // idt.load();
    // }
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame:InterruptStackFrame)
{
    println!("EXCEPTION:BREAKPOINT\n:{:#?}",stack_frame);
}

//运行cargo test --lib来运行lib.rs及其子模块中包含的测试
#[test_case]
fn test_breakpoint_exception(){
    x86_64::instructions::interrupts::int3();
}