cargo build
arm-none-eabi-objcopy --output-target binary target/thumbv4t-none-eabi/debug/main target/zygravan-debug.gba
arm-none-eabi-objdump --demangle --headers --no-show-raw-insn -M reg-names-std -d target/thumbv4t-none-eabi/debug/main >target/dump-debug.s

cargo build --release
arm-none-eabi-objcopy --output-target binary target/thumbv4t-none-eabi/release/main target/zygravan.gba
arm-none-eabi-objdump --demangle --headers --no-show-raw-insn -M reg-names-std -d target/thumbv4t-none-eabi/debug/main >target/dump.s
