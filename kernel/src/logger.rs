use crate::{serial_print,serial_println};
use crate::serial::print;

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


#[macro_export]
macro_rules! color {
    ($color_code:expr) => ( concat!("\u{001b}[3", $color_code, "m") )
}

pub fn _log(color: &str, name: &str, args: ::core::fmt::Arguments) {
    serial_print!("{}{}:{} ", color, name, color!(7));
    crate::serial::print(args);
    serial_println!();
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logger::_log(
            $crate::color!(2),
            "debug",
            format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::_log(
            $crate::color!(4),
            "info",
            format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        $crate::logger::_log(
            $crate::color!(3),
            "warning",
            format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::_log(
            $crate::color!(1),
            "error",
            format_args!($($arg)*));
    };
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
    serial_print!("\n\n");
}
