#![feature(asm)]
#![no_std]
#![no_main]

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

#[no_mangle]
fn kernel_entry() -> ! {
    // Set GPIO Pin 16 as output.
    let pin = 16;
    let shift = (pin % 10) * 3;
    unsafe {
        GPIO_FSEL1.write_volatile(GPIO_FSEL1.read_volatile() & !(0b111 << shift));
        GPIO_FSEL1.write_volatile(GPIO_FSEL1.read_volatile() | (0b001 << shift));
    }

    // Continuously set and clear GPIO 16.
    unsafe {
        loop {
            GPIO_SET0.write_volatile(GPIO_SET0.read_volatile() & !(0b1 << pin));
            GPIO_SET0.write_volatile(GPIO_SET0.read_volatile() | (0b1 << pin));
            spin_sleep_ms(500);
            GPIO_CLR0.write_volatile(GPIO_CLR0.read_volatile() & !(0b1 << pin));
            GPIO_CLR0.write_volatile(GPIO_CLR0.read_volatile() | (0b1 << pin));
            spin_sleep_ms(500);
        }
    }
}
raspi3_boot::entry!(kernel_entry);
