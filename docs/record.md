# References
- 驱动文件：drivers/tty/serial/amba-pl011.c
- 设备树：arch/arm/boot/dts/broadcom/bcm283x.dtsi
- [rust_uart8250](https://github.com/ZR233/linux-rust/blob/dev/samples/rust/rust_uart8250.rs)

# 错误记录
Failed spawning proc-macro server for workspace `/home/leap/private/project/uart_driver/rust-project.json`: no sysroot

树莓派连接笔记本热点，无法从 WSL2 中通过 ssh 连接到树莓派

# 流程
port
console
probe remove

# Env
```sh
# Requires dependent libraries
sudo apt install libelf-dev libgmp-dev libmpc-dev bc flex bison u-boot-tools
sudo apt install llvm-14 lld-14 clang-14

# Requires Rust dependent libs
rustup override set $(scripts/min-tool-version.sh rustc)
rustup component add rust-src
cargo install --locked --version $(scripts/min-tool-version.sh bindgen) bindgen-cli
make ARCH=arm64  O=build_4b LLVM=1 rustavailable # Rust is available!

make ARCH=arm64 O=build_4b LLVM=1 bcm2711_rust_defconfig
make ARCH=arm64 O=build_4b LLVM=1 -j$(nproc)

# 修改.config
make ARCH=arm64 O=build_4b LLVM=1 menuconfig

# 删除编译所生成的文件和配置文件
make ARCH=arm64 O=build_4b LLVM=1 mrproper

# rust-analyzer
make ARCH=arm64 O=build_4b LLVM=1 rust-analyzer

make -C ../linux_raspberrypi M=$PWD ARCH=arm64 O=build_4b rust-analyzer
```