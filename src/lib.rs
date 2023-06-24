
use std::thread::__LocalKeyInner;

use wasm_bindgen::prelude::*;
use js_sys;
use rand::prelude::*;


const MEM_SIZE: usize = 4096;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const PIXELS: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH;
const START_OF_PROG: usize = 0x200;
const ADD: u8 = 4;
const SUB_XY: u8 = 5;
const SUB_YX: u8 = 7;
const SHIFT_LEFT: u8 = 0xE;
const SHIFT_RIGHT: u8 = 0x6;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace= console)]
    fn log(s: &str);
}
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
#[wasm_bindgen]
pub struct Instruction{
    operation: u8,
    x: u8,
    y: u8,
    n: u8,
    nn: u8,
    nnn: u16
}

#[wasm_bindgen]
impl Instruction {
    pub fn to_string(& self) -> String{
        return format!("op={op:2X} x={x:2X} y={y:2X} n={n:2X} nn={nn:2X} nnn={nnn:03X}",
            op=self.operation,
            x=self.x,
            y=self.y,
            n=self.n,
            nn=self.nn,
            nnn=self.nnn
        )
    }
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
struct Chip8 {
    pc: usize,
    index: usize,
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
            stack: Vec::new(),
            display: [PixelState::OFF; PIXELS],
            memory: mem,
            gp_reg: [0; 16],
            keypad: [KeyState::OFF; 16]
        }
    }
    fn exec(&mut self, instr: Instruction){
        match instr.operation{
            0x0 => self.clear(instr),
            0x1 => self.jump(instr),
            0x6 => self.set_register(instr),
            0x7 => self.add_register(instr),
            0xA => self.set_index(instr),
            0xD => self.draw(instr),
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
}

#[wasm_bindgen]
impl Chip8 {

    fn push(&mut self, instr:Instruction){
        // 2NNN
        self.stack.push(self.pc);
        self.pc = instr.nnn as usize;
    }
    fn pop(&mut self, instr: Instruction){
        // 00EE
        self.pc = self.stack.pop().expect("Stack is Empty!!!");

    }
    fn skip_if_eq(&mut self, instr: Instruction){
        // 3XNN
        if self.gp_reg[instr.x as usize] == instr.nn{
            self.pc += 2;
        } 
    }
    fn skip_if_neq(&mut self, instr: Instruction){
        // 4XNN
        if self.gp_reg[instr.x as usize] != instr.nn{
            self.pc += 2;
        } 
    }
    fn skip_eq_reg(&mut self, instr: Instruction){
        // 5XY0
        if self.gp_reg[instr.x as usize] == self.gp_reg[instr.y as usize]{
            self.pc += 2;
        }
    }
    fn skip_neq_reg(&mut self, instr: Instruction){
        //  9XY0
        if self.gp_reg[instr.x as usize] != self.gp_reg[instr.y as usize]{
            self.pc += 2;
        }
    }

    fn skip_key(&mut self, instr: Instruction){
        // EX9E and EXA1
        let key = self.gp_reg[instr.x as usize] as usize;
        if instr.nn == 0x9E && self.keypad[key] == KeyState::ON {
            self.pc += 2;
        }
        if instr.nn == 0xA1 && self.keypad[key] == KeyState::OFF {
            self.pc += 2;
        }
    }
    fn clear(&mut self, instr: Instruction){
        // 00E0
        if instr.nnn == 0x0E0{
            for i in 0..PIXELS{
                self.display[i] = PixelState::OFF;
            }
        }
    }

    fn jump(&mut self, instr: Instruction){
        // 1NNN
        self.pc = instr.nnn as usize;
    }
    fn offset_jump(&mut self, instr: Instruction){
        // BNNN
        self.pc = self.gp_reg[0] as usize + instr.nnn as usize;
    }
    fn set_register(&mut self, instr: Instruction){
        // 6XNN
        self.gp_reg[instr.x as usize] = instr.nn;
    }

    fn add_register(&mut self, instr: Instruction){
        // 7XNN
        self.gp_reg[instr.x as usize].saturating_add(instr.nn);
    }

    fn set_index(&mut self, instr: Instruction){
         // AXNN
        self.index = instr.nnn as usize;
    }
    fn add_to_index(&mut self, instr: Instruction){
        // FX1E
        let vx = self.gp_reg[instr.x as usize];
        self.index = self.index.saturating_add(vx as usize);
    }

    fn stateful_arithmetic(&mut self, instr: Instruction){
        let x = instr.x as usize;
        let y = instr.y as usize;
        let vx = self.gp_reg[x];
        let vy = self.gp_reg[y];
        let mut val: u8 = 0;
        let mut flag: u8 = 0;
        if instr.n == ADD {
            val = vx.saturating_add(vy);
            flag = if vx.checked_add(vy) == None {1} else {0};
        }
        if instr.n == SUB_XY{
            val = vx.wrapping_sub(vy);
            flag = if vx.checked_sub(vy) == None {0} else {1};

        }
        if instr.n == SUB_YX {
            val = vy.wrapping_sub(vx);
            flag = if vy.checked_sub(vx) == None {0} else {1};
        }
        if instr.n == SHIFT_LEFT {
            val = vx << 1;
            flag = (vx >> 7) & 1;

        }
        if instr.n == SHIFT_RIGHT {
            val = vy >> 1;
            flag = vx & 1;
        }
        self.gp_reg[x] = val;
        self.gp_reg[0xF] = flag;
    }

    fn math(&mut self, instr: Instruction){
        // 8XYN
        let x = instr.x as usize;
        let y = instr.y as usize;
        match instr.n {
            0 => self.gp_reg[x] = self.gp_reg[y],
            1 => self.gp_reg[x] |= self.gp_reg[y],
            2 => self.gp_reg[x] &= self.gp_reg[y],
            3 => self.gp_reg[x] ^= self.gp_reg[y],
            4 | 5 | 6 | 7 | 0xE => self.stateful_arithmetic(instr),
            _ => console_log!("Error with {:?}",instr.to_string())

        }
    }
    fn random(&mut self, instr: Instruction){
        // CXNN
        let random = rand::random::<u8>();
        self.gp_reg[instr.x as usize] = random & instr.nn;
    }
    fn draw(&mut self, instr: Instruction){
        // DXYN
        let x = (self.gp_reg[instr.x as usize] % 64) as u16;
        let mut y = (self.gp_reg[instr.y as usize] % 32) as u16;
        self.gp_reg[0xF] = 0;
        for i in 0..instr.n{
            let temp_idx = self.index + i as usize;
            let sprite_byte = self.memory[temp_idx];
            for mask_idx in 0..8{
                let pixel_idx = ((x + mask_idx) + (y * 64)) as usize;
                let pixel = self.display[pixel_idx];
                let bit = (sprite_byte >> 7 - mask_idx) & 1;
                if bit == 1{
                    if pixel == PixelState::ON{
                        self.display[pixel_idx] = PixelState::OFF;
                        self.gp_reg[0xF] = 1;
                    }
                    else{
                        self.display[pixel_idx] = PixelState::ON;
                    }
                }
            }
            y += 1;
        }
    }
}