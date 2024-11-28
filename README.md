# Rust Geodesy on ESP32
The Rust Geodesy library (RG) may not be intended as a direct replacement for the PROJ library, as its developers emphasize, but I believe it is the best choice for projects using the Rust language. Integrating PROJ into a Rust project can be cumbersome due to the need to compile its C/C++ source code, whereas RG is written in pure Rust, offering seamless compatibility and easier integration.


Using Rust Instead of C or MicroPython on ESP32
In projects that require interfacing with RTK GNSS devices, there may be a need to convert UTM or TM grid coordinates into geographic coordinates. Choosing Rust over C can help reduce memory usage, while opting for Rust instead of MicroPython can improve the processing speed, making it a better choice for microcontroller operations in such applications.

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
Prepare a USB cable and connect it to the ESP32. Ensure that WSL2 recognizes the cable as a serial device. 
![ESP32](https://github.com/pbroboto/rust-geodesy-esp32/blob/main/esp32_usb_cable.jpg)

Ready to flash to ESP32.
```console
pbrobo@pbrworkstation:~/esp/rust-esp/rust-geodesy-esp32$cargo run
```
Finally, if successful, here is the result:
```console
pbrobo@pbrworkstation:~/esp/rust-esp/rust-geodesy-esp32$cargo run
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.38s
     Running `espflash flash --monitor target/xtensa-esp32-espidf/debug/rust-geodesy2`
[2024-10-20T10:30:54Z INFO ] 🚀 A new version of espflash is available: v3.2.0
[2024-10-20T10:30:54Z INFO ] Serial port: '/dev/ttyUSB0'
[2024-10-20T10:30:54Z INFO ] Connecting...
[2024-10-20T10:30:54Z INFO ] Using flash stub
Chip type:         esp32 (revision v3.1)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi, BT, Dual Core, 240MHz, Coding Scheme None
MAC address:       88:13:bf:07:b8:98
App/part. size:    850,720/4,128,768 bytes, 20.60%
[2024-10-20T10:30:55Z INFO ] Segment at address '0x1000' has not changed, skipping write
[2024-10-20T10:30:55Z INFO ] Segment at address '0x8000' has not changed, skipping write
[2024-10-20T10:30:56Z INFO ] Segment at address '0x10000' has not changed, skipping write
[2024-10-20T10:30:56Z INFO ] Flashing has completed!
Commands:
    CTRL+R    Reset chip
    CTRL+C    Exit

ets Jul 29 2019 12:21:46

rst:0x1 (POWERON_RESET),boot:0x13 (SPI_FAST_FLASH_BOOT)
configsip: 0, SPIWP:0xee
clk_drv:0x00,q_drv:0x00,d_drv:0x00,cs0_drv:0x00,hd_drv:0x00,wp_drv:0x00
mode:DIO, clock div:2
load:0x3fff0030,len:7104
load:0x40078000,len:15576
load:0x40080400,len:4
0x40080400 - _invalid_pc_placeholder
    at ??:??
ho 8 tail 4 room 4
load:0x40080404,len:3876
entry 0x4008064c
I (31) boot: ESP-IDF v5.1-beta1-378-gea5e0ff298-dirt 2nd stage bootloader
I (31) boot: compile time Jun  7 2023 07:48:23
I (33) boot: Multicore bootloader
I (37) boot: chip revision: v3.1
I (41) boot.esp32: SPI Speed      : 40MHz
I (46) boot.esp32: SPI Mode       : DIO
I (50) boot.esp32: SPI Flash Size : 4MB
I (55) boot: Enabling RNG early entropy source...
I (60) boot: Partition Table:
I (64) boot: ## Label            Usage          Type ST Offset   Length
I (71) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (79) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (86) boot:  2 factory          factory app      00 00 00010000 003f0000
I (94) boot: End of partition table
I (98) esp_image: segment 0: paddr=00010020 vaddr=3f400020 size=34d20h (216352) map
I (184) esp_image: segment 1: paddr=00044d48 vaddr=3ffb0000 size=02158h (  8536) load
I (188) esp_image: segment 2: paddr=00046ea8 vaddr=40080000 size=09170h ( 37232) load
I (206) esp_image: segment 3: paddr=00050020 vaddr=400d0020 size=8cda4h (576932) map
I (414) esp_image: segment 4: paddr=000dcdcc vaddr=40089170 size=02d28h ( 11560) load
I (425) boot: Loaded app from partition at offset 0x10000
I (426) boot: Disabling RNG early entropy source...
I (437) cpu_start: Multicore app
I (446) cpu_start: Pro cpu start user code
I (446) cpu_start: cpu freq: 160000000 Hz
I (446) cpu_start: Application information:
I (449) cpu_start: Project name:     libespidf
I (454) cpu_start: App version:      1
I (458) cpu_start: Compile time:     Oct 20 2024 16:35:45
I (464) cpu_start: ELF file SHA256:  000000000...
I (470) cpu_start: ESP-IDF:          v5.2.2
I (475) cpu_start: Min chip rev:     v0.0
I (479) cpu_start: Max chip rev:     v3.99
I (484) cpu_start: Chip rev:         v3.1
I (489) heap_init: Initializing. RAM available for dynamic allocation:
I (496) heap_init: At 3FFAE6E0 len 00001920 (6 KiB): DRAM
I (502) heap_init: At 3FFB2AC0 len 0002D540 (181 KiB): DRAM
I (508) heap_init: At 3FFE0440 len 00003AE0 (14 KiB): D/IRAM
I (515) heap_init: At 3FFE4350 len 0001BCB0 (111 KiB): D/IRAM
I (521) heap_init: At 4008BE98 len 00014168 (80 KiB): IRAM
I (529) spi_flash: detected chip: generic
I (532) spi_flash: flash io: dio
W (536) pcnt(legacy): legacy driver is deprecated, please migrate to `driver/pulse_cnt.h`
W (545) timer_group: legacy driver is deprecated, please migrate to `driver/gptimer.h`
I (554) main_task: Started on CPU0
I (558) main_task: Calling app_main()
I (564) rust_geodesy2: Hello, ESP32 Rust Geodesy!
I (723) rust_geodesy2: Test forward pipeline of RG:
I (724) rust_geodesy2:
From grid coordinates of WGS84 UTM zone 47:
I (725) rust_geodesy2: Point 1, coordinates: Coor3D([685063.5075, 1521137.2111, 0.0])
I (734) rust_geodesy2: Point 2, coordinates: Coor3D([681899.5402, 1685083.143, 0.0])
I (750) rust_geodesy2:
To grid coordinates of Thailand Indian1975 UTM Zone 47:
I (750) rust_geodesy2: Point 1, coordinates: Coor3D([685396.1990916887, 1520834.3178692292, 17.46010040061179])
I (761) rust_geodesy2: Point 2, coordinates: Coor3D([682232.3647343164, 1684780.1251680395, 12.757210301214014])
I (772) rust_geodesy2:

=========Round the trip back!==============
I (779) rust_geodesy2: Test inverse pipeline of RG:
I (785) rust_geodesy2:
From grid coordinates of Thailand Indian1975 UTM Zone 47:
I (793) rust_geodesy2: Point 1, coordinates: Coor3D([685396.1990916887, 1520834.3178692292, 17.46010040061179])
I (804) rust_geodesy2: Point 2, coordinates: Coor3D([682232.3647343164, 1684780.1251680395, 12.757210301214014])
I (821) rust_geodesy2:
To grid coordinates of WGS84 UTM zone 47:
I (822) rust_geodesy2: Point 1, coordinates: Coor3D([685063.5074999997, 1521137.2110999997, 0.0])
I (831) rust_geodesy2: Point 2, coordinates: Coor3D([681899.5401999967, 1685083.143, -1.000184868473265e-9])
I (848) main_task: Returned from app_main()
pbrobo@pbrworkstation:~/esp/rust-esp/rust-geodesy-esp32$
```
