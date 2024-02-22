import { SnakeGame, Cell, Direction } from "wasm-snake";
import { memory } from "wasm-snake/wasm_snake_bg.wasm";

const CELL_SIZE = 20; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const SNAKE_COLOR = "#6B8E23";
const FRUIT_COLOR = "#FF0000"
const WIDTH = 20
const HEIGHT = 20

const game = SnakeGame.new(WIDTH,HEIGHT);


// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("snake-canvas");
canvas.height = (CELL_SIZE + 1) * WIDTH + 1;
canvas.width = (CELL_SIZE + 1) * HEIGHT + 1;

const ctx = canvas.getContext('2d');

const score = document.getElementById('score-count')

const gameSpeed = 100
const renderLoop = (speedTime) => {
  let speed = speedTime
  let lastScore = game.get_score()
  setTimeout(() => {
    game.update();

    drawGrid();
    drawCells();

    if (game.is_game_over()) {
      document.querySelector("#game-over").classList.remove('hidden')
      document.querySelector("#game-over").classList.add('display-center')
    } else {
      score.innerText = game.get_score()
      if (game.get_score() !== 0 && game.get_score() !== lastScore && game.get_score() % 10 === 0 ) {
        speed = speed - 10
    }
      requestAnimationFrame(() => renderLoop(speed));
    }
  }, speedTime)
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = DEAD_COLOR;

  // Vertical lines.
  for (let i = 0; i <= WIDTH; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * HEIGHT + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= HEIGHT; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * WIDTH + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * WIDTH + column;
};

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << (n % 8);
  return (arr[byte] & mask) === mask;
};

const drawCells = () => {
  const cellsPtr = game.get_cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, WIDTH * HEIGHT);

  ctx.beginPath();

  for (let row = 0; row < HEIGHT; row++) {
    for (let col = 0; col < WIDTH; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Deactivated
        ? DEAD_COLOR
        : game.is_snake_cell(idx) ? SNAKE_COLOR : FRUIT_COLOR;

      ctx.fillRect(
        row * (CELL_SIZE + 1) + 1,
        col * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

window.addEventListener("keydown", (e) => {
  if (e.key === 'ArrowUp') {
    e.preventDefault()
    game.set_direction(Direction.Up)
  }
  if (e.key === 'ArrowDown') {
    e.preventDefault()
    game.set_direction(Direction.Down)
  }
  if (e.key === 'ArrowRight') {
    e.preventDefault()
    game.set_direction(Direction.Right)
  }
  if (e.key === 'ArrowLeft') {
    e.preventDefault()
    game.set_direction(Direction.Left)
  }

})

drawGrid();
drawCells();
requestAnimationFrame(() => renderLoop(gameSpeed));
