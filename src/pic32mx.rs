
#[cfg(target_arch="mipsel")]
mod port
{

}


/* heap_4.rs */
#[cfg(target_arch="mipsel")]
mod port
{
	struct BlockLink {
	}

	pub fn malloc(size: uint) -> *mut u8 {
    	vTaskSuspendAll();

    	xTaskResumeAll() as ();
    }
}
