#![allow(unused_variables, dead_code)]

use rand::{self, Rng};
use std::time;

// CHIP-8 can access 4KB (4096 bytes) of RAM
const MEMORY_SIZE: usize = 4096;

// The display should be 64 pixels wide and 32 pixels tall
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

// After loading, CHIP-8 programs start at address 0x200
const START_ADDRESS: u16 = 0x200;

/* The delay and sound timers decrement at a rate of 60Hz (60 times per second)
 * Therefore, (1 / 60) = 0.0166666667s = 16667μs */
const TIMER_RATE: u64 = 16667;

const CHIP8_FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

pub struct Cpu {

    // RAM, writable memory
    memory: [u8; MEMORY_SIZE],
    
    // A program counter (PC) which points at the current instruction in memory
    pc: u16,
    
    // General-purpose variable registers numbered 0 through F hexadecimal
    v: [u8; 16],
    
    // Index register I, which is generally used to store memory addresses
    i: u16,
    
    // A stack which is used to call subroutines/functions and return from them
    stack: [u16; 16],
    
    // Stack pointer which is used to point to the topmost level of the stack
    sp: usize,
    
    // Delay timer which is decremented at a rate of 60 Hz until it reaches 0
    delay_timer: u8,
    
    // Sound timer which gives off a beeping sound as long as it’s not 0
    sound_timer: u8,
    
    // A display that updates at 60 Hz and whose each pixel can be on or off
    display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    
    /* CHIP-8 uses a hexadecimal keypad that had 16 keys, labelled 0 through F,
     * and were arranged in a 4x4 grid */
    keypad: [bool; 16],

    // The period of time the CPU uses to finish a cycle
    tick_period: time::Instant,
    
}

impl Cpu {

    // Creating new instance of CPU
    pub fn new() -> Self {
        
        // Initializing memory with 0's and storing font data at 0x50 ~ 0x9F address interval
        let mut aux_memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
        aux_memory[0x50..(0x50 + CHIP8_FONT.len())].clone_from_slice(&CHIP8_FONT);

        Self {
            memory: aux_memory,
            pc: START_ADDRESS, // Program counter starts at the initial address
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            keypad: [false; 16], // Keys start as not pressed
            tick_period: time::Instant::now(), // Storing when the CPU cycle begins
        }
    }

    // Loading ROM data into memory, starting at the initial address
    pub fn load_rom_in_memory(&mut self, rom_data: &[u8]) {
        let initial_address = START_ADDRESS as usize;
        let final_address = START_ADDRESS as usize + rom_data.len();

        self.memory[initial_address..final_address].copy_from_slice(rom_data);
    }

    // Returns the display
    pub fn get_display(&self) -> &[bool] {
        &self.display
    }

    // Fetching the instruction from memory at the current PC
    fn fetch(&mut self) -> u16 {
        
        /* An instruction is two bytes. Therefore, two consecutive bytes
         * from memory are read and combined into one 2-bytes instruction */
        let op1 = self.memory[self.pc as usize];
        let op2 = self.memory[(self.pc + 1) as usize];
        
        /* To get the opcode, the first byte should be shifted to the left by 8 bits
         * and then combined with the second byte by an logical OR operation */
        let instruction_opcode = (op1 as u16) << 8 | op2 as u16;

        instruction_opcode
    }

    // Decoding the instruction and calling its individual execution method
    pub fn decode(&mut self, opcode: u16) {
        
        // The fourth nibble of the instruction (lowest 4 bits)
        let n = opcode & 0x000F;
        // The second byte of the instruction (lowest 8 bits)
        let nn = opcode & 0x00FF;
        // The second, third and fourth nibbles of the instruction (lowest 12 bits)
        let nnn = opcode & 0x0FFF;

        // The second nibble. Used as index for one of the 16 registers (Vx)
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let vx = self.v[x];

        // The third nibble. Used as index for one of the 16 registers (Vy)
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vy = self.v[y];
        
        println!("opcode: {:X}", opcode);

        println!("N: {:X}", n);
        println!("NN: {:X}", nn);
        println!("NNN: {:X}", nnn);
        
        println!("X: {:X}", x);
        println!("Y: {:X}", y);

        // Dividing opcode in nibbles to select the instruction to be executed based on them
        let op1 = ((opcode & 0xF000) >> 12) as u8;
        let op2 = ((opcode & 0x0F00) >> 8) as u8;
        let op3 = ((opcode & 0x00F0) >> 4) as u8;
        let op4 = (opcode & 0x000F) as u8;
        println!("op1: {:X}", op1);
        println!("op2: {:X}", op2);
        println!("op3: {:X}", op3);
        println!("op4: {:X}", op4);

        // Control flow of instructions
        match (op1, op2, op3, op4) {
            (0x0, 0x0, 0xE, 0x0) => self.instruction_00e0(),
            (0x0, 0x0, 0xE, 0xE) => self.instruction_00ee(),
            (0x1, _, _, _) => self.instruction_1nnn(nnn),
            (0x2, _, _, _) => self.instruction_2nnn(nnn),
            (0x3, _, _, _) => self.instruction_3xnn(x, nn),
            (0x4, _, _, _) => self.instruction_4xnn(x, nn),
            (0x5, _, _, 0x0) => self.instruction_5xy0(x, y),
            (0x6, _, _, _) => self.instruction_6xnn(x, nn),
            (0x7, _, _, _) => self.instruction_7xnn(x, nn),
            (0x8, _, _, 0x0) => self.instruction_8xy0(x, y),
            (0x8, _, _, 0x1) => self.instruction_8xy1(x, y),
            (0x8, _, _, 0x2) => self.instruction_8xy2(x, y),
            (0x8, _, _, 0x3) => self.instruction_8xy3(x, y),
            (0x8, _, _, 0x4) => self.instruction_8xy4(x, y),
            (0x8, _, _, 0x5) => self.instruction_8xy5(x, y),
            (0x8, _, _, 0x6) => self.instruction_8xy6(x),
            (0x8, _, _, 0x7) => self.instruction_8xy7(x, y),
            (0x8, _, _, 0xE) => self.instruction_8xye(x),
            (0x9, _, _, 0x0) => self.instruction_9xy0(x, y),
            (0xA, _, _, _) => self.instruction_annn(nnn),
            (0xB, _, _, _) => self.instruction_bnnn(nnn),
            (0xC, _, _, _) => self.instruction_cxnn(x, nn),
            (0xD, _, _, _) => self.instruction_dxyn(x, y, n),
            (0xE, _, 0x9, 0xE) => self.instruction_ex9e(x),
            (0xE, _, 0xA, 0x1) => self.instruction_exa1(x),
            (0xF, _, 0x0, 0x7) => self.instruction_fx07(x),
            (0xF, _, 0x0, 0xA) => self.instruction_fx0a(x),
            (0xF, _, 0x1, 0x5) => self.instruction_fx15(x),
            (0xF, _, 0x1, 0x8) => self.instruction_fx18(x),
            (0xF, _, 0x1, 0xE) => self.instruction_fx1e(x),
            (0xF, _, 0x2, 0x9) => self.instruction_fx29(x),
            (0xF, _, 0x3, 0x3) => self.instruction_fx33(x),
            (0xF, _, 0x5, 0x5) => self.instruction_fx55(x),
            (0xF, _, 0x6, 0x5) => self.instruction_fx65(x),
            _ => panic!("Unknown instruction {:#06X}", opcode),
        }

    }

    // Running the CPU cycle
    pub fn run(&mut self) {

        let opcode = self.fetch();

        // PC is incremented by 2 to be ready to fetch the next instruction 
        self.pc += 2;

        self.decode(opcode);

        /* If the time elapsed is greater or equal to the timer rate, the timers are decremented.
         * This ensures the timer rate is kept at 60Hz.  */
        if self.tick_period.elapsed() >= time::Duration::from_micros(TIMER_RATE) {
            
            self.update_timers();
            self.tick_period = time::Instant::now(); // Updating tick period after the cycle ends
        }
    }

    // Decrementing timers when they are greater than zero
    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            // PLAY SOUND HERE
            self.sound_timer -= 1;
        }
    }

    /* EXECUTION OF INDIVIDUAL INSTRUCTIONS */

    // 	Clears the display
    fn instruction_00e0(&mut self) {
        self.display = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
    }

    /* Returns from a subroutine, setting the PC to the address at the top of the stack
     * and then subtracting 1 from the stack pointer. */
    fn instruction_00ee(&mut self) {
        self.pc = self.stack[self.sp];
        self.sp -= 1;
    }

    // Jumps to address NNN
    fn instruction_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    /* Call subroutine at NNN, incrementing the stack pointer and then putting the current PC
     * on the top of the stack.
     * 
     * The PC is then set to NNN. */
    fn instruction_2nnn(&mut self, nnn: u16) {
        self.sp += 1;
        self.stack[self.sp] = self.pc;
        self.pc = nnn;
    }

    // Skips the next instruction if Vx equals NN
    fn instruction_3xnn(&mut self, x: usize, nn: u16) {
        if self.v[x] as u16 == nn {
            self.pc += 2;
        }
    }

    // Skips the next instruction if Vx does not equal NN
    fn instruction_4xnn(&mut self, x: usize, nn: u16) {
        if self.v[x] as u16 != nn {
            self.pc += 2;
        }
    }

    // Skips the next instruction if Vx equals Vy
    fn instruction_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    // Sets Vx to NN
    fn instruction_6xnn(&mut self, x:usize, nn: u16) {
        self.v[x] = nn as u8;
    }

    // Adds NN to Vx
    fn instruction_7xnn(&mut self, x:usize, nn: u16) {
        self.v[x] += nn as u8;
    }

    // Sets Vx to the value of Vy
    fn instruction_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    // Sets Vx to Vx OR Vy
    fn instruction_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[x] | self.v[y];
    }

    // Sets Vx to Vx AND Vy
    fn instruction_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[x] & self.v[y];
    }
    
    // Sets Vx to Vx XOR Vy
    fn instruction_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[x] ^ self.v[y];
    }

    // Adds Vy to Vx. VF is set to 1 when there's a carry, and to 0 when there is not
    fn instruction_8xy4(&mut self, x: usize, y: usize) {
        self.v[0xF] = if (u16::from(self.v[x]) + u16::from(self.v[y])) > u16::from(u8::MAX) {
            1
        } else {
            0
        };
        self.v[x] = self.v[x].wrapping_add(self.v[y]);
    }

    // Vy is subtracted from Vx. VF is set to 0 when there's a borrow, and 1 when there is not
    fn instruction_8xy5(&mut self, x: usize, y: usize) {
        self.v[0xF] = if self.v[x] > self.v[y] {
            1
        } else {
            0
        };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    }

    // Stores the least significant bit of Vx in VF and then shifts Vx to the right by 1
    fn instruction_8xy6(&mut self, x: usize) {
        self.v[0xF] = self.v[x] & 1; // Getting LSB
        self.v[x] >>= 1;
    }

    // Sets Vx to Vy minus Vx. VF is set to 0 when there's a borrow, and 1 when there is not
    fn instruction_8xy7(&mut self, x: usize, y: usize) {
        self.v[0xF] = if self.v[y] > self.v[x] {
            1
        } else {
            0
        };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
    }

    // Stores the most significant bit of Vx in VF and then shifts Vx to the left by 1
    fn instruction_8xye(&mut self, x: usize) {
        self.v[0xF] = (self.v[x] >> 7) & 1; // Getting MSB
        self.v[x] <<= 1;
    }

    // Skips the next instruction if Vx does not equal Vy
    fn instruction_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    // 	Sets I to the address NNN
    fn instruction_annn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    // Jumps to the address NNN plus V0
    fn instruction_bnnn(&mut self, nnn: u16) {
        self.pc = nnn + self.v[0] as u16;
    }

    // Sets Vx to the result of a bitwise AND operation on a random number from 0 to 255 and NN
    fn instruction_cxnn(&mut self, x: usize, nn: u16) {
        self.v[x] = (nn as u8) & rand::thread_rng().gen::<u8>();
    }

    /* Draws a sprite starting at coordinate (Vx, Vy) that has a width of 8 pixels and a height of N pixels.
     * 
     * Sprites are XORed onto the existing display.
     * 
     * Each row of 8 pixels is read as bit-coded starting from memory location I;
     * I value does not change after the execution of this instruction.
     * 
     * If the sprite is positioned so part of it is outside the coordinates of the display,
     * it wraps around to the opposite side of the display.
     * 
     * VF is set to 1 if any display pixels are flipped from set to unset when the sprite is drawn,
     * and to 0 if that does not happen. */
    fn instruction_dxyn(&mut self, x: usize, y: usize, n: u16) {

        let height = n as usize;

        // Initially, VF is set to 0
        self.v[0xF] = 0;

        for byte in 0..height {
            let y_coord = (self.v[y] as usize + byte) % DISPLAY_HEIGHT;

            // Accessing the current row of sprite pixels from RAM memory
            let pixels = self.memory[self.i as usize + byte];

            for bit in 0..8 {   
                let x_coord = (self.v[x] as usize + bit) % DISPLAY_WIDTH;

                /* Accessing specific pixel from the current row of sprite pixels
                 * (most significant to least significant bit) */
                let current_pixel = (pixels >> (7 - bit)) & 0x001;
                
                // Current sprite pixel is on
                if current_pixel == 1 {

                    // Getting index of current display pixel for the 1D display array
                    let index = (DISPLAY_WIDTH * y_coord) + x_coord;

                    /* If the sprite pixel and display pixel are both on, the display pixel will flip
                     * from set to unset and VF should be set to 1 */
                    if self.display[index] == true {
                        self.v[0xF] = 1;
                    }
                    self.display[index] ^= true; // XOR operation that flips the display pixel
                }
            }
        }
    }

    // Skips the next instruction if the key stored in Vx is pressed 
    fn instruction_ex9e(&mut self, x: usize) {
        if self.keypad[self.v[x] as usize] {
            self.pc += 2;
        }
    }

    // Skips the next instruction if the key stored in Vx is not pressed
    fn instruction_exa1(&mut self, x: usize) {
        if !self.keypad[self.v[x] as usize] {
            self.pc += 2;
        }
    }

    // 	Sets Vx to the value of the delay timer
    fn instruction_fx07(&mut self, x: usize) {

    }

    // A key press is awaited, and then stored in Vx
    fn instruction_fx0a(&mut self, x: usize) {

    }

    // Sets the delay timer to Vx
    fn instruction_fx15(&mut self, x: usize) {

    }

    // Sets the sound timer to Vx
    fn instruction_fx18(&mut self, x: usize) {

    }

    // Adds Vx to I
    fn instruction_fx1e(&mut self, x: usize) {

    }

    // Sets I to the location of the sprite for the character in Vx
    fn instruction_fx29(&mut self, x: usize) {

    }

    // Stores the binary-coded decimal representation of Vx in memory locations I, I+1, and I+2
    fn instruction_fx33(&mut self, x: usize) {

    }

    // Store registers V0 through Vx in memory starting at location I
    fn instruction_fx55(&mut self, x: usize) {

    }

    // Read registers V0 through Vx from memory starting at location I
    fn instruction_fx65(&mut self, x: usize) {
        
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_initial_state() {
        let cpu = Cpu::new();
        
        // Testing first line of font data in memory
        assert_eq!(cpu.memory[0x50..0x55], [0xF0, 0x90, 0x90, 0x90, 0xF0]);
        
        // Getting indexes related to the last line of font data in memory
        const FONT_INITIAL_IDX: usize = 0x50 + (CHIP8_FONT.len() - 5);
        const FONT_FINAL_IDX: usize = 0x50 + CHIP8_FONT.len();

        // Testing last line of font data in memory
        assert_eq!(
            cpu.memory[FONT_INITIAL_IDX..FONT_FINAL_IDX],
            [0xF0, 0x80, 0xF0, 0x80, 0x80]
        );

        // Testing if memory after font data is correct
        assert_eq!(
            cpu.memory[(0x9F + 1)..],
            [0; (MEMORY_SIZE - FONT_FINAL_IDX)]
        );
        assert_eq!(cpu.pc, 0x200);
        assert_eq!(cpu.v, [0; 16]);
        assert_eq!(cpu.i, 0);
        assert_eq!(cpu.stack, [0; 16]);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.delay_timer, 0);
        assert_eq!(cpu.sound_timer, 0);
        assert_eq!(cpu.display, [false; DISPLAY_WIDTH * DISPLAY_HEIGHT]);
        assert_eq!(cpu.keypad, [false; 16]);
    }

    #[test]
    fn test_rom_loading() {
        let mut cpu = Cpu::new();
        cpu.load_rom_in_memory(&vec![1, 2, 3, 4]);
        
        assert_eq!(cpu.memory[0x200], 1);
        assert_eq!(cpu.memory[0x201], 2);
        assert_eq!(cpu.memory[0x202], 3);
        assert_eq!(cpu.memory[0x203], 4);
        assert_eq!(cpu.memory[0x205], 0); // 0x205 address should stay unchanged
    }

    #[test]
    fn test_get_display() {
        let cpu = Cpu::new();
        assert_eq!(cpu.get_display(), [false; DISPLAY_WIDTH * DISPLAY_HEIGHT]);
    }

    #[test]
    fn test_fetch() {
        let mut cpu = Cpu::new();
        cpu.load_rom_in_memory(&vec![0x24, 0x7C, 0xFF, 0x1]);
        
        cpu.pc = 0x200; // ROM is loaded starting on address 0x200
        assert_eq!(cpu.fetch(), 0x247C);

        cpu.pc = 0x201;
        assert_eq!(cpu.fetch(), 0x7CFF);

        cpu.pc = 0x202;
        assert_eq!(cpu.fetch(), 0xFF01);
    }

    #[test]
    #[should_panic]
    fn test_decode_invalid_instruction() {
        let mut cpu = Cpu::new();
        cpu.decode(0x00FF); // Instruction 00FF is invalid
    }

    #[test]
    fn test_update_timers() {
        let mut cpu = Cpu::new();
        cpu.delay_timer = 3;
        cpu.sound_timer = 2;

        cpu.update_timers();
        assert_eq!(cpu.delay_timer, 2); assert_eq!(cpu.sound_timer, 1);

        cpu.update_timers();
        assert_eq!(cpu.delay_timer, 1); assert_eq!(cpu.sound_timer, 0);

        cpu.update_timers();
        assert_eq!(cpu.delay_timer, 0); assert_eq!(cpu.sound_timer, 0);
    }

    #[test]
    fn test_instruction_00e0() {
        let mut cpu = Cpu::new();

        cpu.display = [true; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        assert_eq!(cpu.display, [true; DISPLAY_WIDTH * DISPLAY_HEIGHT]);

        cpu.decode(0x00E0);

        assert_eq!(cpu.display, [false; DISPLAY_WIDTH * DISPLAY_HEIGHT]);

    }

    #[test]
    fn test_instruction_00ee() {
        let mut cpu = Cpu::new();
        cpu.sp = 3;
        cpu.stack[3] = 0x1C;

        cpu.decode(0x00EE);
        assert_eq!(cpu.sp, 2);
        assert_eq!(cpu.pc, 0x1C);
    }
    
    #[test]
    fn test_instruction_1nnn() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.pc, 0x200);
        cpu.decode(0x1420);
        assert_eq!(cpu.pc, 0x0420);
    }

    #[test]
    fn test_instruction_2nnn() {
        let mut cpu = Cpu::new();
        cpu.pc = 3;

        cpu.decode(0x2369);
        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.stack[1], 3);
        assert_eq!(cpu.pc, 0x0369);
    }

    #[test]
    fn test_instruction_3xnn() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0x13;
        cpu.pc = 1;

        cpu.decode(0x3026);
        assert_ne!(cpu.pc, 3);

        cpu.decode(0x3013);
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn test_instruction_4xnn() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0x13;
        cpu.pc = 1;

        cpu.decode(0x4026);
        assert_eq!(cpu.pc, 3);

        cpu.decode(0x4013);
        assert_ne!(cpu.pc, 5);
    }

    #[test]
    fn test_instruction_5xy0() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0x4;
        cpu.v[1] = 0x4;
        cpu.pc = 1;
        
        cpu.decode(0x5010); // Vx and Vy are equal
        assert_eq!(cpu.pc, 3);

        cpu.decode(0x5120); // Vx and Vy are not equal
        assert_ne!(cpu.pc, 5);
    }
    
    #[test]
    fn test_instruction_6xnn() {
        let mut cpu = Cpu::new();
        
        cpu.decode(0x6CD4);
        assert_eq!(cpu.v[0xC], 0x0D4);

        cpu.decode(0x643F);
        assert_eq!(cpu.v[0x4], 0x03F);
    }

    #[test]
    fn test_instruction_7xnn() {
        let mut cpu = Cpu::new();

        cpu.v[0xD] = 0x78;
        let initial_v = cpu.v[0xD];

        cpu.decode(0x7D21);
        assert_eq!(cpu.v[0xD], (0x0021 + initial_v))
    }

    #[test]
    fn test_instruction_8xy0() {
        let mut cpu = Cpu::new();
        cpu.v[2] = 0x7F;

        cpu.decode(0x8120);
        assert_eq!(cpu.v[1], 0x7F);
    }

    #[test]
    fn test_instruction_8xy1() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0xA;
        cpu.v[1] = 0xFF;

        cpu.decode(0x8011);
        assert_eq!(cpu.v[0], 0xFF);
    }

    #[test]
    fn test_instruction_8xy2() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0xA;
        cpu.v[1] = 0xFF;

        cpu.decode(0x8012);
        assert_eq!(cpu.v[0], 0xA);
    }

    #[test]
    fn test_instruction_8xy3() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0xA;
        cpu.v[1] = 0xFF;

        cpu.decode(0x8013);
        assert_eq!(cpu.v[0], 0xF5);
    }
    
    #[test]
    fn test_instruction_8xy4() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0xF;
        cpu.v[1] = 0xA;

        cpu.decode(0x8014); // Addition without carry
        assert_eq!(cpu.v[0], 0x19);
        assert_eq!(cpu.v[0xF], 0);

        cpu.v[0] = 0xFF;
        cpu.v[1] = 0xF;

        cpu.decode(0x8014); // Addition with carry
        assert_eq!(cpu.v[0], 0xE);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn test_instruction_8xy5() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0xA;
        cpu.v[1] = 0xF;

        cpu.decode(0x8015); // Subtraction with borrow (VF should be 0)
        assert_eq!(cpu.v[0], 0xFB);
        assert_eq!(cpu.v[0xF], 0);

        cpu.v[0] = 0xF;
        cpu.v[1] = 0xA;

        cpu.decode(0x8015); // Subtraction without borrow (VF should be 1)
        assert_eq!(cpu.v[0], 0x5);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn test_instruction_8xy6() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0xC; // Decimal = 12; Binary = 1100
        
        cpu.decode(0x8006); // LSB is 0
        assert_eq!(cpu.v[0], 0x6);
        assert_eq!(cpu.v[0xF], 0);

        cpu.v[0] = 0x11; // Decimal = 17; Binary = 10001
        
        cpu.decode(0x8006); // LSB is 1
        assert_eq!(cpu.v[0], 0x8);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn test_instruction_8xy7() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0xF;
        cpu.v[1] = 0xA;

        cpu.decode(0x8017); // Subtraction with borrow (VF should be 0)
        assert_eq!(cpu.v[0], 0xFB);
        assert_eq!(cpu.v[0xF], 0);

        cpu.v[0] = 0xA;
        cpu.v[1] = 0xF;

        cpu.decode(0x8017); // Subtraction without borrow (VF should be 1)
        assert_eq!(cpu.v[0], 0x5);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn test_instruction_8xye() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0xA; // Decimal = 10; Binary = 1010
        
        cpu.decode(0x800E); // MSB is 0
        assert_eq!(cpu.v[0], 0x14);
        assert_eq!(cpu.v[0xF], 0);

        cpu.v[0] = 0xF0; // Decimal = 240; Binary = 11110000
        
        cpu.decode(0x800E); // MSB is 1
        assert_eq!(cpu.v[0], 0xE0);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn test_instruction_9xy0() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0x4;
        cpu.v[1] = 0x4;
        cpu.pc = 1;
        
        cpu.decode(0x9010); // Vx and Vy are equal
        assert_ne!(cpu.pc, 3);

        cpu.decode(0x9120); // Vx and Vy are not equal
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn test_instruction_annn() {
        let mut cpu = Cpu::new();

        assert_eq!(cpu.i, 0);
        cpu.decode(0xA123);
        assert_eq!(cpu.i, 0x0123);
    }

    #[test]
    fn test_instruction_bnnn() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 0x5;
        cpu.decode(0xB666);

        assert_eq!(cpu.pc, 0x066B);
    }

    #[test]
    fn test_instruction_cxnn() {
        let mut cpu = Cpu::new();

        cpu.decode(0xC000);
        assert_eq!(cpu.v[0], 0x0);

        /* Binary of F:  00001111
         * Binary of F0: 11110000
         * Therefore, (F & [random u8]) & F0 should always be 0 */
        cpu.decode(0xC00F);
        assert_eq!(cpu.v[0] & 0xF0, 0)
    }

    #[test]
    /*
     * For this test, the first three lines and three columns of the display will be used,
     * with the initial state as below:
     * 
     *    Before
     *  1	0	1
     *  1	1	1
     *  0	1	0
     * 
     * 
     * Because the test uses a 3x3 display area, the sprite chosen has height N = 3, represented below:
     * 
     *   HEX      BIN       Sprite
     *   0x3C   00111100     ****
     *   0xC3   11000011   **    **
     *   0xFF   11111111   ********
     * 
     * 
     * After the sprite is drawn on the display, the 3x3 display area will change as below:
     * 
     *   After
     * 1   0   0
     * 0   0   1
     * 1   0   1
     * 
     * Also, consider that initially I = 0, X = 0 and Y = 0.
     */
    fn test_instruction_dxyn() {
        let mut cpu = Cpu::new();
        
        // Loading sprite in memory
        cpu.memory[0] = 0x3C; // memory[I]
        cpu.memory[1] = 0xC3; // memory[I + 1]
        cpu.memory[2] = 0xFF; // memory[I + 2]

        // Setting up display initial state
        cpu.display[0] = true; // First line
        cpu.display[1] = false;
        cpu.display[2] = true;

        cpu.display[0 + 1 * DISPLAY_WIDTH] = true; // Second line
        cpu.display[1 + 1 * DISPLAY_WIDTH] = true;
        cpu.display[2 + 1 * DISPLAY_WIDTH] = true;

        cpu.display[0 + 2 * DISPLAY_WIDTH] = false; // Third line
        cpu.display[1 + 2 * DISPLAY_WIDTH] = true;
        cpu.display[2 + 2 * DISPLAY_WIDTH] = false;

        cpu.decode(0xD003);
        
        assert_eq!(cpu.display[0], true); // Checking first line result
        assert_eq!(cpu.display[1], false);
        assert_eq!(cpu.display[2], false);

        assert_eq!(cpu.display[0 + 1 * DISPLAY_WIDTH], false); // Checking second line result
        assert_eq!(cpu.display[1 + 1 * DISPLAY_WIDTH], false);
        assert_eq!(cpu.display[2 + 1 * DISPLAY_WIDTH], true);

        assert_eq!(cpu.display[0 + 2 * DISPLAY_WIDTH], true); // Checking third line result
        assert_eq!(cpu.display[1 + 2 * DISPLAY_WIDTH], false);
        assert_eq!(cpu.display[2 + 2 * DISPLAY_WIDTH], true);

        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn test_instruction_ex9e() {
        let mut cpu = Cpu::new();
        
        cpu.v[0] = 0xF;
        cpu.keypad[0xF] = true;
        cpu.pc = 1;
        cpu.decode(0xE09E);
        
        assert_eq!(cpu.pc, 3);

        cpu.v[0] = 0x3;
        cpu.decode(0xE09E);
        
        assert_ne!(cpu.pc, 5);
    }

    #[test]
    fn test_instruction_exa1() {
        let mut cpu = Cpu::new();
        
        cpu.v[0] = 0xF;
        cpu.keypad[0xF] = true;
        cpu.pc = 1;
        cpu.decode(0xE0A1);
        
        assert_ne!(cpu.pc, 3);

        cpu.v[0] = 0x3;
        cpu.decode(0xE0A1);
        
        assert_eq!(cpu.pc, 3);
    }

    fn test_instruction_fx07() {

    }

    fn test_instruction_fx0a() {

    }

    fn test_instruction_fx15() {

    }

    fn test_instruction_fx18() {

    }

    fn test_instruction_fx1e() {

    }

    fn test_instruction_fx29() {

    }

    fn test_instruction_fx33() {

    }

    fn test_instruction_fx55() {

    }

    fn test_instruction_fx65() {

    }
}