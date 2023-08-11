#![no_std] // no_std -> because Rust std libs is stores things that's we not need to use.
#![no_main]

use core::panic::PanicInfo;

use arduino_hal::prelude::_void_ResultVoidExt;

// Because set #![no_main] on line:3, Then we have to told entry point to program.
#[arduino_hal::entry]
fn main() -> ! {
    // get modules.
    let dp = arduino_hal::Peripherals::take().unwrap();

    // get pin that's module connected on.
    let pins = arduino_hal::pins!(dp);

    // Serialize: Module, Pin, baudRate.
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    loop {
        for i in 0..=10 {
            ufmt::uwriteln!(&mut serial, "B:: {}", i).void_unwrap(); // print 0 -> 10 to console.
            arduino_hal::delay_ms(2000); // set delay to program. [2sec] .
        }

        // program panic here!
        panic!();
    }
}

// Macro: panic_handler. use to told what function that program can use to handle panic.
#[panic_handler]
// This function use to handle panic at program runtime.
fn panic(info: &PanicInfo) -> ! {
    // steal peripherals from main function.
    let dp = unsafe { arduino_hal::Peripherals::steal() };

    // pin that's have a peripherals.
    let pins = arduino_hal::pins!(dp);

    // Serialize device, pin, baudRate.
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Show panic messages.
    ufmt::uwriteln!(&mut serial, "Panic!").void_unwrap();
    // show panic info messages.
    if let Some(inform) = info.location() {
        ufmt::uwriteln!(
            &mut serial, // borrow `serial` as mutable, So we can change it's value.
            "Panic INFO:: \n\tfile:: {}\n\tline:: {}\n\tcolm:: {}", // panic info format.
            inform.file(), // What file is error.
            inform.line(), // what line on file.
            inform.column(), // What column of line is error.
        )
        .void_unwrap();
        /*
         * Error Message format:
         * """
         * Panic!.
         * Panic INFO:: .
         * .file:: {{ file that's have error }}.
         * .line:: {{ line that's error on }}.
         * .colm:: {{ column that's error on }}.
         * """
         */
    }

    /*
     * program have to exit by manual.
     * so it will wait until user manual shutdown it.
     */
    loop {}
}
