const wasm = import('../pkg')

async function initChip8(rom){
    let mod = await wasm;
    let chip8 = mod.Chip8.new(rom);
    return chip8;
}
async function loadRom(){
    let data = await fetch('/roms/ibm.ch8');
    let buff = await data.arrayBuffer();
    return new Uint8Array(buff);
}
function buf2hex(buffer) { // buffer is an ArrayBuffer
    return [...new Uint8Array(buffer)]
        .map(x => x.toString(16).padStart(2, '0'))
        .join('');
}

async function start(){
   let rom = await loadRom();
   let chip8 = await initChip8(rom);
   for(let i = 0; i < rom.length / 2; i++){
    chip8.tick();
    chip8.get_display()
   }
}

start().then().catch(console.error);