use crate::{gdt, println, scheduler, process};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use pic8259::ChainedPics;
use x86_64::instructions::port::Port;

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn catchall_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: catchall\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    // this gets called if we call SYSENTER
    println!("EXCEPTION: invalid_opcode\n{:#?}", stack_frame);
}


extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn syscall_handler(stack_frame: InterruptStackFrame) {
    use core::arch::asm;

    let rax: u64; // system call number
    let rcx: u64; // return address
    let r11: u64; // saved rflags
    let rdi: u64; // arg0
    let rsi: u64; // arg1
    let rdx: u64; // arg2
    let r10: u64; // arg3
    let r9: u64;  // arg4
    let r8: u64;  // arg5
    unsafe {
        asm!(
            "mov {}, rax", 
            "mov {}, rcx",
            "mov {}, r11",
            "mov {}, rdi",
            "mov {}, rsi",
            "mov {}, rdx",
            "mov {}, r10",
            "mov {}, r9",
            "mov {}, r8",
             out(reg) rax,
             out(reg) rcx,
             out(reg) r11,
             out(reg) rdi,
             out(reg) rsi,
             out(reg) rdx,
             out(reg) r10,
             out(reg) r9,
             out(reg) r8,
        );
    }
    process::syscall_interrupt(rax, rcx, r11, &[rdi, rsi, rdx, r10, r9, r8]);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
    RTC
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    scheduler::step();
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scancode);
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        //idt.divide_by_zero.set_handler_fn(catchall_handler);
        //idt.debug.set_handler_fn(catchall_handler);
        //idt.non_maskable_interrupt.set_handler_fn(catchall_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        //idt.overflow.set_handler_fn(catchall_handler);
        //idt.bound_range_exceeded.set_handler_fn(catchall_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        //idt.device_not_available.set_handler_fn(catchall_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        //idt.invalid_tss
        //idt.segment_not_present
        //idt.stack_segment_fault
        //idt.general_protection_fault
        //idt.page_fault
        idt.x87_floating_point.set_handler_fn(catchall_handler);
        //idt.alignment_check
        //idt.machine_check
        idt.simd_floating_point.set_handler_fn(catchall_handler);
        idt.virtualization.set_handler_fn(catchall_handler);
        //idt.security_exception
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);
        idt[0x80].set_handler_fn(syscall_handler);
        idt
    };
}
