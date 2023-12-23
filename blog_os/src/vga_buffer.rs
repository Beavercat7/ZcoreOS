//颜色,我们使用枚举表示特定的颜色
#[allow(dead_code)]
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(u8)]
pub enum Color{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow =14,
    White = 15,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(transparent)]
struct ColorCode(u8);

//包装了一个完整的颜色代码字节
//包含前景色和背景色信息
//前4个bit是foreground所以background<<4
impl ColorCode{
    fn new(foreground:Color,background:Color)->ColorCode{
        ColorCode((background as u8)<<4|(foreground as u8))
    }
}

//字符缓冲区,添加更多的结构体来描述屏幕上的字符和整个字符缓冲区
//Rust并不保证按顺序布局成员变量
//因此，我们需要使用 #[repr(C)] 标记结构体；这将按 C 语言约定的顺序布局它的成员变量，让我们能正确地映射内存片段。
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(C)]
struct ScreenChar{
    ascii_character:u8,
    color_code:ColorCode,
}

const BUFFER_HEIGHT:usize = 25;
const BUFFER_WIDTH:usize = 80;
//我们再次使用 repr(transparent)，来确保类型和它的单个成员有相同的内存布局。
#[repr(transparent)]
struct Buffer{
    chars:[[ScreenChar;BUFFER_WIDTH];BUFFER_HEIGHT],
}

//为了输出字符到屏幕,我们来创建一个Writer类型
pub struct Writer{
    ///要写入的最后一行所在列的位置
    column_position:usize,
    ///事先设置好的前景色和背景色
    color_code:ColorCode,
    ///区间的VGA字符缓冲区
    buffer:&'static mut Buffer,
}

//打印字符:我们可以使用Writer来更改缓冲区内的字符了
//为了写入一个ASCII码字节,我们创建这样的函数
//要写入的字节为byte
impl Writer{
    pub fn write_byte(&mut self,byte:u8){
        match byte{
            b'\n' => self.new_line(),
            byte =>{
                if self.column_position >= BUFFER_WIDTH{
                    self.new_line();
                }
            
        
            //写入最后一行
            let row = BUFFER_HEIGHT-1;
            let col = self.column_position;

            let color_code = self.color_code;
            self.buffer.chars[row][col] = ScreenChar{
                ascii_character:byte,
                color_code,
            };
            self.column_position+=1;
            }
        }
    }
    fn new_line(&mut self){

    }
    ///要打印整个字符串，我们将它转换成字节并依次输出
    pub fn write_string(&mut self,s:&str){
        for byte in s.bytes(){
            match byte{
                //可以是能打印的ASCII码字符,也可以是换行符
                0x20..=0x7e|b'\n' => self.write_byte(byte),
                //不包含在上述范围之内的的字节
                _ => self.write_byte(0xfe),
            }
        }
    }
}
//编写一个临时的函数,来测试
pub fn print_something(){
    //首先创建一个指向0xb8000地址VHA缓冲区的Write

    let mut writer = Writer{
        column_position:0,
        color_code:ColorCode::new(Color::Yellow,Color::Black),
        buffer:unsafe{
            &mut *(0xb8000 as *mut Buffer)},
        };
    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
//易失操作,这告诉编译器,这些写入可能会产生副效应,不应该被优化掉
//volatile包装了read、write方法,这些方法包装了
//core::ptr内的read_volatile和write_volatile函数
//从而保证读操作和写操作不会被编译器优化