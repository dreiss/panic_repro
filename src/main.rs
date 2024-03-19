#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod ns16550_uart;
use riscv_rt::entry;

use spinning_top::Spinlock;

use crate::ns16550_uart::Uart;
use core::fmt::Write as _;

const QEMU_VIRT_UART_BASE: usize = 0x10000000;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // Can't use the global UART instance, because it might be locked.
    // Just unsafely make a new one and hope for the best.
    let mut uart = unsafe { Uart::new(QEMU_VIRT_UART_BASE) };

    let _ = write!(uart, "Rust panic");
    if let Some(loc) = info.location() {
        let _ = write!(uart, " (at {}:{}:{})", loc.file(), loc.line(), loc.column());
    }
    let _ = write!(uart, "!\n");
    if let Some(msg) = info.message() {
        let _ = write!(uart, "{}\n", msg);
    }
    loop {}
}

static UART: Spinlock<Option<Uart>> = Spinlock::new(None);

macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write as _;
			write!(UART.lock().as_mut().unwrap(), $($args)+).unwrap();
	});
}

fn getc() -> Option<u8> {
    UART.lock().as_mut().unwrap().get()
}

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    *UART.lock() = Some(unsafe { Uart::new(QEMU_VIRT_UART_BASE) });

    print!("Hello, world!\n");

    None::<usize>.unwrap();

    loop {
        riscv::asm::wfi();
    }
}
