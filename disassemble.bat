cargo build && arm-none-eabi-objdump --demangle --headers --no-show-raw-insn -M reg-names-std -d target/thumbv4t-none-eabi/debug/main >target/dump.s
