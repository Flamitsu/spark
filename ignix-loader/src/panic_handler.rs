use core::panic::PanicInfo;
#[cfg(not(test))]
#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    loop{}
}
