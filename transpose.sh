#!/bin/zsh
rm -f ./wokwiki.hex
rm -f ./wokwiki.elf

# cargo clean
cargo build --target avr-none --release

avr-objcopy -O ihex ./target/avr-none/release/wokwiki.elf ./target/avr-none/release/wokwiki.hex
# cp ./target/avr-none/debug/wokwiki.hex ./wokwiki.hex
# cp ./target/avr-none/debug/wokwiki.elf ./wokwiki.elf