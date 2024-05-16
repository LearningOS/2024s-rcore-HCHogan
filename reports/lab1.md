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
