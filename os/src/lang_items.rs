//! The panic handler

use crate::sbi::shutdown;
// crate指的是当前编译单元
use core::panic::PanicInfo;

#[panic_handler]
/// panic handler属性宏，告诉编译器，这个函数是程序发生panic时应该调用的函数
/// panic 是一种不可恢复状态，通常通常由断言失败、数组越界访问、或者显式调用panic!宏引起
/// 当程序发生 panic 时，Rust 的运行时系统会立即调用这个被标记的 panic 函数
fn panic(info: &PanicInfo) -> ! {
    // -> ! 表示永不返回类型，表示函数永远不会正常返回到调用者
    if let Some(location) = info.location() {
        // PanicInfo中包含了发生panic的
        println!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            // panic发生的文件名和行号
            info.message()
            // 获取panic的具体消息
        );
    } else {
        println!("[kernel] Panicked: {}", info.message());
    }
    shutdown()
    // 终止系统
}
