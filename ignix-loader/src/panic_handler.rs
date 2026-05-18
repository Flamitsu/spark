use core::panic::PanicInfo;
use core::arch::asm;
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            asm!("hlt");
            /* Those archs uses the same instruction 'wfi' to
             * put the CPU into a deep C state*/
            #[cfg(any(
                    target_arch = "arm", 
                    target_arch = "aarch64", 
                    target_arch = "riscv64",
                    target_arch = "riscv32"))]
            asm!("wfi");
        }
    }
}
