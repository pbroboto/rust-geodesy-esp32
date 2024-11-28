# Rust Geodesy on ESP32
The Rust Geodesy library (RG) may not be intended as a direct replacement for the PROJ library, as its developers emphasize, but I believe it is the best choice for projects using the Rust language. Integrating PROJ into a Rust project can be cumbersome due to the need to compile its C/C++ source code, whereas RG is written in pure Rust, offering seamless compatibility and easier integration.


Using Rust Instead of C or MicroPython on ESP32
In projects that require interfacing with RTK GNSS devices, there may be a need to convert UTM or TM grid coordinates into geographic coordinates. Choosing Rust over C can help reduce memory usage, while opting for Rust instead of MicroPython can improve the processing speed, making it a better choice for microcontroller operations in such applications.Using Rust instead of C or MicroPython on ESP32: In projects that need to interface with RTK GNSS devices, there may be a need to convert UTM or TM grid coordinates to geographic coordinates. Using Rust instead of C is for memory reduction reasons, while using Rust instead of MicroPython may be for speed reasons for microcontroller operation, which is another option.

The installation process for WSL2 on Windows is quite detailed, so I won’t cover it here. You can follow the step-by-step instructions provided at this [link](https://www.instructables.com/ESP32-ESP32C3-Blink-Test-Rust-Development-in-Windo/)


Use Git to clone my repository.
```console
pbrobo@pbrworkstation:~/esp/rust-esp/$git clone https://github.com/pbroboto/rust-geodesy-esp32
pbrobo@pbrworkstation:~/esp/rust-esp/$cd rust-geodesy-esp32
```
Use cargo to build.
```console
pbrobo@pbrworkstation:~/esp/rust-esp/rust-geodesy-esp32$cargo +esp build
```
Ready to flash to ESP32.
![ESP32](https://github.com/pbroboto/rust-geodesy-esp32/blob/main/esp32_usb_cable.jpg)

