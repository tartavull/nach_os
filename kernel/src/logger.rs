use crate::{serial_print,serial_println};

const Green: &str = "\u{001b}[32m";
const White: &str = "\u{001b}[37m";

const CodeStart: &str = "\u{001b}[3";
const CodeEnd: &str = "m";

// https://chrisyeh96.github.io/2020/03/28/terminal-colors.html
#[derive(Debug)]
enum Color {
    Black = 0,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

macro_rules! color {
    ($color_code:expr) => ( concat!("\u{001b}[3", $color_code, "m") )
}


pub fn debug(msg: &str) {
    serial_print!("{}debug:{} {}\n", color!(2), color!(7), msg);
}

pub fn info(msg: &str) {
    serial_print!("{}info:{} {}\n", color!(4), color!(7), msg);
}

pub fn warning(msg: &str) {
    serial_print!("{}warning:{} {}\n", color!(3), color!(7), msg);
}

pub fn error(msg: &str) {
    serial_print!("{}error:{} {}\n", color!(1), color!(7), msg);
}

pub fn print_logo() {
    const N: &str = r"
      ___     
     /\__\    
    /::|  |   
   /:|:|  |   
  /:/|:|  |__ 
 /:/ |:| /\__\
 \/__|:|/:/  /
     |:/:/  / 
     |::/  /  
     /:/  /   
     \/__/    
";

    const A: &str = r"
      ___     
     /\  \    
    /::\  \   
   /:/\:\  \  
  /::\~\:\  \ 
 /:/\:\ \:\__\
 \/__\:\/:/  /
      \::/  / 
      /:/  /  
     /:/  /   
     \/__/    
";
    
    const C: &str = r"
      ___     
     /\  \    
    /::\  \   
   /:/\:\  \  
  /:/  \:\  \ 
 /:/__/ \:\__\
 \:\  \  \/__/
  \:\  \      
   \:\  \     
    \:\__\    
     \/__/    
    ";

    const H: &str = r"
      ___     
     /\__\    
    /:/  /    
   /:/__/     
  /::\  \ ___ 
 /:/\:\  /\__\
 \/__\:\/:/  /
      \::/  / 
      /:/  /  
     /:/  /   
     \/__/    
    ";

    const O: &str = r"
      ___     
     /\  \    
    /::\  \   
   /:/\:\  \  
  /:/  \:\  \ 
 /:/__/ \:\__\
 \:\  \ /:/  /
  \:\  /:/  / 
   \:\/:/  /  
    \::/  /   
     \/__/    
    ";

    const S: &str = r"
      ___     
     /\  \    
    /::\  \   
   /:/\ \  \  
  _\:\~\ \  \ 
 /\ \:\ \ \__\
 \:\ \:\ \/__/
  \:\ \:\__\  
   \:\/:/  /  
    \::/  /   
     \/__/    
    ";

    
    // https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
    serial_print!("\u{001b}[2J");
    for n in 0..11 {
        let start =  n * 15 + 1;
        let end = start + 14;
        serial_print!("{}{} {} {} {} {}{} {}\n", 
            color!(4),
            &N[start..end], 
            &A[start..end], 
            &C[start..end],
            &H[start..end],
            color!(1),
            &O[start..end],
            &S[start..end]
            );
    }
}
