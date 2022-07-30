use crate::{print, println, debug, process};

// We probably want to have some sort of heap from which we take out
// the element from highest priority, execute it, and then put it back to the bottom

pub fn step() {
    // 1. Find the highest priority process to run
    // context switch to them
    //print!(".");
}

// As the call stack can be very large, the operating system typically sets up a separate call
// stack for each task instead of backing up the call stack content on each task switch. Such a
// task with a separate stack is called a thread of execution or thread for short. By using a
// separate stack for each task, only the register contents need to be saved on a context switch
// (including the program counter and stack pointer). This approach minimizes the performance
// overhead of a context switch, which is very important since context switches often occur up to
// 100 times per second.
fn context_switch(p: process::Process) {
    // 1. set up pagging
    println!("scheduler: switching to: \n{:#?}", p);
}

pub fn create_process(_start: &dyn Fn()) {
    // we should pass a function that starts the program.
    _start();
    debug!("scheduler: process created: \n");
}
