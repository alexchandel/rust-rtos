RustRTOS
=========

RustRTOS is FreeRTOSv8.0.1 rewritten in Rust for the PIC32MX target, and eventually for the x86, ARM, and MSP430 targets as well.


## Developer Quick Start

1.	Install [Rust][rust-github]
2.	Download the [FreeRTOS][FreeRTOS-download] sources.
3.	Start reimplementing FreeRTOS in Rust!

[rust-github]: https://github.com/rust-lang/rust
[FreeRTOS-download]: http://www.freertos.org/a00104.html


## Notes

RustRTOS aims to replace FreeRTOS with no loss of functionality. It doesn't use the Rust standard library (`#![no_std]`), and there may be significant `unsafe` code. See the Rust guide for [Writing Safe Unsafe and Low-Level Code][rust-unsafe].

Current work is focused on translating the three minimum files, `list.c`, `tasks.c`, and `queue.c`, and their headers into Rust. Then, the PIC32MX-specific (or x86 or ARM) files `port.c` and `heap_4.c` will be translated. Once a bare bones project can be built, other features of FreeRTOS, including timers and coroutines, will be reimplemented.

[rust-unsafe]: http://doc.rust-lang.org/guide-unsafe.html


## Progress

#### list.c
* In Progress

#### tasks.c
* In Progress

#### queue.c
* Started

#### timers.c
* Not Started

#### croutine.c
* Not Started

#### event_groups.c
* Not Started

#### FreeRTOS.h / StackMacros.h
* Not Started

#### port.c / portmacro.h
* Not Started

#### heap_1.c / heap_2.c / heap_4.c
* Not Started

#### port ISR/ASM
* Unnecessary, in assembly


## License

RustRTOS is presently distributed under the full GPLv3. See LICENSE for details. If RustRTOS is ever production worthy, I may redistribute it under FreeRTOS's Modified GPL license.
