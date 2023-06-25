use crate::{KeyState, PixelState};
use crate::log;
use crate::Chip8;
use crate::Instruction;
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
const FONT_OFFSET: usize = 0x050;

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
impl Chip8 {

    pub fn eight(&mut self, instr: Instruction){
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
    pub fn zero(&mut self, instr: Instruction){
        match instr.nn {
            0xE0 => self.clear(instr),
            0xEE => self.pop(instr),
            _ => console_log!("Unknown Zero! {:?}", instr.to_string())
        }
    }
    pub fn f(&mut self, instr: Instruction){
        match instr.nn {
            0x07 | 0x15 | 0x18 => self.timers(instr),
            0x1E => self.add_to_index(instr),
            0x0A => self.get_key(instr),
            0x29 => self.get_font(instr),
            0x33 => self.decimal_conversion(instr),
            0x55 => self.store(instr),
            0x65 => self.load(instr),
            _ => console_log!("Unknown FXNN instruction {:?}",instr.to_string())
        }
    }
    pub fn push(&mut self, instr:Instruction){
        // 2NNN
        self.stack.push(self.pc);
        self.pc = instr.nnn as usize;
    }
    pub fn pop(&mut self, instr: Instruction){
        // 00EE
        self.pc = self.stack.pop().expect("Stack is Empty!!!");

    }
    pub fn skip_if_eq(&mut self, instr: Instruction){
        // 3XNN
        if self.gp_reg[instr.x as usize] == instr.nn{
            self.pc += 2;
        } 
    }
    pub fn skip_if_neq(&mut self, instr: Instruction){
        // 4XNN
        if self.gp_reg[instr.x as usize] != instr.nn{
            self.pc += 2;
        } 
    }
    pub fn skip_eq_reg(&mut self, instr: Instruction){
        // 5XY0
        if self.gp_reg[instr.x as usize] == self.gp_reg[instr.y as usize]{
            self.pc += 2;
        }
    }
    pub fn skip_neq_reg(&mut self, instr: Instruction){
        //  9XY0
        if self.gp_reg[instr.x as usize] != self.gp_reg[instr.y as usize]{
            self.pc += 2;
        }
    }

    pub fn skip_key(&mut self, instr: Instruction){
        // EX9E and EXA1
        let key = self.gp_reg[instr.x as usize] as usize;
        if instr.nn == 0x9E && self.keypad[key] == KeyState::ON {
            self.pc += 2;
        }
        if instr.nn == 0xA1 && self.keypad[key] == KeyState::OFF {
            self.pc += 2;
        }
    }
    pub fn clear(&mut self, instr: Instruction){
        // 00E0
        if instr.nnn == 0x0E0{
            for i in 0..PIXELS{
                self.display[i] = PixelState::OFF;
            }
        }
    }

    pub fn jump(&mut self, instr: Instruction){
        // 1NNN
        self.pc = instr.nnn as usize;
    }
    pub fn offset_jump(&mut self, instr: Instruction){
        // BNNN
        self.pc = self.gp_reg[0] as usize + instr.nnn as usize;
    }
    pub fn set_register(&mut self, instr: Instruction){
        // 6XNN
        self.gp_reg[instr.x as usize] = instr.nn;
    }

    pub fn add_register(&mut self, instr: Instruction){
        // 7XNN
        let vx = self.gp_reg[instr.x as usize];
        self.gp_reg[instr.x as usize] = vx.saturating_add(instr.nn);
    }

    pub fn set_index(&mut self, instr: Instruction){
         // ANNN
        self.index = instr.nnn as usize;
    }
    pub fn add_to_index(&mut self, instr: Instruction){
        // FX1E
        let vx = self.gp_reg[instr.x as usize];
        self.index = self.index.saturating_add(vx as usize);
    }
    pub fn timers(&mut self, instr: Instruction){
        // FX07, FX15, FX18
        match instr.nn {
            0x07 => self.gp_reg[instr.x as usize] = self.delay_timer,
            0x15 => self.delay_timer = self.gp_reg[instr.x as usize],
            0x18 => self.sound_timer = self.gp_reg[instr.x as usize],
            _ => console_log!("Unknown timer instruction {:?}", instr.to_string())
        }
    }

    pub fn get_key(&mut self, instr: Instruction){
        // FX0A
        let mut pressed_key = u8::MAX;
        for i in 0..16{
            if self.keypad[i] == KeyState::ON{
                pressed_key = i as u8;
                break;
            }
        }
        if pressed_key == u8::MAX{
            self.pc -= 2;
        }
        else{
            self.gp_reg[instr.x as usize] = pressed_key; 
        }
        
    }
    pub fn get_font(&mut self, instr: Instruction){
        // FX29
        let character = self.gp_reg[instr.x as usize] & 0xF;
        let char_addr = (character * 5) as usize + FONT_OFFSET;
        self.index = char_addr;

    }
    pub fn store(&mut self, instr: Instruction){
        // FX55
        let temp_idx = self.index;
        for i in 0..(instr.x + 1){
            self.memory[temp_idx + i as usize] = self.gp_reg[i as usize];
        }
    }
    pub fn load(&mut self, instr: Instruction){
        // FX65
        let temp_idx = self.index;
        for i in 0..(instr.x + 1){
            self.gp_reg[i as usize]  = self.memory[temp_idx + i as usize]; 
        }
    }
    pub fn decimal_conversion(&mut self, instr: Instruction){
        // FX33
        let mut vx = self.gp_reg[instr.x as usize];
        for i in (0..2).rev(){
            self.memory[self.index + i] = vx % 10;
            vx /= 10;
        }
    }

    pub fn stateful_arithmetic(&mut self, instr: Instruction){
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
            val = vx.saturating_mul(2);
            flag = (vx >> 7) & 1;

        }
        if instr.n == SHIFT_RIGHT {
            val = vx.saturating_div(2);
            flag = vx & 1;
        }
        self.gp_reg[x] = val;
        self.gp_reg[0xF] = flag;
    }


    pub fn random(&mut self, instr: Instruction){
        // CXNN
        let random = rand::random::<u8>();
        self.gp_reg[instr.x as usize] = random & instr.nn;
    }
    pub fn draw(&mut self, instr: Instruction){
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