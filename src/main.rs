// stm32f3-got-mcve.rs

// A minimal example to demonstrate the problem where:
// - using gcc-crate to compile C code under xargo with cortex-m-rt
// - gcc-crate by default compiles C code as position independent
// - some C code under these circumstances will generate a .got Global Offset
//     Table section
// - Current cortex-m-rt will not correctly relocate/fix-up the .got
//     section, nor does it give a helpful warning/error about the presence of
//     the .got.

#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;

use cortex_m::asm;

extern { fn c_test_function() -> i32; }

#[inline(never)]
fn main() {
    hprintln!("before calling c_test_function()");

    hprintln!("c_test_function() returned {}", unsafe { c_test_function() });

    hprintln!("after calling c_test_function()");
}

// ---- interrupt handlers

#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
// no (real) interrupt handlers
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
