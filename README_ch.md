| language                        | 
|:--------------------------------|
| 简体中文                            |
| <a href='https://github.com/einQimiaozi/RNesEmu/blob/master/README.md'>English</a> |

RNesEmu是基于rust语言开发的NES模拟器。

目前，该项目正在开发中。

## 项目结构

cpu.rs:
- 模拟6502 CPU，包括所有常用指令
- 目前正在开发中。

addressing_modes.rs:
- 包括6502支持的15种寻址模式。

ops_codes.rs:
- 它包含与所有指令对应的十六进制操作码、操作数和其他信息。

目前，我是唯一一个开发该项目的人，总体进度缓慢。如果你对硬件模拟或rust语言感兴趣，欢迎你加入这个项目！

## 开发日志

2022.04.29：项目创建了！！

2022.04.30：增加了几个关于CPU的指令，完善了与栈操作相关的代码
