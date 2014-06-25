
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

}type TaskHandle_t = *u8

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
