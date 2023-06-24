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
    let instr = Instruction::from_str("2390");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.push(instr);
    assert_eq!(chip8.get_pc(),0x390);
    assert_eq!(chip8.get_top_of_stack(),0x200);
}

#[wasm_bindgen_test]
fn pop_test(){
    let push_instr= Instruction::from_str("2390");
    let instr = Instruction::from_str("0000");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.push(push_instr);
    assert_eq!(chip8.get_pc(),0x390);
    chip8.pop(instr);
    assert_eq!(chip8.get_pc(),0x200);
}


#[wasm_bindgen_test]
fn set_register_test(){
    let instr = Instruction::from_str("60EE");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_register(instr);
    let v0 = chip8.get_register(0);
    assert_eq!(v0, 0xEE);
}

#[wasm_bindgen_test]
fn add_register_test(){
    let instr = Instruction::from_str("6003");
    let add_instr = Instruction::from_str("7004");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_register(instr);
    let v0 = chip8.get_register(0);
    assert_eq!(v0, 0x03);
    chip8.add_register(add_instr);
    let v0 = chip8.get_register(0);
    assert_eq!(v0, 0x07);
    let overflow_instr = Instruction::from_str("70FF");
    chip8.add_register(overflow_instr);
    let v0 = chip8.get_register(0);
    let vF = chip8.get_register(0xF);
    assert_eq!(v0, 0xFF);
    assert_eq!(vF, 0x01);
}