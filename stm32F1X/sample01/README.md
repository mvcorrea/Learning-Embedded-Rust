# Embedded Rust on the stm32f103

## to build
> cd sample01

! the executable will be created on "target/thumbv7m-none-eabi/release"
> cargo build --release

```shell
osx% cargo build --release
   Compiling typenum v1.12.0
   Compiling version_check v0.9.2
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v1.0.24
   Compiling unicode-xid v0.2.1
   Compiling cortex-m v0.6.3
   Compiling stable_deref_trait v1.2.0
   Compiling syn v1.0.48
   Compiling vcell v0.1.2
   Compiling bitfield v0.13.2
   Compiling cortex-m v0.5.10
   Compiling cortex-m-rt v0.6.13
   Compiling aligned v0.2.0
   Compiling stm32f1 v0.6.0
   Compiling r0 v0.2.2
   Compiling panic-halt v0.2.0
   Compiling semver v0.9.0
   Compiling generic-array v0.14.4
   Compiling volatile-register v0.2.0
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling quote v1.0.7
   Compiling generic-array v0.12.3
   Compiling generic-array v0.13.2
   Compiling as-slice v0.1.4
   Compiling aligned v0.3.4
   Compiling cortex-m-rt-macros v0.1.8
   Compiling stm32_sample v0.1.0 (/_RUST/Learning-Embedded-Rust/stm32F1X/sample01)
    Finished release [optimized + debuginfo] target(s) in 1m 56s
```


! then with the mcu connected to the debugger
> arm-none-eabi-gdb ./target/thumbv7m-none-eabi/release/stm32_sample

...
