# riscv 架构支持

支持riscv架构的features如下

- `rv_dbg`：riscv Debug模式CSR寄存器操作支持
- `rv_mch`：riscv Machine模式CSR寄存器操作及特权指令支持
- `rv_spv`：riscv Debug模式CSR寄存器操作及特权指令支持
- `rv_hpv`：riscv Hpervisor 虚拟化CSR寄存器操作支持
- `rv_virt：riscv 虚拟化CSR寄存器操作支持`
- `rv_user`:riscv User模式CSR寄存器操作及指令支持
- `rv_trap`:riscv trap处理相关逻辑和接口

使用方法：

```toml
[dependencies]
lego_arch = { git = "https://github.com/lego-os/lego_arch.git", features = [
    "rv_mch",
], branch = "main" }
```

按需引入features，并将构建目标设置为：

```
riscv32i-unknown-none-elf
riscv32im-unknown-none-elf
riscv32imac-unknown-none-elf
riscv32imafc-unknown-none-elf
riscv32imc-unknown-none-elf
riscv64gc-unknown-linux-gnu
riscv64gc-unknown-linux-musl
riscv64gc-unknown-none-elf
riscv64imac-unknown-none-elf
```

目前，支持riscv64架构，需要注意调整build target

## 参考资料

- [Riscv isa specification](https://github.com/riscv/riscv-isa-manual)
- [Riscv plic](https://github.com/riscv/riscv-plic-spec.git)
- [Riscv clic](https://github.com/riscv/riscv-fast-interrupt)