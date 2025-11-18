//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
// 将缺少文档注释的公共项视为编译错误
#![deny(warnings)]
// 将所有警告是为编译错误，确保代码质量
#![no_std]
// 不链接标准库
#![no_main]
// 不使用标准库定义的main函数作为入口点

use core::arch::global_asm;
use log::*;
use crate::sbi::shutdown;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    //  清空BSS段，BSS段存放了未初始化的全局和静态变量
    unsafe extern "C" {
        // 声明了两个外部符号，在链接脚本中定义
        fn sbss();
        fn ebss();
    }
    (sbss as *const () as usize..ebss as *const () as usize)
        .for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
    // 遍历BSS起始地址到结束地址范围，将每一个地址转换为可变指针*mut u8，强制写入0
}

/// the rust entry-point of os
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    unsafe extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
        // 启动栈的边界
    }
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as *const () as usize,
        etext as *const () as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as *const () as usize, erodata as *const () as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as *const () as usize, edata as *const () as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as *const () as usize, boot_stack_lower_bound as *const () as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as *const () as usize, ebss as *const () as usize);

    shutdown()
}
