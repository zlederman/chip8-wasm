import { drawGrid, drawPixels } from './display';
const wasm = import('../pkg')
const wasm_memory = import('../pkg/index_bg.wasm')
var memory;
var chip8;
var mod;


function render() {
    for(let i=0; i < 10; i++){
        chip8.tick();
    }
    drawGrid();
    drawPixels(chip8, memory, mod);
    requestAnimationFrame(render);
}
async function initChip8(rom){
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
   mod = await wasm;
   memory = (await wasm_memory).memory
   let rom = await loadRom();
   chip8 = await initChip8(rom);
   render()
}
start().then().catch(console.error);
