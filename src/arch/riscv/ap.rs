#[naked]
#[no_mangle]
#[link_section = ".text"]
unsafe extern "C" fn _secondary_start() -> ! {
    core::arch::asm!("
        csrw sstatus, zero
        csrw sie, zero
        // At start, A1 holds the top of the stack.
        mv   sp, a1
        call _secondary_init
        ret
        "
    )
}