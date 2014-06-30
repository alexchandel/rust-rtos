
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

impl Queue {
	fn create(length: uint, item_size: uint, queue_type: u8) -> Queue {
		new_queue: Queue;
		size: uint;

		/* Create the list of pointers to queue items.  The queue is one byte
		longer than asked for to make wrap checking easier/faster. */
		size = length * item_size + 1;

		// if length > 0
		new_queue = Queue {
			head: [0u8, ..size], // TODO box
			tail: &head[head.len() - 1],
			write_to: head,

			length: length,
			item_size: item_size
		};

		return new_queue
	}
}
