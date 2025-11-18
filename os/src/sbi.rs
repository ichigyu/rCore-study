//! SBI call wrappers
// SBI的全称是Supervisor Binary Interface （特权层二进制接口）
// RISC-V定义了三种主要特权模式（无Hypervisor层）
// - M模式：最高权限。执行底层固件，可以访问所有硬件资源，如时钟、中断控制器和串口
// - S模式：内核权限。运行操作系统内核，需要M模式的帮助才能执行特权操作。
// - U模式：应用权限。运行用户程序。
// 内核（S-Mode）不能直接访问某些底层硬件或执行关机操作，必须通过SBI向M-Mode发出请求 - 相当于ARMv8 HVC
// U-Mode需要请求S-Mode的帮助时，需要通过Systen Calls
// 当存在Hypervisor层时，RISC-V的特权模式包括
// - VU-mode：虚拟用户层
// - VS-mode：虚拟内核层
// - HS-mode：监管模式
// - M-mode： 最高权限
use core::arch::asm;

const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_SHUTDOWN: usize = 8;
// 控制台字符输出 EID #0x01

/// general sbi call
#[inline(always)]
// 强烈建议编译器始终进行内联优化
// sbi_call是底层、频繁调用的短小函数，内联可以消除函数调用开销，提高性能
// EID不为#0x10时，在SBI v0.2规范中：
// - a6寄存器中FID被忽略
// - a1寄存器不返回任何值
// - SBI调用期间，除a0寄存器外所有寄存器都必须被调用者保存
// - - 从ecall指令看，a0需要保存错误号，因此a0寄存器中的值如果需要保留，需要在调用前入栈
// - - 除a0寄存器外所有寄存器对于调用者无意义，因此可以由被调用者在返回前恢复
// - a0寄存器中返回值是特定于SBI传统扩展的

fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        // 内联汇编是不安全操作，需要至于unsafe块中
        asm!(
            "li x16, 0",
            // 为什么要清空寄存器x16?
            // a6 - x16 寄存器用于标识FID
            // EID #0x10定义为基本扩展，当EID为#0x10时，FID有意义
            // FID  #0  sbi_get_sbi_spec_version
            // FID  #1  sbi_get_sbi_impl_id
            // FID  #2  sbi_get_sbi_impl_version
            // FID  #3  sbi_probe_extension
            // FID  #4  sbi_get_mvendorid
            // FID  #5  sbi_get_marchid
            // FID  #6  sbi_get_mimpid
            "ecall",
            // ecall指令是从低特权模式向高特权模式的提权操作，通过
            // 通过RISC-V的ecall指令，实现对SBI服务的调用
            // 服务ID放在 a7 - x17 寄存器中
            // 参数放在 a0-a2 - x10-x12 寄存器中
            // 返回值放在 a0 - x10 寄存器中，将其赋值给ret作为SBI调用返回
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,

        );
    }
    ret
    // SBI_SUCCESS	                 0	    顺利完成
    // SBI_ERR_FAILED	            -1	    失败
    // SBI_ERR_NOT_SUPPORTED	    -2	    不支持操作
    // SBI_ERR_INVALID_PARAM	    -3	    非法参数
    // SBI_ERR_DENIED	            -4	    拒绝
    // SBI_ERR_INVALID_ADDRESS	    -5	    非法地址
    // SBI_ERR_ALREADY_AVAILABLE	-6	    （资源）已可用
    // SBI_ERR_ALREADY_STARTED	    -7	    （操作）已启动
    // SBI_ERR_ALREADY_STOPPED	    -8	    （操作）已停止
    // SBI_ERR_NO_SHMEM	            -9	    共享内存不可用
}

/// use sbi call to putchar in console (qemu uart handler)
pub fn console_putchar(c: usize) {
    // long sbi_console_putchar(int ch)，调用成功时返回0
    // 否则返回实现特定的负错误代码
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

// /// use sbi call to shutdown the kernel
pub fn shutdown() -> ! {
    // void sbi_shutdown(void)
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("System shutdown!");

}
