

## 1.  正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行  [三个 bad 测例 (ch2b_bad_*.rs)](https://github.com/LearningOS/rCore-Tutorial-Test-2025S/tree/master/src/bin)  ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。
Rustsbi 版本为:0.3.0-alpha.2
出现报错:
 [kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
`ch2b_bad_address.rs` 由于除0错误触发异常退出 `ch2b_bad_instructions.rs` 在用户态非法使用指令`sret`  `ch2b_bad_register.rs` 在用户态非法使用指令`csrr`。

## 2.深入理解 [trap.S](https://github.com/LearningOS/rCore-Camp-Code-2025S/blob/ch3/os/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:

### 1.  L40：刚进入  `__restore`  时，`sp`  代表了什么值。请指出  `__restore`  的两种使用情景。
刚进__restore时，a0为当前内核态task的TaskContext地址。 
使用场景一，通过在内核栈上构造用户态上下文和内核态上下文，切换到新task的内核栈上执行，跳转到__restore,启动一个新的的task； 使用场景二，作为用户态系统调用结束后的处理，从内核态返回用户态。

###  2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。
ld t0, 32*8(sp) # 内核栈 32*8(sp) 处存储了原 sstatus 寄存器的值, 将其读取到 t0
ld t1, 33*8(sp) # 内核栈 32*8(sp) 处存储了原 sepc 寄存器的值, 将其读取到 t1
ld t2, 2*8(sp) # 内核栈 32*8(sp) 处存储了原 sscratch 寄存器的值, 将其读取到 t2
csrw sstatus, t0 # 将 t0中原 sstatus 寄存器的值读取到 sstatus
csrw sepc, t1 # 将 t0中原 sepc 寄存器的值读取到 sepc
csrw sscratch, t2 # 将 t0中原 sscratch 寄存器的值读取到 sscratch

###  3. L50-L56：为何跳过了 x2 和 x4？


1.  跳过`x2`是因为`x2`对应的用户栈指针保存到了sscratch寄存器, 不需要从内核栈中进行恢复
2.  跳过`x4`是因为并没有使用它, 所以无需恢复

###  4. L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？
`sp`指向用户进程的栈, `sscratch`指向内核进程的栈.


### 5.  `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

sret，用户态陷入内核态时sepc寄存器保存了用户态陷入内核态时的指令地址, sret指令跳转到ra寄存器地址并切换为s态。
###  6.L13 csrrw指令后sp和sscratch中的值分别有什么意义？
内核进程的栈指针，用户进程的栈指针。
###  7. 从U态进入S态是哪一条指令发生的？
ecall 
# 补充说明
1.  在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2.  此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
       
http://riscvbook.com/chinese/RISC-V-Reader-Chinese-v2p1.pdf
[https://git.ustc.edu.cn/gaoway/ustc_ca2021_lab/-/tree/master/References]
3.  我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
    
4.  我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

