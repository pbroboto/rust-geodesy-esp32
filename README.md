# Rust Geodesy on ESP32
The Rust Geodesy library (RG) may not be intended as a direct replacement for the PROJ library, as its developers emphasize, but I believe it is the best choice for projects using the Rust language. Integrating PROJ into a Rust project can be cumbersome due to the need to compile its C/C++ source code, whereas RG is written in pure Rust, offering seamless compatibility and easier integration.


Using Rust Instead of C or MicroPython on ESP32
In projects that require interfacing with RTK GNSS devices, there may be a need to convert UTM or TM grid coordinates into geographic coordinates. Choosing Rust over C can help reduce memory usage, while opting for Rust instead of MicroPython can improve the processing speed, making it a better choice for microcontroller operations in such applications.Using Rust instead of C or MicroPython on ESP32: In projects that need to interface with RTK GNSS devices, there may be a need to convert UTM or TM grid coordinates to geographic coordinates. Using Rust instead of C is for memory reduction reasons, while using Rust instead of MicroPython may be for speed reasons for microcontroller operation, which is another option.
