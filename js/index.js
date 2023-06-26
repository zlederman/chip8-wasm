import { drawGrid, drawPixels, updateRegisters } from './display';
import { keyBoardSetUp } from './keyboard';
import {play} from './audio';
const wasm = import('../pkg')
const wasm_memory = import('../pkg/index_bg.wasm')
var memory;
var chip8;
var mod;


function render() {
    for(let i=0; i < 10; i++){
        chip8.tick();
        updateRegisters(chip8)
    }
    drawGrid();
    chip8.tick_timers();
    drawPixels(chip8, memory, mod);
    requestAnimationFrame(render);
}
async function initChip8(rom){
    let chip8 = mod.Chip8.new(rom);
    return chip8;
}
async function loadRom(rom){
    let url = "https://chip8-wasm.pages.dev/roms/"
    let data = await fetch(url + rom);
    let buff = await data.arrayBuffer();
    return new Uint8Array(buff);
}
function buf2hex(buffer) { // buffer is an ArrayBuffer
    return [...new Uint8Array(buffer)]
        .map(x => x.toString(16).padStart(2, '0'))
        .join('');
}

async function start(rom){
   mod = await wasm;
   memory = (await wasm_memory).memory;
   let loadedRom = await loadRom(rom);
   chip8 = await initChip8(loadedRom);
   keyBoardSetUp(chip8, mod);
   render();
}
let button = document.getElementById('start-button')
button.addEventListener('click',()=>{
    start(document.getElementById('rom-select').value).then().catch(console.error)
})
