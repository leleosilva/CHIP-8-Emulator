// CHIP-8 can access 4KB (4096 bytes) of RAM
const MEMORY_SIZE: usize = 4096;

// The display should be 64 pixels wide and 32 pixels tall
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

// After loading, CHIP-8 programs start at address 0x200
const START_ADDRESS: u16 = 0x200;

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
    pub memory: [u8; MEMORY_SIZE],
    
    // A program counter (PC) which points at the current instruction in memory
    pc: u16,
    
    // General-purpose variable registers numbered 0 through F hexadecimal
    v: [u8; 16],
    
    // Index register I, which is generally used to store memory addresses
    i: u16,
    
    // A stack which is used to call subroutines/functions and return from them
    stack: [u16; 16],
    
    // Stack pointer which is used to point to the topmost level of the stack
    sp: u8,
    
    // Delay timer which is decremented at a rate of 60 Hz until it reaches 0
    delay_timer: u8,
    
    // Sound timer which gives off a beeping sound as long as it’s not 0
    sound_timer: u8,
    
    // A display that updates at 60 Hz and whose each pixel can be on or off
    display: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    
    /* CHIP-8 uses a hexadecimal keypad that had 16 keys, labelled 0 through F,
     * and were arranged in a 4x4 grid */
    keypad: [bool; 16],
    
}

impl Cpu {

    // Creating new instance of CPU
    pub fn new() -> Cpu {
        
        // Initializing memory with 0's and storing font data at 0x50 ~ 0x9F address interval
        let mut aux_memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
        aux_memory[0x50..(0x50 + CHIP8_FONT.len())].clone_from_slice(&CHIP8_FONT);

        Cpu {
            memory: aux_memory,
            pc: START_ADDRESS, // Program counter starts at the initial address
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            keypad: [false; 16], // Keys start as not pressed
        }
    }

    // Loading ROM data into memory, starting at the initial address
    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.memory[(START_ADDRESS as usize)..(START_ADDRESS as usize + rom_data.len())].copy_from_slice(rom_data);
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
        
        // PC is incremented by 2 to be ready to fetch the next instruction 
        self.pc += 2;

        instruction_opcode
    }

    // Decoding the instruction according to its opcode to find out what the emulator should do
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
    }

    /* EXECUTION OF INDIVIDUAL INSTRUCTIONS */


}