# blinky
Assignment 0 from the [CS140 course](https://cs140e.sergio.bz/assignments/0-blinky/).

I have removed the `xargo` dependency and used `cargo xbuild` instead, using the linker, assembly, and make files from the phenominal [rust-raspi3-OS-tutorials repo from rust-embedded](https://github.com/rust-embedded/rust-raspi3-OS-tutorials) as a guide. The rust-raspi3-OS-tutorials repo also provides a convinent docker container that allows use of the qemu emulator, which is also used here.


## Phase 3: Shining C
Makes an LED blinky on a Raspberry Pi 3 using C code.

## Phase 4: Rusting Away
Makes an LED blinky on a Raspberry Pi 3 using Rust code.

### Run
To compile:
```
make
```

Grab the `kernel8.img` file along with the `bootcode.bin`, `start.elf`, `fixup.dat`, and `config.txt` (which all are found in the [raspberrypi repo](https://github.com/raspberrypi/firmware/tree/master/boot)) and copy them onto you microSD card. Plug it into your RPi3B+, as instructed on the [CS140 Assignment 0 website](https://cs140e.sergio.bz/assignments/0-blinky/), and watch the light blink!

To use the qemu emulator docker container from the rust-raspi3-OS-tutorials repo:

```
make qemu
```

