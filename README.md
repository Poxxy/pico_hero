# Pico Hero is a Text Display System

## Requirements
1. RP2040 Microcontroller
2. OLED
3. PIR Motion Sensor
4. Wires

## How To
1. Clone this repo and use `cargo run --release` to load code onto pico
2. This program uses I2C to communicate with an OLED using GPIO17 and GPIO16 for SCL and SDA
3. Make sure to connect the power and ground correctly
4. A switch pin can be setup by connecting a button to GPIO0
5. Motion sensor triggers signal to GPIO1
6. Alarm gets triggered and sends power to GPIO2

## Notes

* Text can be changed with the write macro.
* Alarm functionality can be modified to be longer/shorter or faster/slower.
