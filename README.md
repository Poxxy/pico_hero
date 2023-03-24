# Pico Hero is a Text Display System

## Requirements
1. RP2040 Microcontroller
2. OLED
3. Wires

## How To
1. Clone this repo and use `cargo run --release` to load code onto pico
2. This program uses I2C to communicate with an OLED using GPIO17 and GPIO16 for SCL and SDA
3. Make sure to connect the power and ground correctly.

## Notes

Text can be changed with the write macro.
