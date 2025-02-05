# lego_arch
lego标准接口组件，与架构相关

## 简介

涉及与架构相关的代码，可以被分为三种功能类型的操作，其一，需要按位操作寄存器；其二，在代码中嵌入汇编指令；其三，处理trap逻辑。

基于以上三点，该核心组件的设计如下：

- 提供了便于按位操作寄存器的方法
- 将一些特殊指令包装为Rust函数
- 定义一系列的与架构trap相关的逻辑和接口



## 架构支持

<table>
    <tr>
        <th colspan="2">架构/features</th> 
        <th>寄存器操作</th>
        <th>特殊指令封装</th>
        <th>trap处理</th>
        <tr><td rowspan = "2">riscv</td> <td>rv32</td><td>todo</td><td>todo</td><td>todo</td></tr>
        <tr><td>rv64</td> <td>rv标准csr</td><td>rv标准指令</td><td>todo</td></tr>
        <tr><td rowspan = "2">arm</td> <td>aarch32</td><td>todo</td><td>todo</td><td>todo</td></tr>
        <tr><td>aarch64</td> <td>todo</td><td>todo</td><td>todo</td></tr>
    </tr>
</table>
## 使用

参考`riscv` `arm` `x86`下的README.md

## 贡献

如果您对lego_os感兴趣，欢迎贡献代码。

`email：18304935041@163.com`

`QQ：2925212608`