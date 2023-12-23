# ZcoreOS
用Rust写的一个小型操作系统  
#使用这条指令可以看到qemu输出  
#运行cargo run 再输入下一条指令  
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin  -display curses