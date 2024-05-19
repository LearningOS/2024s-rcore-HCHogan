1. 行为: 
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.
引起了store exception.

[kernel] IllegalInstruction in application, kernel killed it.
在用户态尝试使用sret, 引起了Exception::IllegalInstruction

[kernel] IllegalInstruction in application, kernel killed it.
尝试在用户态csrr sstatus, sstatus,引起了IllegalInstruction

2. 

1. 第一个task第一次调度的时候a0是run_first_task中的_unused,其他task第一次调度的时候是run_next_task里面的current_task_cx_ptr, 后面是_alltraps调用trap_handler返回的cx(当前任务的TaskContext)
restore的场景 1. 第一次进入这个任务的用户态 2. trap返回
2. sstatus sepcs scratch, sstatus.spp设置特权级, sepc, 返回用户态的时候的地址, sscratch,用来暂存kernel/user sp

3. x2 是sp,现在指着kernel stack, x4是tp,没用
4. 运行csrrw sp, sscratch, sp后, sp: userstack, sscratch: kernel stack
5. sret, CPU 会将当前的特权级按照 sstatus 的 SPP 字段设置为 U
6. sp现在是kernelstack, sscratch现在是userstack
7. ecall

总结:
在taskmanager里面放了一个start_time和task_syscall_times,记录启动时间和调用次数

在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无 

此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

xv6

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
