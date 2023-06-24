use wasm_bindgen_test::{wasm_bindgen_test};
use wasm_bindgen::JsValue;
use js_sys::Uint8Array;
use chip8_emulator::Chip8;
use chip8_emulator::instructions::Instruction;

fn empty_program() -> Uint8Array {
    return Uint8Array::new(&JsValue::from(43));
}

#[wasm_bindgen_test]
fn push_test(){
    let instr = Instruction{
        operation:0x2, x:0,y:0,n:0, nn:0, nnn:0x390
    };
    let mut chip8 = Chip8::new(&empty_program());
    chip8.push(instr);
    assert_eq!(chip8.get_pc(),0x390);
}