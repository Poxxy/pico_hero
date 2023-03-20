# Pico Hero is a Low Power Alarm System

## Requirements
1. Speaker of some kind
2. Wires
3. RP2040 Microcontroller
4. [Optional] LEDs

## How To
1. Clone this repo and use `cargo run --release` to load code onto pico
2. Wire GP16 as your alarm.
3. Wire GP14 as your seconds button.
4. Wire GP15 as your minutes button.
5. Plug into power.

## Notes

The alarm has a minimum timer of 10 seconds. You can add 5 seconds with each click of a button or for larger increases use the other button to add one minute.
