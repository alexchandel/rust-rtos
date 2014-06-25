
mod port
{

}


/* heap_4.rs */

mod port
{
	struct BlockLink {
	}

	pub fn malloc(size: uint) -> *mut u8 {
    	vTaskSuspendAll();

    	xTaskResumeAll() as ();
    }
}
