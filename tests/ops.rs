use wasm_bindgen_test::{wasm_bindgen_test};
use wasm_bindgen::JsValue;
use js_sys::Uint8Array;
use chip8_emulator::Chip8;
use chip8_emulator::KeyState;
use chip8_emulator::fonts::{FONT_OFFSET};
use chip8_emulator::instructions::Instruction;
fn empty_program() -> Uint8Array {
    return Uint8Array::new(&JsValue::from(43));
}
#[wasm_bindgen_test]
fn pop_test(){
    // test 00EE
    let push_instr= Instruction::from_str("2390");
    let instr = Instruction::from_str("0000");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.push(push_instr);
    assert_eq!(chip8.get_pc(),0x390);
    chip8.pop(instr);
    assert_eq!(chip8.get_pc(),0x200);
}

#[wasm_bindgen_test]
fn jump_test(){
    // test 1NNN
    let jumper = Instruction::from_str("1369");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.jump(jumper);
    assert_eq!(chip8.get_pc(), 0x369);
}

#[wasm_bindgen_test]
fn offset_jump_test(){
    // test BNNN
    let setter = Instruction::from_str("6001");
    let offseter = Instruction::from_str("B368");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_register(setter);
    chip8.offset_jump(offseter);
    assert_eq!(chip8.get_pc(), 0x369);
}  
#[wasm_bindgen_test]
fn index_set_test(){
    // test ANNN
    let setter = Instruction::from_str("A333");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_index(setter);
    assert_eq!(chip8.get_index(), 0x333);

}

#[wasm_bindgen_test]
fn index_add_test(){
    // test FX1E
    let setter = Instruction::from_str("6033");
    let adder = Instruction::from_str("F01E");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_register(setter);
    chip8.add_to_index(adder);
    assert_eq!(chip8.get_index(), 0x033);
}

#[wasm_bindgen_test]
fn push_test(){
    // test 2NNN
    let instr = Instruction::from_str("2390");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.push(instr);
    assert_eq!(chip8.get_pc(),0x390);
    assert_eq!(chip8.get_top_of_stack(),0x200);
}


#[wasm_bindgen_test]
fn test_skip_if_eq(){
    // test 3XNN
    let set_instr = Instruction::from_str("60EE");
    let skip_instr_pass = Instruction::from_str("30EE");
    let skip_instr_fail = Instruction::from_str("3000");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_register(set_instr);
    chip8.skip_if_eq(skip_instr_fail);
    assert_ne!(chip8.get_pc(), 0x202);
    chip8.skip_if_eq(skip_instr_pass);
    assert_eq!(chip8.get_pc(), 0x202);
}

#[wasm_bindgen_test]
fn test_skip_if_neq(){
    // test 4XNN
    let set_instr = Instruction::from_str("60EE");
    let skip_instr_pass = Instruction::from_str("4000");
    let skip_instr_fail = Instruction::from_str("40EE");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_register(set_instr);
    chip8.skip_if_neq(skip_instr_fail);
    assert_ne!(chip8.get_pc(), 0x202);
    chip8.skip_if_neq(skip_instr_pass);
    assert_eq!(chip8.get_pc(), 0x202);
}

#[wasm_bindgen_test]
fn test_skip_eq_reg(){
    // test 5XY0
    let set_y_instr = Instruction::from_str("60EE");
    let set_x_instr = Instruction::from_str("61EE");
    let skip_pass = Instruction::from_str("5010");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_register(set_x_instr);
    chip8.set_register(set_y_instr);
    chip8.skip_eq_reg(skip_pass);
    assert_eq!(chip8.get_pc(), 0x202);
}

#[wasm_bindgen_test]
fn test_skip_neq_reg(){
    // test 9XY0
    let set_y_instr = Instruction::from_str("60EE");
    let set_x_instr = Instruction::from_str("61EF");
    let skip_pass = Instruction::from_str("9010");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_register(set_x_instr);
    chip8.set_register(set_y_instr);
    chip8.skip_neq_reg(skip_pass);
    assert_eq!(chip8.get_pc(), 0x202);
}

#[wasm_bindgen_test]
fn set_register_test(){
    //  test 6XNN
    let instr = Instruction::from_str("60EE");
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_register(instr);
    let v0 = chip8.get_register(0);
    assert_eq!(v0, 0xEE);
}

#[wasm_bindgen_test]
fn add_register_test(){
    // test 7NXX
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
    let vf = chip8.get_register(0xF);
    assert_eq!(v0, 0xFF);
    assert_eq!(vf, 0x00);
}


#[wasm_bindgen_test]
fn math_basic_test(){
    // 8XY[0:4]
    let mut chip8 = Chip8::new(&empty_program());
    let init_x = Instruction::from_str("60EE");
    let init_y = Instruction::from_str("6111");
    let set = Instruction::from_str("8010");
    let or = Instruction::from_str("8011");
    let and  = Instruction::from_str("8012");
    let xor = Instruction::from_str("8013");
    chip8.set_register(init_x.clone());
    chip8.set_register(init_y);
    chip8.eight(or);
    assert_eq!(chip8.get_register(0),0xEE | 0x11);
    chip8.set_register(init_x.clone());
    chip8.eight(set);
    assert_eq!(chip8.get_register(0), 0x11);
    chip8.set_register(init_x.clone());
    chip8.eight(and);
    assert_eq!(chip8.get_register(0), 0xEE & 0x11);
    chip8.set_register(init_x.clone());
    chip8.eight(xor);
    assert_eq!(chip8.get_register(0), 0xEE ^ 0x11);
}

#[wasm_bindgen_test]
fn stateful_math_add_test(){
    // 8XY4
    let mut chip8 = Chip8::new(&empty_program());
    let init_x = Instruction::from_str("60EE");
    let init_y = Instruction::from_str("6111");
    let init_x_overflow = Instruction::from_str("60FE");
    chip8.set_register(init_x.clone());
    chip8.set_register(init_y);
    let add = Instruction::from_str("8014");
    chip8.eight(add.clone());
    assert_eq!(chip8.get_register(0), 0xEE + 0x11);
    assert_eq!(chip8.get_register(0xF), 0);
    chip8.set_register(init_x_overflow);
    chip8.eight(add.clone());
    assert_eq!(chip8.get_register(0), 0xFF);
    assert_eq!(chip8.get_register(0xF), 1);
}

#[wasm_bindgen_test]
fn stateful_math_sub_xy_test(){
    // 8XY5
    let mut chip8 = Chip8::new(&empty_program());
    let init_x_overflow = Instruction::from_str("60EE");
    let init_y = Instruction::from_str("6111");
    let init_x = Instruction::from_str("6001");
    let sub_xy = Instruction::from_str("8015");
    chip8.set_register(init_x_overflow);
    chip8.set_register(init_y);
    chip8.eight(sub_xy.clone());
    assert_eq!(chip8.get_register(0), 0xEE - 0x11);
    assert_eq!(chip8.get_register(0xF), 1);
    chip8.set_register(init_x);
    chip8.eight(sub_xy.clone());
    assert_eq!(chip8.get_register(0), 0x01_u8.wrapping_sub(0x11));
    assert_eq!(chip8.get_register(0xF), 0);
}

#[wasm_bindgen_test]
fn stateful_math_sub_yx_test(){
    // 8XY7
    let mut chip8 = Chip8::new(&empty_program());
    let init_y_overflow = Instruction::from_str("61EE");
    let init_x = Instruction::from_str("6011");
    let init_y = Instruction::from_str("6101");
    let sub_yx = Instruction::from_str("8017");
    chip8.set_register(init_y_overflow);
    chip8.set_register(init_x.clone());
    chip8.eight(sub_yx.clone());
    assert_eq!(chip8.get_register(0), 0xEE - 0x11);
    assert_eq!(chip8.get_register(0xF), 1);
    chip8.set_register(init_y);
    chip8.set_register(init_x.clone());
    chip8.eight(sub_yx.clone());
    assert_eq!(chip8.get_register(0), 0x01_u8.wrapping_sub(0x11));
    assert_eq!(chip8.get_register(0xF), 0);
}

#[wasm_bindgen_test]
fn shift_left_test(){
    // 8XYE
    let mut chip8 = Chip8::new(&empty_program());
    let init_x = Instruction::from_str("60FF");
    let init_x_small = Instruction::from_str("6001");
    let lshift = Instruction::from_str("801E");
    chip8.set_register(init_x);
    chip8.eight(lshift.clone());
    assert_eq!(chip8.get_register(0),0xFF);
    assert_eq!(chip8.get_register(0xF),0x1);
    chip8.set_register(init_x_small);
    chip8.eight(lshift.clone());
    assert_eq!(chip8.get_register(0),0x02);
    assert_eq!(chip8.get_register(0xF),0x0);
}

#[wasm_bindgen_test]
fn shift_right_test(){
    // 8XY6
    let mut chip8 = Chip8::new(&empty_program());
    let init_x = Instruction::from_str("60FF");
    let init_x_small = Instruction::from_str("6000");
    let rshift = Instruction::from_str("8016");
    chip8.set_register(init_x);
    chip8.eight(rshift.clone());
    assert_eq!(chip8.get_register(0),0xFF_u8.saturating_div(2));
    assert_eq!(chip8.get_register(0xF),0x1);
    chip8.set_register(init_x_small);
    chip8.eight(rshift.clone());
    assert_eq!(chip8.get_register(0),0x00);
    assert_eq!(chip8.get_register(0xF),0x0);
}

#[wasm_bindgen_test]
fn test_timers(){
    let mut chip8 = Chip8::new(&empty_program());
    let init_x = Instruction::from_str("61FE");
    let set_delay = Instruction::from_str("F115");
    let set_sound = Instruction::from_str("F118");
    let get_delay = Instruction::from_str("F207");
    chip8.set_register(init_x);
    chip8.f(set_delay);
    chip8.f(set_sound);
    chip8.f(get_delay);
    assert_eq!(chip8.delay_timer, 0xFE);
    assert_eq!(chip8.sound_timer, 0xFE);
    assert_eq!(chip8.get_register(2), 0xFE);
}
#[wasm_bindgen_test]
fn test_store_and_load(){
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_index(Instruction::from_str("A300"));
    let setters = ["6001","6102","6203"];
    let resetters = ["6000","6100","6200"];
    let store3 = Instruction::from_str("F255");
    let load3 = Instruction::from_str("F265");
    for instr_str in setters{
        chip8.set_register(Instruction::from_str(instr_str));
    }
    chip8.f(store3);
    for instr_str in resetters{
        chip8.set_register(Instruction::from_str(instr_str));
    }
    chip8.f(load3);
    for i in 0..3{
        assert_eq!(chip8.get_register(i),i as u8 + 1);
    }
}

#[wasm_bindgen_test]
fn test_get_font(){
    let mut chip8 = Chip8::new(&empty_program());
    let mut font_idxs = [0_usize;16];
    for i in 0..16 {
        font_idxs[i] = (i * 5) + FONT_OFFSET;
    }
    for i in 0..16 {
        chip8.set_register(Instruction::from_str(format!("60{:02X}",i as u16).as_str()));
        chip8.f(Instruction::from_str("F029"));
        assert_eq!(chip8.get_index(),font_idxs[i])
    }
}

#[wasm_bindgen_test]
fn test_decimal_conversion(){
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_register(Instruction::from_str("60FE"));
    chip8.f(Instruction::from_str("F033"));
    assert_eq!(chip8.get_mem_at(0),2);
    assert_eq!(chip8.get_mem_at(1),5);
    assert_eq!(chip8.get_mem_at(2),4);
}

#[wasm_bindgen_test]
fn test_get_key(){
    let mut chip8 = Chip8::new(&empty_program());
    chip8.f(Instruction::from_str("F00A"));
    assert_eq!(chip8.get_pc(), 0x200 - 2);
    chip8.set_key_state(2, KeyState::ON);
    chip8.f(Instruction::from_str("F00A"));
    assert_eq!(chip8.get_register(0),2);
}

#[wasm_bindgen_test]
fn test_skip_key_eq(){
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_register(Instruction::from_str("600F"));
    chip8.set_key_state(0xF, KeyState::ON);
    chip8.skip_key(Instruction::from_str("E09E"));
    assert_eq!(chip8.get_pc(), 0x200 + 2);
}
#[wasm_bindgen_test]
fn test_skip_key_neq(){
    let mut chip8 = Chip8::new(&empty_program());
    chip8.set_register(Instruction::from_str("600F"));
    chip8.set_key_state(0xF, KeyState::OFF);
    chip8.skip_key(Instruction::from_str("E0A1"));
    assert_eq!(chip8.get_pc(), 0x200 + 2);
}