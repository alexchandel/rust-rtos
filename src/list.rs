type TickType_t = u32
static TickType_MaxDelay: TickType_t = 0xffffffff

type TaskFunction_t = (*int) -> ()

type StackType_t = u32

static portNUM_CONFIGURABLE_REGIONS: int = 10

static configMAX_TASK_NAME_LEN: int = 16

mod list
{
	//v Definition of the only type of object that a list can contain.
	struct ListItem {
	    value: TickType_t,		//< The value being listed.  In most cases this is used to sort the list in descending order.
	    next: &ListItem,		//< Pointer to the next ListItem_t in the list.
	    previous: &ListItem,	//< Pointer to the previous ListItem_t in the list.
	    owner: &TaskControlBlock,	//< Pointer to the object (normally a TCB) that contains the list item.
	    						//  There is therefore a two way link between the object containing the list item and the list item itself.
	    container: &List		//< Pointer to the list in which this list item is placed (if any).
	}

	//v Definition of the type of queue used by the scheduler.
	struct MiniListItem {
	    value: TickType_t,
	    next: &ListItem,
	    previous: &ListItem
	}

	//v Definition of the type of queue used by the scheduler.
	struct List {
	    number_of_items: uint,
	    index: &ListItem,		//< Used to walk through the list.  Points to the last item returned by a call to listGET_OWNER_OF_NEXT_ENTRY ().
	    list_end: MiniListItem	//< List item that contains the maximum possible item value meaning it is always at the end of the list and is therefore used as a marker.
	}

	impl List {
		pub fn init(&self) {
			// The list structure contains a list item which is used to mark the
			// end of the list.  To initialise the list the list end is inserted
			// as the only list entry.
			self.index = &self.list_end

			// The list end value is the highest possible value in the list to
			// ensure it remains at the end of the list.
			self.list_end.value = TickType_MaxDelay

			// The list_end next and previous pointers point to itself so we know
			// when the list is empty.
			self.list_end.next = &self.list_end
			self.list_end.previous = &self.list_end

			self.number_of_items = 0;
		}

		pub fn insert_end(&self, new_item: &ListItem) {
			let old_index = self.index
			/* Insert a new list item into pxList, but rather than sort the list,
			   makes the new list item the last item to be removed by a call to
			   listGET_OWNER_OF_NEXT_ENTRY(). */
			new_item.next = old_index
			new_item.previous = old_index.previous
			old_index.previous.next = new_item
			old_index.previous = new_item

			new_item.container = self

			self.number_of_items++;
		}

		//v Insert the new list item into the list, sorted in value order.
		pub fn insert(&self, new_item: &ListItem) {
			ListItem_t &iterator =
				if new_item.value == TickType_MaxDelay {
					self.list_end.previous
				} else {
					/* If you find your application is crashing here then likely causes are
					   1) Stack overflow
					   2) Incorrect interrupt priority assignment
					   3) Calling an API function from within a critical section
					   4) Using a queue or semaphore before it has been initialised */
					sub_iterator = & self.list_end
					while sub_iterator.next.value <= new_item.value {
						sub_iterator = sub_iterator.next
					}
					sub_iterator
				}
			new_item.next = iterator.next
			new_item.next.previous = new_item
			new_item.previous = iterator
			iterator.next = new_item

			new_item.container = self

			self.number_of_items++;
		}

		//v Advance index, wrapping around if at end, and return owner
		pub fn owner_of_next(&self) -> TaskControlBlock {
			self.index = self.index.next
			if self.index == &(self.index.list_end) {
				self.index = self.index.next
			}
			return self.index.owner
		}

		pub fn owner_of_head(&self) -> TaskControlBlock {
			return self.list_end.next.owner
		}

		pub fn contains(&self, item: &ListItem) -> bool {
			return self == item.container
		}
	}

	impl ListItem {
		pub fn remove(&self) -> uint {
			self.next.previous = self.previous
			self.previous.next = self.next
			// Make sure the index is left pointing to a valid item.
			if (self.container.index == self) {
				self.container.index = self.previous
			} else {
				mtCOVERAGE_TEST_MARKER();
			}

			let new_num = self.container.number_of_items--
			self.container = NULL // ERROR
			new_num
		}
	}
}

mod queue
{
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
}

mod task {
	type TaskHandle_t = *u8

	enum TaskState {
		Running = 0,	//< A task is querying the state of itself, so must be running.
		Ready,			//< The task being queried is in a read or pending ready list.
		Blocked,		//< The task being queried is in the Blocked state.
		Suspended,		//< The task being queried is in the Suspended state, or is in the Blocked state with an infinite time out.
		Deleted			//< The task being queried has been deleted, but its TCB has not yet been freed.
	}

	struct TimeOut {
	    overflow_count: int,
	    time_on_entering: TickType_t
	}

	struct MemoryRegion {
		base_address: *u8,
		length: u32,			//< in bytes
		params: u32
	}

	struct TaskParameters {
		task_code: TaskFunction_t,
		name: &str,
		stack_depth: u16,
		params: &u8,
		priority: uint,
		stack_buffer: &StackType_t,
		regions: [MemoryRegion, ..portNUM_CONFIGURABLE_REGIONS]
	}

	struct TaskStatus {
		handle: TaskHandle_t,		//< The handle of the task to which the rest of the information in the structure relates.
		task_name: &str,			//< A pointer to the task's name.  This value will be invalid if the task was deleted since the structure was populated!
		task_number: uint,			//< A number unique to the task.
		current_state: TaskState,	//< The state in which the task existed when the structure was populated.
		current_priority: uint,		//< The priority at which the task was running (may be inherited) when the structure was populated.
		base_priority: uint,		//< The priority to which the task will return if the task's current priority
									//  has been inherited to avoid unbounded priority inversion when obtaining a mutex.
									//  Only valid if configUSE_MUTEXES is defined as 1 in FreeRTOSConfig.h.
		runtime_counter: u32,		//< The total run time allocated to the task so far, as defined by the run time stats clock.
									//  Only valid when configGENERATE_RUN_TIME_STATS is defined as 1 in FreeRTOSConfig.h
		stack_highwater_mark: u16	//< The minimum amount of stack space that has remained for the task since the task was created.
									//  The closer this value is to zero the closer the task has come to overflowing its stack.
	}

	enum SleepModeStatus {
		AbortSleep = 0,	//< A task has been made ready or a context switch pended since portSUPPORESS_TICKS_AND_SLEEP() was called - abort entering a sleep mode.
		StandardSleep,	//< Enter a sleep mode that will not last any longer than the expected idle time.
		NoTasksWaitingTimeout	//< No tasks are waiting for a timeout so it is safe to enter a sleep mode that can only be exited by an external interrupt.
	}

	/**
	 * Task control block.  A task control block (TCB) is allocated for each task,
	 * and stores task state information, including a pointer to the task's context
	 * (the task's run time environment, including register values)
	 */
	struct TaskControlBlock {
		top_of_stack: &StackType_t,	//< Points to the location of the last item placed on the tasks stack.
									//  THIS MUST BE THE FIRST MEMBER OF THE TCB STRUCT.

		// ERROR ifdef portUSING_MPU_WRAPPERS

		generic: ListItem,	//< The list that the state list item of a task is reference from denotes the state of that task (Ready, Blocked, Suspended ).
		event: ListItem,	//< Used to reference a task from an event list.
		priority: uint,		//< The priority of the task.  0 is the lowest priority.
		stack: &StackType_t,	//< Points to the start of the stack.

		task_name: u8[configMAX_TASK_NAME_LEN],	//< Descriptive name given to the task when created.  Facilitates debugging only.

		// ERROR if portSTACK_GROWTH > 0
		end_of_stack: StackType_t,	//< Points to the end of the stack on architectures where the stack grows up from low memory.

		// ERROR if portCRITICAL_NESTING_IN_TCB == 1
		critical_nesting: uint,	//< Holds the critical section nesting depth for ports that do not maintain their own count in the port layer.

		// ERROR if configUSE_TRACE_FACILITY == 1
		tcb_number: uint, //< Stores a number that increments each time a TCB is created.  It allows debuggers to determine when a task has been deleted and then recreated.
		task_number: uint, //< Stores a number specifically for use by third party trace code.

		// ERROR if configUSE_MUTEXES == 1
		base_priority: uint,	//< The priority last assigned to the task - used by the priority inheritance mechanism.

		// ERROR if configUSE_APPLICATION_TASK_TAG == 1
		task_tag: TaskHookFunction_t,	//<

		// ERROR if configGENERATE_RUN_TIME_STATS == 1
		runtime_counter: u32,	//< Stores the amount of time the task has spent in the Running state.

		// ERROR if configUSE_NEWLIB_REENTRANT == 1
		// Allocate a Newlib reent structure
		newlib_reent: Reent
	}

	static current_tcb: volatile PRIVILEGED_DATA &TaskControlBlock

	pub fn create_generic(code: TaskFunction_t, name: &str, stack_depth: u16, params, priority: uint) {
		configASSERT(code)
		configASSERT(uxPriority & ~portPRIVILEGE_BIT < configMAX_PRIORITIES)

	}

	fn allocate_tcb_and_stack(stack_depth: u16, stack_buffer: &StackType_t) -> TaskControlBlock {

	}
}









