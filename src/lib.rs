pub mod instructions;
mod operations;

use instructions::Instruction;


use wasm_bindgen::prelude::*;
use js_sys;


const MEM_SIZE: usize = 4096;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const PIXELS: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH;
const START_OF_PROG: usize = 0x200;



#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace= console)]
    pub fn log(s: &str);
}
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelState{
    ON = 1,
    OFF = 0
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyState{
    ON = 1,
    OFF = 0
}


#[wasm_bindgen]
pub struct Chip8 {
    pc: usize,
    index: usize,
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<usize>,
    display: [PixelState; PIXELS],
    memory: [u8; MEM_SIZE],
    gp_reg: [u8; 16],
    keypad: [KeyState; 16]
}


#[wasm_bindgen]
impl Chip8 {
    pub fn new(rom: &js_sys::Uint8Array) -> Chip8 {
        let mut mem:[u8;MEM_SIZE] = [0; MEM_SIZE];
        let mut slice: Vec<u8> = vec![0; rom.length() as usize];
        rom.copy_to(&mut slice[..]);
        for (i, val) in slice.iter().enumerate(){
            mem[i + START_OF_PROG] = val.clone() as u8;
        }
        return Chip8 {
            pc: START_OF_PROG,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: Vec::new(),
            display: [PixelState::OFF; PIXELS],
            memory: mem,
            gp_reg: [0; 16],
            keypad: [KeyState::OFF; 16]
        }
    }

    fn exec(&mut self, instr: Instruction){
        match instr.operation{
            0x0 => self.zero(instr), 
            0x1 => self.jump(instr),
            0x2 => self.push(instr),
            0x3 => self.skip_if_eq(instr),
            0x4 => self.skip_if_neq(instr),
            0x5 => self.skip_eq_reg(instr),
            0x6 => self.set_register(instr),
            0x7 => self.add_register(instr),
            0x8 => self.eight(instr),
            0x9 => self.skip_neq_reg(instr),
            0xA => self.set_index(instr),
            0xB => self.offset_jump(instr),
            0xC => self.random(instr),
            0xD => self.draw(instr),
            0xE => self.skip_key(instr),
            0xF => self.f(instr),
            _ => panic!("Unknown instruction!!!!")
        }
    }

    fn fetch(&mut self) -> Instruction{
        let upper_byte = self.memory[self.pc];
        let lower_byte = self.memory[self.pc + 1];
        let raw = u16::from_be_bytes([upper_byte, lower_byte]);
        return Instruction { 
            operation: ((raw >> 12) & 0xF) as u8,
            x: ((raw >> 8) & 0xF) as u8,
            y: ((raw >> 4) & 0xF) as u8,
            n: (raw & 0xF) as u8,
            nn: (raw & 0xFF) as u8,
            nnn: (raw & 0xFFF) as u16
        }
    }
    pub fn get_display(&self) -> *const PixelState {
        return self.display.as_ptr();
    }
    pub fn tick(&mut self){ 
        let instr = self.fetch();
        self.pc += 2;
        let instr_debug = instr.to_string();
        // console_log!("Instruction {}",instr_debug);
        self.exec(instr);
    }
    pub fn get_pc(&self) -> usize{
        return self.pc;
    }
    pub fn get_top_of_stack(&self) -> usize {
        return self.stack[0];
    }
    pub fn get_register(&self, idx: usize) -> u8 {
        return self.gp_reg[idx];
    }
}