const height = 32;
const width = 64;
const PIXEL_SIZE = 8;
const GRID_COLOR = "#CCCCCC";
const OFF_COLOR = "#000000";
const ON_COLOR = "#48ff00";
const canvas = document.getElementById("chip8-screen");

const ctx = canvas.getContext('2d');
canvas.height = (PIXEL_SIZE + 1) * height + 1;
canvas.width = (PIXEL_SIZE + 1) * width + 1;


export function drawGrid() {
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
  
export function drawPixels(chip8, memory, mod) {
    const pixelsPtr = chip8.get_display()
    const pixels = new Uint8Array(memory.buffer, pixelsPtr, width * height);
    ctx.beginPath();
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);

        ctx.fillStyle = pixels[idx] === mod.PixelState.OFF
            ? OFF_COLOR
            : ON_COLOR;

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

export function updateRegisters(chip8) {
  for(let i = 0; i < 16; i++){
    let regVal = chip8.get_register(i);
    let regComp = document.getElementById(`${i}`)
    console.log(regComp.childNodes)
    let val = regComp.getElementsByClassName('reg-val')[0].getElementsByTagName('p')[0]
    val.textContent = regVal
  }
}