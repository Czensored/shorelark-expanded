import * as sim from "../libs/simulation-wasm/pkg/lib_simulation_wasm.js";
const SPRITE_SIZE = 0.012;
const PREDATOR_SPRITE_SIZE = 0.016;

function cssColorFromPackedRGBA(packed) {
  const u = packed >>> 0; // ensure unsigned
  const r = (u >>> 24) & 255;
  const g = (u >>> 16) & 255;
  const b = (u >>> 8) & 255;
  const a = u & 255;
  return `rgba(${r},${g},${b},${a / 255})`;
}

CanvasRenderingContext2D.prototype.drawTriangle =
  function(x, y, size, rotation) {
    this.beginPath();

    this.moveTo(
      x - Math.sin(rotation) * size * 1.5,
      y + Math.cos(rotation) * size * 1.5,
    );

    this.lineTo(
      x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
      y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
    );

    this.lineTo(
      x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
      y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
    );

    this.lineTo(
      x - Math.sin(rotation) * size * 1.5,
      y + Math.cos(rotation) * size * 1.5,
    );

    // this.stroke();
    // this.fillStyle = 'rgb(255, 255, 255)';
    this.fill();
  };

CanvasRenderingContext2D.prototype.drawCircle =
  function(x, y, radius) {
    this.beginPath();

    this.arc(x, y, radius, 0, 2.0 * Math.PI);

    this.fillStyle = 'rgb(255, 255, 255)';
    this.fill();
  };

const simulation = new sim.Simulation();
const statsHistory = [];

const genValue = document.getElementById('genValue');
const preyMin = document.getElementById('preyMin');
const preyMax = document.getElementById('preyMax');
const preyAvg = document.getElementById('preyAvg');
const preyDead = document.getElementById('preyDead');
const predMin = document.getElementById('predMin');
const predMax = document.getElementById('predMax');
const predAvg = document.getElementById('predAvg');
const historyList = document.getElementById('statsHistory');

function formatFitness(value) {
  return Number(value).toFixed(2);
}

function updateStatsPanel(stats) {
  if (!stats) {
    return;
  }

  const generation = Number(stats.generation);
  const preyMinFitness = formatFitness(stats.prey_min_fitness);
  const preyMaxFitness = formatFitness(stats.prey_max_fitness);
  const preyAvgFitness = formatFitness(stats.prey_avg_fitness);
  const preyDeaths = Number(stats.prey_dead);
  const predatorMinFitness = formatFitness(stats.predator_min_fitness);
  const predatorMaxFitness = formatFitness(stats.predator_max_fitness);
  const predatorAvgFitness = formatFitness(stats.predator_avg_fitness);

  genValue.textContent = generation;
  preyMin.textContent = preyMinFitness;
  preyMax.textContent = preyMaxFitness;
  preyAvg.textContent = preyAvgFitness;
  preyDead.textContent = preyDeaths;
  predMin.textContent = predatorMinFitness;
  predMax.textContent = predatorMaxFitness;
  predAvg.textContent = predatorAvgFitness;

  statsHistory.unshift({
    generation,
    preyAvg: preyAvgFitness,
    predAvg: predatorAvgFitness,
    preyDead: preyDeaths,
  });

  historyList.textContent = '';
  for (const entry of statsHistory) {
    const item = document.createElement('li');
    item.className = 'history-entry';

    const genLabel = document.createElement('span');
    genLabel.textContent = 'Gen';

    const genValueCell = document.createElement('span');
    genValueCell.className = 'num';
    genValueCell.textContent = entry.generation;

    const preyCell = document.createElement('span');
    preyCell.className = 'history-prey num';
    preyCell.textContent = entry.preyAvg;

    const predCell = document.createElement('span');
    predCell.className = 'history-pred num';
    predCell.textContent = entry.predAvg;

    const spacer = document.createElement('span');
    spacer.className = 'history-gap';

    const deathsLabel = document.createElement('span');
    deathsLabel.textContent = 'deaths';

    const deathsValue = document.createElement('span');
    deathsValue.className = 'num';
    deathsValue.textContent = entry.preyDead;

    item.appendChild(genLabel);
    item.appendChild(genValueCell);
    item.appendChild(preyCell);
    item.appendChild(predCell);
    item.appendChild(spacer);
    item.appendChild(deathsLabel);
    item.appendChild(deathsValue);
    historyList.appendChild(item);
  }
}

document.getElementById('train').onclick = function() {
  const count = parseInt(document.getElementById('fastForwardCount').value) || 1;
  for (let i = 0; i < count; i++) {
    updateStatsPanel(simulation.fast_forward());
  }
};

const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;
const viewportScale = window.devicePixelRatio || 1;

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;
viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

const ctxt = viewport.getContext('2d');

ctxt.scale(viewportScale, viewportScale);

ctxt.fillStyle = 'rgb(0, 0, 0)';

function redraw() {
  ctxt.clearRect(0, 0, viewportWidth, viewportHeight);

  const stats = simulation.step();
  if (stats) {
    updateStatsPanel(stats);
  }

  const world = simulation.world();

  for (const food of world.foods) {
    ctxt.drawCircle(
      food.x * viewportWidth,
      food.y * viewportHeight,
      (SPRITE_SIZE / 2.0) * viewportWidth,
    );
  }

  for (const animal of world.animals) {
    if (!animal.alive) {
      continue;
    }
    ctxt.fillStyle = cssColorFromPackedRGBA(animal.color);
    ctxt.drawTriangle(
      animal.x * viewportWidth,
      animal.y * viewportHeight,
      SPRITE_SIZE * viewportWidth,
      animal.rotation,
    );
  }

  for (const predator of world.predators) {
    if (!predator.alive) {
      continue;
    }
    ctxt.fillStyle = cssColorFromPackedRGBA(predator.color);
    ctxt.drawTriangle(
      predator.x * viewportWidth,
      predator.y * viewportHeight,
      PREDATOR_SPRITE_SIZE * viewportWidth,
      predator.rotation,
    );
  }

  requestAnimationFrame(redraw);
}

redraw();
