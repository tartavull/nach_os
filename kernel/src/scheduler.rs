use crate::{print, println, process};

// We probably want to have some sort of heap from which we take out
// the element from highest priority, execute it, and then put it back to the bottom

pub fn step() {
    // 1. Find the highest priority process to run
    // context switch to them
    print!(".");
}

fn context_switch(p: process::Process) {
    // 1. set up pagging
    println!("scheduler: switching to: \n{:#?}", p);
}

pub fn create_process(_start: &dyn Fn()) {
    // we should pass a function that starts the program.
    _start();
    println!("scheduler: process created: \n");
}
