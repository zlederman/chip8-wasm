
use wasm_bindgen::prelude::*;
use js_sys;
const MEM_SIZE: usize = 4096;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const PIXELS: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH;
const START_OF_PROG: usize = 0x200;


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
struct Chip8 {
    pc: usize,
    index: usize,
    display: [PixelState; PIXELS],
    memory: [u8; MEM_SIZE],
    gp_reg: [u8; 16],
}
#[wasm_bindgen]
impl Chip8 {
    fn clear(&mut self, instr: Instruction){
        if instr.nnn == 0x0E0{
            for i in 0..PIXELS{
                self.display[i] = PixelState::OFF;
            }
        }
    }

    fn jump(&mut self, instr: Instruction){
        self.pc = instr.nnn as usize;
    }

    fn set_register(&mut self, instr: Instruction){
        self.gp_reg[instr.x as usize] = instr.nn;
    }

    fn add_register(&mut self, instr: Instruction){
        self.gp_reg[instr.x as usize] += instr.nn;
    }

    fn set_index(&mut self, instr: Instruction){
        self.index = instr.nnn as usize;
    }

    fn draw(&mut self, instr: Instruction){
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
            display: [PixelState::OFF; PIXELS],
            memory: mem,
            gp_reg: [0; 16]
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
        console_log!("Instruction {}",instr_debug);
        self.exec(instr);
    }
}