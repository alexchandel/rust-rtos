#![crate_id = "rtos#0.0.0"]
#![license = "GPLv3"]
#![crate_type = "lib"]
#![no_std]
#![no_main]

type TickType_t = u32
static TickType_MaxDelay: TickType_t = 0xffffffff

type TaskFunction_t = (*int) -> ()

type StackType_t = u32

static portNUM_CONFIGURABLE_REGIONS: int = 10

static configMAX_TASK_NAME_LEN: int = 16

