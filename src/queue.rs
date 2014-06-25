
enum QueueLock {
	Unlocked = -1,
	LockedUnmodified = 0
}

struct Queue {
	head: &int,
	tail: &int,
	write_to: &int,

	recursive_call_count: uint,

	tasks_waiting_to_send: List,	//< blocked waiting to post onto this
	tasks_waiting_to_recv: List,	//< blocked waiting to read from this

	messages_waiting: uint,			//< number currently in the queue
	length: uint,					//< number of items it will hold
	item_size: uint,				//< size of each item it will hold

	rx_lock: QueueLock,
	tx_lock: QueueLock,

}
