# 我做了哪些工作

- 修改 dts 并编译为 dts 以及设计 cpu.rs 来解析设备树
- 增加 SbiMessage `HSM(HSMFunction)`
- 增加多核启动的内容
  - 使用静态变量 `INITED_VCPUS` 与 `HS_VM` 
  - 使用 *spin_table* 的方式启动多核
  - 在 arceos 多核启动的最后一个流程跳转到虚拟机的多核启动
- 增加 SbiMessage `IPI(IPIFunction)`


# 详细介绍

## step1——设备树

设备树（Device Tree）的更改并编译没有什么困难的地方，不再赘述。但看群里似乎可以通过更改 Makefile 的方式来进行自动的修改，但还没有时间做这里的优化。

而之所以想到要用 cpu.rs 来做设备树的解析工作是参考了 [salus](https://github.com/rivosinc/salus) 的实现。所以这里我也用 fdt crate 做了设备树的解析，而 salus 则是使用了自己定义的 crate。

这里分析一下设备树的结构

```
- fdt
    - node
        - roperty
        - node
```

从最顶端的 fdt 获取节点可以使用 `pub fn find_node(&self, path: &str) -> Option<FdtNode<'_, 'a>>` 或者 `pub fn find_all_nodes(&self, path: &'a str) -> impl Iterator<Item = FdtNode<'_, 'a>>` 获得 FdtNode 结构体，而想要在上述结构体中获得 node 则只能使用 `pub fn children(self) -> impl Iterator<Item = FdtNode<'b, 'a>>`

## step——HSM

在做完上述修改之后，运行，得到下面的报错：

```shell
[  0.782353 0 hypercraft::arch::sbi:80] args: [1, 2418020454, 2533065584, 0, 0, 0, 0, 4739917]
[  0.797255 0 hypercraft::arch::sbi:81] args[7]: 0x48534d
[  0.799761 0 hypercraft::arch::sbi:82] EID_RFENCE: 0x52464e43
[  0.802749 0 axruntime::lang_items:5] panicked at 'explicit panic', /home/xuzx/arceos/crates/hypercraft/src/arch/riscv/vm.rs:96:25
```

开始看到这个错误毫无思路，虽然看到了是由于 SbiMessage 出现的问题，但我没有想到需要增加新的扩展进去，看源码，找到了是 **crates/hypercraft/src/arch/riscv/vcpu.rs** 中执行 `run()` 之后会跳转到相应的 SbiMessage 处，而这之前有一段汇编代码，我认为是这里出了问题，想要调试一下，无果，因为我也不知道寄存器这么变化是因为什么。在卡了一段时间之后我搜索了 `EID_RFENCE: 0x52464e43` 这个错误，然后发现是不支持 HSM，增加信息之后仍然报错：

```shell
[  0.347916 0 axruntime::lang_items:5] panicked at 'not yet implemented', /home/xuzx/arceos/crates/hypercraft/src/arch/riscv/vm.rs:119:34
```

这个错误比较好定位，因为我没有实现相应的 SbiMessage 的处理函数。但写出了第一版的处理函数之后程序仍然运行报错：

```shell
[  0.297477 1 axruntime::lang_items:5] panicked at 'Unhandled trap: Exception(InstructionGuestPageFault), sepc: 0x0, stval: 0x0', /home/xuzx/arceos/crates/hypercraft/src/arch/riscv/vcpu.rs:363:17
```

很明显是没有找到正确的入口函数，但我仔细检查了之后仍然没有发现错误，参考了 刘金成 的实现，仍然需要排查第一版的实现哪里出现了问题。

## step3——多核逻辑

其实做完了上面的工作，就有点进行不下去了。因为根据代码中的注释以及 salus 的实现过程，应该尝试使用汇编来做整个工作。我尝试在 **crates/hypercraft/src/arch/riscv/smp.rs** 中增加 PerCpu 的一个方法 `start_secondary_cpus()` 然后再进行类似 salus 以及 arceos 的 `start_scondary_cpu()` 的工作，但尝试写了汇编之后程序只能正常运行，但不能查看出 cpu 的信息。而我现在的设想则是，如果想要完成上述工作，是不是要把前面的 cpu.rs 文件放到 apps 下面，这样可以更好的区分硬件和 Rust 的执行过程。由 cpu.rs 执行硬件相关的内容然后进入 Rust 世界再到 hypercraft 中来操作，但由于汇编不太会写，所以还没有实现。

后来在讨论会上提到了使用静态变量的方法来实现这件事情我尝试那么做之后，发现的确可以运行。主要是下面的过程：

- 主核首先初始化
- 从核初始化
- 从核初始化完成后主核启动(vm.run)

并且由于新增了处理器的状态，所以在执行的过程中也需要检测相应的状态

## step4——IPI

在完成上述的多核逻辑的增加之后处理核间中断的内容也是由报错信息提供的，但对于核间中断的内容还需要更多的了解

## step-支持更多的核

由于上述的过程中只能支持两个核心的运行，当多余两个核心的时候程序会陷入死循环，最后定位到的错误是在 CPU 状态的检测中会出现死循环，但明明设置了 CPU 为可执行状态，这里百思不得其解。

而这周解决这个问题的思路出了问题，我认为是多核启动的过程中没有完成同步工作，我找了很多多核启动流程的资料，但现在感觉这个应该更多的和 SBI 以及中断相关。
