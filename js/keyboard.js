
const keyMap = {
    'KeyQ':0,
    'KeyW':1,
    'KeyE':2,
    'KeyR':3,
    'KeyT':4,
    'KeyA':5,
    'KeyS':6,
    'KeyD':7,
    'KeyF':8,
    'KeyG':9,
    'KeyZ':0xA,
    'KeyX':0xB,
    'KeyC':0xC,
    'KeyV':0xD,
    'KeyB':0xE,
    'Space':0xF,
}

export function keyBoardSetUp(chip8, mod){
    document.addEventListener('keydown',(event)=>{
        if(event.code in keyMap){
            chip8.set_key_state(keyMap[event.code],mod.KeyState.ON);
            console.log(`pressed Key ${event.code} with map ${keyMap[event.code]}`);
        }
        else{
            console.error("Pressed unknown key "+ event.code);
        }
     })
     document.addEventListener('keyup',(event)=>{
        if(event.code in keyMap){
            chip8.set_key_state(keyMap[event.code],mod.KeyState.OFF);
            console.log(`pressed Key ${event.code} with map ${keyMap[event.code]}`);
        }
        else{
            console.error("Pressed unknown key "+ event.code);
        }
     })
}
