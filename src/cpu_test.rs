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

#[test]
fn test_instruction_fx07() {
    let mut cpu = Cpu::new();
    cpu.delay_timer = 0xA3;

    assert_eq!(cpu.v[0], 0);
    cpu.decode(0xF007);
    assert_eq!(cpu.v[0], 0xA3);
}

#[test]
fn test_instruction_fx0a() {
    let mut cpu = Cpu::new();
    cpu.pc = 3;

    cpu.decode(0xF00A); // No keypresses
    assert_eq!(cpu.pc, 1);
    assert_eq!(cpu.v[0], 0);

    cpu.keypad[7] = true; // Keypress on index 7
    cpu.decode(0xF00A);
    assert_eq!(cpu.pc, 3);
    assert_eq!(cpu.v[0], 7);
}

#[test]
fn test_instruction_fx15() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0xA3;

    assert_eq!(cpu.delay_timer, 0);
    cpu.decode(0xF015);
    assert_eq!(cpu.delay_timer, 0xA3);
}

#[test]
fn test_instruction_fx18() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0xA3;

    assert_eq!(cpu.sound_timer, 0);
    cpu.decode(0xF018);
    assert_eq!(cpu.sound_timer, 0xA3);
}

#[test]
fn test_instruction_fx1e() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0x5;

    assert_eq!(cpu.i, 0);
    cpu.decode(0xF01E);
    assert_eq!(cpu.i, 0x5);
    cpu.decode(0xF01E);
    assert_eq!(cpu.i, 0xA);
}

#[test]
fn test_instruction_fx29() {
    let mut cpu = Cpu::new();
    cpu.v[0] = 0xD;

    let expected_value = 0x50 + (cpu.v[0] * 5) as u16;
    cpu.decode(0xF029);
    assert_eq!(cpu.i, expected_value);
}

#[test]
fn test_instruction_fx33() {
    let mut cpu = Cpu::new();  
    cpu.v[0] = 214;

    cpu.decode(0xF033);
    assert_eq!(cpu.memory[cpu.i as usize], 2);
    assert_eq!(cpu.memory[cpu.i as usize + 1], 1);
    assert_eq!(cpu.memory[cpu.i as usize + 2], 4);
}

#[test]
fn test_instruction_fx55() {
    let mut cpu = Cpu::new(); 
    cpu.i = 2000;

    for idx in 0..16 {
        cpu.v[idx] = idx as u8;
    }

    cpu.decode(0xFF55);
    for idx in 0..16 {
        assert_eq!(cpu.memory[2000 + idx], cpu.v[idx])
    }
}

#[test]
fn test_instruction_fx65() {
    let mut cpu = Cpu::new(); 
    cpu.i = 2000;

    for idx in 0..16 {
        cpu.memory[2000 + idx] = idx as u8;
    }

    cpu.decode(0xFF65);
    for idx in 0..16 {
        assert_eq!(cpu.v[idx], cpu.memory[2000 + idx]);
    }
}