
const wasm = import('../pkg')
const wasm_memory = import('../pkg/index_bg.wasm')
var memory;
var chip8;
var mod;
height = 32;
width = 64;
const PIXEL_SIZE = 16;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const canvas = document.getElementById("chip8-screen");

const ctx = canvas.getContext('2d');
canvas.height = (PIXEL_SIZE + 1) * height + 1;
canvas.width = (PIXEL_SIZE + 1) * width + 1;


function drawGrid() {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
  
    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (PIXEL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (PIXEL_SIZE + 1) + 1, (PIXEL_SIZE + 1) * height + 1);
    }
  
    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0,                           j * (PIXEL_SIZE + 1) + 1);
      ctx.lineTo((PIXEL_SIZE + 1) * width + 1, j * (PIXEL_SIZE + 1) + 1);
    }
  
    ctx.stroke();
  };
function getIndex(row, column) {
    return row * width + column;
};
  
function drawPixels() {
    const pixelsPtr = chip8.get_display()
    const pixels = new Uint8Array(memory.buffer, pixelsPtr, width * height);
    ctx.beginPath();
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);

        ctx.fillStyle = pixels[idx] === mod.PixelState.OFF
            ? DEAD_COLOR
            : ALIVE_COLOR;

        ctx.fillRect(
            col * (PIXEL_SIZE + 1) + 1,
            row * (PIXEL_SIZE + 1) + 1,
            PIXEL_SIZE,
            PIXEL_SIZE
        );
        }
    }

    ctx.stroke();
};
function render() {
    for(let i=0; i < 10; i++){
        chip8.tick();
    }
    drawGrid();
    drawPixels();
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
   console.log(memory)
   let rom = await loadRom();
   chip8 = await initChip8(rom);
   render()
}
start().then().catch(console.error);