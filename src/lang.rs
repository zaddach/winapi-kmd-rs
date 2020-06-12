use core::panic::PanicInfo;

#[panic_handler]
extern fn panic_fmt(_info: &PanicInfo) -> ! {
	KdPrint!("panic_fmt() -> !");
	loop{}
}
