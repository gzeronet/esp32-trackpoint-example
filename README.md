# esp32-trackpoint-example



https://github.com/gzeronet/esp32-trackpoint-example/assets/4970608/0ec4569b-8dae-4a6d-956a-3c8cbf9ef297




Port trackpoint ps/2 to esp32 c6
 
[Here](https://github.com/gzeronet/stm32f4-trackpoint-mouse) is my stm32F4 version maybe someone interested in.

Just try to port esp32 and ready to do some POC with BLE trackpoint mouse, so have this example. 

Seems esp32 hal is much easier than stm32 one, and more powerful with wireless.

And no difficult on porting by rust esp-template :D

## Hardware

* esp32 board c6 or others
* trackpoint module
* breadboard

## Connect


ESP32C6 --- Trackpoint

* Pin 5 --- RST
* Pin 6 --- CLK
* Pin 7 --- DATA

** Don't forget connect power on Trackpoint module, I just made the mistake.

## Build & Run

cargo run --example tp


** check & replace keyword "esp32c6" in repo files if you use other board.

## TODO


Trackpoint stream mode with interrupt.
