use wasm_bindgen::prelude::*;



#[wasm_bindgen]
pub struct Instruction{
    pub operation: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16
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