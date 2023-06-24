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
    pub fn from_str(val: &str) -> Instruction {
        let mut digits = val.chars();
        let mut u8s: [u8;4] = [0;4];
        for (i,c) in digits.enumerate(){
            u8s[i] = char::to_digit(c, 16).unwrap() as u8;
        }
        let nn = u8::from_str_radix(&val[2..4], 16).expect("Cant parse string");
        let nnn = u16::from_str_radix(&val[1..4], 16).expect("Cant parse string");
        return Instruction { operation: u8s[0], x: u8s[1], y: u8s[2], n: u8s[3], nn: nn, nnn: nnn }

    }
}