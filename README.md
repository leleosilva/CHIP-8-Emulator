


# CHIP-8 Emulator

## Table of contents
  * [Concept](#concept)
  * [Prerequisites](#prerequisites)
    + [Rust](#rust)
    + [SDL2](#sdl2)
  * [Step by step](#step-by-step)
    + [CPU and instructions](#cpu-and-instructions)
    + [Testing](#testing)
      - [Test ROMs](#test-roms)
      - [Unit tests](#unit-tests)
    + [Drivers](#drivers)
  * [Installation](#installation)
  * [Usage](#usage)
    + [How to run](#how-to-run)
    + [Controls](#controls)
  * [To-Do List](#to-do-list)
  * [Author](#author)
  * [References](#references)
  * [Screenshots](#screenshots)

## Concept
**CHIP-8** is an interpreted programming language, developed by Joseph Weisbecker and initially used on the COSMAC VIP and Telmac 1800 8-bit microcomputers in the mid-1970s. CHIP-8 programs are run on a CHIP-8 virtual machine.

This project consists of an emulator for the original CHIP-8 interpreted language. It implements some of the fundamental concepts of Computer Architecture, such as dealing with the CPU structure, instruction cycle, I/O and system frequency.

## Prerequisites
### Rust
This emulator is coded using the [Rust](https://www.rust-lang.org/) programming language.

### SDL2
The [SDL2](https://github.com/Rust-SDL2/rust-sdl2) library is used to handle user input, audio, and graphics.

## Step by step
### CPU and instructions
CHIP-8 has 34 opcodes (35 actually, but one of them is ignored by modern interpreters), which are all two bytes long.

To make sure that all opcodes and the entire CPU structure work as intended, all the links at the [References](#references) section can - and should - be used. They have enough information to understand how the emulator should work from start to finish.

Overall, the most important thing to understand at the beginning is probably how the *fetch-decode-execute* cycle works.
The emulator runs in an infinite loop. In each loop, the emulator:

 1. **Fetchs** the instruction from memory at the current program counter (PC);
	- Because each instruction is two bytes long, 2 should be added to the program counter so it points to the next instruction address in memory. 
 2. **Decodes**  the instruction to find out what it has to do;
 3. **Executes**  the instruction and do what it tells the emulator to do.

### Testing
In emulation projects, testing can be quite the challenge. To make sure everything worked fine, test ROMs were used while the instructions were implemented, as well as unit tests for the CPU methods and each individual instruction.

#### Test ROMs
Test ROMs can be found at the [test_roms/](https://github.com/leleosilva/CHIP-8-Emulator/tree/sdl2-development/roms/test_roms) folder in this repository, and can be run normally by the emulator just like any other CHIP-8 ROM.

The [IBM logo ROM](https://github.com/leleosilva/CHIP-8-Emulator/blob/sdl2-development/roms/test_roms/IBM%20Logo.ch8) was the program that initially guided the implementation of the CPU instructions. All it does is display the IBM logo, and it only uses six instructions:
- ``00E0 (clear screen)``
- ``1NNN  (jump)``
- ``6XNN  (set register  `VX`)``
- ``7XNN  (add value to register  `VX`)``
- ``ANNN  (set index register I)``
- ``DXYN  (display/draw)``

<p align="center">
    <img src="https://github.com/leleosilva/CHIP-8-Emulator/blob/sdl2-development/imgs/ibm_logo.png" alt="Running the IBM Logo" width="700"> 
</p>

It allows us to quickly implement the required instructions and also test the instruction `DXYN`, the most difficult instruction to implement. Making sure this instruction works properly will allow us to use SDL2 to display the results on screen, which is required by the other test ROMs.

#### Unit tests
Unit tests were created for all CHIP-8 instructions and some CPU methods, in a total of 40 unit tests available at [cpu_test.rs](https://github.com/leleosilva/CHIP-8-Emulator/blob/sdl2-development/src/cpu_test.rs).
To run the tests, use
```
cargo test
```
All of them should pass accordingly.

### Drivers
To handle graphics, user input and audio, the files `display_driver.rs`, `keypad_driver.rs` and `audio_driver.rs` were created and can be found at [drivers/](https://github.com/leleosilva/CHIP-8-Emulator/tree/sdl2-development/src/drivers) in this repository. These are the only files that use SDL2 code in the project.

Even though they could be directly implemented in the `main.rs` file, dividing them into their own appropriate drivers makes it so they encapsulate the third-party code from the rest of the project.
Therefore, changing these files (for example, using a library other than SDL2) shouldn't affect the project at all.

## Installation
 - Clone and move to the repository by using the ``git clone`` and ``cd`` commands:
	 - If you want to clone the entire repository (all branches), you should clone it and move to the current branch by using
		```
		git clone --branch sdl2-development git@github.com:leleosilva/CHIP-8-Emulator.git
		cd CHIP-8-Emulator
		```
	- If you want to clone only the current branch (that uses SDL2), use
		```
		git clone --branch sdl2-development --single-branch git@github.com:leleosilva/CHIP-8-Emulator.git
		cd CHIP-8-Emulator
		```

- Build the project by entering the following command which uses Cargo, Rust’s build system and package manager:
	```
	cargo build --release
	```
By default, it will create a binary file in ``./target/release``.
This binary is completely self-contained, so it can be movd or copied to somewhere else on your computer.

## Usage
### How to run
From the ``CHIP-8-Emulator/`` directory, you can run the emulator with 
```
# Mac/Linux
./target/release/chip-8 <PATH TO ROM>

# Windows
.\target\release\chip-8 <PATH TO ROM>
```
If you moved or copied the binary file elsewhere in your computer, go to the directory that contains the binary and run the emulator with
```
# Mac/Linux
./chip-8 <PATH TO ROM>

# Windows
.\chip-8 <PATH TO ROM>
```

<p align="center" width="100%">
    <img src="https://github.com/leleosilva/CHIP-8-Emulator/blob/sdl2-development/imgs/run_error.png" alt="Running the emulator without defined ROM path"> 
</p>

<p align="center" width="100%">
    <img src="https://github.com/leleosilva/CHIP-8-Emulator/blob/sdl2-development/imgs/run_help.png" alt="Running the emulator with --help flag"> 
</p>

### Controls
CHIP-8 uses a 16-key hexadecimal keypad labeled `0` through `F`, arranged in a 4x4 grid, with the following layout:
<table align="center">
<tr><th>CHIP-8 keypad </th></tr>
<tr><td>

| 1 | 2 | 3 | C | 
|---|---|---|---|
| 4 | 5 | 6 | D |
| 7 | 8 | 9 | E |
| A | 0 | B | F |

</td></tr></table>

It’s customary to map the original keypad layout using the left side of the QWERTY keyboard. In this project, the keys used by the user have the following layout:
<table align="center">
<tr><th>User keyboard</th></tr>
<tr><td>
  
| 1 | 2 | 3 | 4 |
|---|---|---|---|
| Q | W | E | R |
| A | S | D | F |
| Z | X | C | V |
</td></tr></table>

## To-Do List
- Test compatibility with Windows systems;
- Add support to WebAssembly;
- Substitute SDL2 for `egui`;
- Create a proper debugger with:
	- A disassembler;
	- An option to pause/resume the emulator;
	- An option to choose new ROM files;
	- An option to reload the same game;
	- An option to change background and sprite colors.
  
## Author
Leonardo Cavalcante da Silva ([@leleosilva](https://github.com/leleosilva))

## References
- [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [How to write an emulator (CHIP-8 interpreter)](https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
- [Building a CHIP-8 emulator [C++]](https://austinmorlan.com/posts/chip8_emulator/)
- [An Introduction to Chip-8 Emulation using the Rust Programming Language](https://github.com/aquova/chip8-book)
- [Awesome CHIP-8](https://chip-8.github.io/links/)
- [CHIP-8 - Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
- [/r/EmuDev](https://www.reddit.com/r/EmuDev/)

## Screenshots
<p align="center">
    <img src="https://github.com/leleosilva/CHIP-8-Emulator/blob/sdl2-development/imgs/brix.png" alt="Playing BRIX" width="700"> 
</p>

<p align="center">
    <img src="https://github.com/leleosilva/CHIP-8-Emulator/blob/sdl2-development/imgs/kaleid.png" alt="Playing KALEID" width="700"> 
</p>

<p align="center">
    <img src="https://github.com/leleosilva/CHIP-8-Emulator/blob/sdl2-development/imgs/tetris.png" alt="Playing TETRIS" width="700"> 
</p>
