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
const commandHistory = [];
let commandHistoryIndex = -1;
let commandDraft = '';
const LIVE_STATS_INTERVAL_MS = 1000;
let lastLiveStatsAtMs = -LIVE_STATS_INTERVAL_MS;
let isPaused = false;
let extinctionOverlayDismissed = false;
const DEFAULT_FOV_DEG = 225.0;

const DEFAULT_COMMANDS = {
  prey: 40,
  pred: 6,
  food: 60,
  preyNeurons: 9,
  predNeurons: 9,
  preyPhotoreceptors: 9,
  predPhotoreceptors: 9,
  preyFovDeg: DEFAULT_FOV_DEG,
  predFovDeg: DEFAULT_FOV_DEG,
  preySpeedMul: 1.0,
  predSpeedMul: 1.0,
};
const commandConfig = { ...DEFAULT_COMMANDS };

const genValue = document.getElementById('genValue');
const preyMin = document.getElementById('preyMin');
const preyMax = document.getElementById('preyMax');
const preyAvg = document.getElementById('preyAvg');
const preyDead = document.getElementById('preyDead');
const predMin = document.getElementById('predMin');
const predMax = document.getElementById('predMax');
const predAvg = document.getElementById('predAvg');
const historyList = document.getElementById('statsHistory');
const commandInput = document.getElementById('commandInput');
const commandStatus = document.getElementById('commandStatus');
const cmdPauseBtn = document.getElementById('cmdPauseBtn');
const cmdTrain1Btn = document.getElementById('cmdTrain1Btn');
const cmdTrain100Btn = document.getElementById('cmdTrain100Btn');
const cmdResetBtn = document.getElementById('cmdResetBtn');
const cfgPrey = document.getElementById('cfgPrey');
const cfgPred = document.getElementById('cfgPred');
const cfgFood = document.getElementById('cfgFood');
const cfgPreyN = document.getElementById('cfgPreyN');
const cfgPredN = document.getElementById('cfgPredN');
const cfgPreyP = document.getElementById('cfgPreyP');
const cfgPredP = document.getElementById('cfgPredP');
const cfgPreyFov = document.getElementById('cfgPreyFov');
const cfgPredFov = document.getElementById('cfgPredFov');
const cfgPreySpeed = document.getElementById('cfgPreySpeed');
const cfgPredSpeed = document.getElementById('cfgPredSpeed');
const extinctionOverlay = document.getElementById('extinctionOverlay');
const extinctionResetBtn = document.getElementById('extinctionResetBtn');

function formatFitness(value) {
  return Number(value).toFixed(2);
}

function setCommandStatus(text) {
  if (!commandStatus) {
    return;
  }
  commandStatus.textContent = text;
}

function setExtinctionOverlayVisible(isVisible) {
  if (!extinctionOverlay) {
    return;
  }
  extinctionOverlay.classList.toggle('is-visible', isVisible);
  extinctionOverlay.setAttribute('aria-hidden', String(!isVisible));
}

function updatePauseButtonLabel() {
  if (!cmdPauseBtn) {
    return;
  }
  cmdPauseBtn.textContent = isPaused ? 'Resume' : 'Pause';
}

function renderCommandConfig() {
  if (cfgPrey) cfgPrey.textContent = String(commandConfig.prey);
  if (cfgPred) cfgPred.textContent = String(commandConfig.pred);
  if (cfgFood) cfgFood.textContent = String(commandConfig.food);
  if (cfgPreyN) cfgPreyN.textContent = String(commandConfig.preyNeurons);
  if (cfgPredN) cfgPredN.textContent = String(commandConfig.predNeurons);
  if (cfgPreyP) cfgPreyP.textContent = String(commandConfig.preyPhotoreceptors);
  if (cfgPredP) cfgPredP.textContent = String(commandConfig.predPhotoreceptors);
  if (cfgPreyFov) cfgPreyFov.textContent = Number(commandConfig.preyFovDeg).toFixed(1);
  if (cfgPredFov) cfgPredFov.textContent = Number(commandConfig.predFovDeg).toFixed(1);
  if (cfgPreySpeed) cfgPreySpeed.textContent = Number(commandConfig.preySpeedMul).toFixed(2);
  if (cfgPredSpeed) cfgPredSpeed.textContent = Number(commandConfig.predSpeedMul).toFixed(2);
}

function parsePositiveInt(value) {
  const n = Number(value);
  if (!Number.isInteger(n) || n <= 0) {
    return null;
  }
  return n;
}

function parsePositiveFloat(value) {
  const n = Number(value);
  if (!Number.isFinite(n) || n <= 0) {
    return null;
  }
  return n;
}

function degToRad(deg) {
  return (deg * Math.PI) / 180.0;
}

function applyResetParams(paramTokens) {
  let changed = false;
  for (const token of paramTokens) {
    const eqIdx = token.indexOf('=');
    if (eqIdx <= 0) {
      continue;
    }

    const rawKey = token.slice(0, eqIdx).toLowerCase();
    const value = token.slice(eqIdx + 1);
    let key = rawKey;

    // advanced syntax prefixes: i:key=123 / f:key=0.5
    if ((key.startsWith('i:') || key.startsWith('f:')) && key.length > 2) {
      key = key.slice(2);
    }

    if (key === 'animals' || key === 'a') {
      return 'Use prey=<count> and pred=<count> instead of animals.';
    }

    if (key === 'prey') {
      const parsed = parsePositiveInt(value);
      if (parsed === null) {
        return `Invalid prey value: ${value}`;
      }
      commandConfig.prey = parsed;
      changed = true;
      continue;
    }

    if (key === 'pred' || key === 'predators') {
      const parsed = parsePositiveInt(value);
      if (parsed === null) {
        return `Invalid pred value: ${value}`;
      }
      commandConfig.pred = parsed;
      changed = true;
      continue;
    }

    if (key === 'foods') {
      return 'Use food=<count> instead of foods.';
    }

    if (key === 'f' || key === 'food') {
      const parsed = parsePositiveInt(value);
      if (parsed === null) {
        return `Invalid food value: ${value}`;
      }
      commandConfig.food = parsed;
      changed = true;
      continue;
    }

    if (key === 'n' || key === 'neurons') {
      return 'Use prey-n=<count> and pred-n=<count> instead of neurons.';
    }

    if (key === 'prey-n' || key === 'prey-neurons') {
      const parsed = parsePositiveInt(value);
      if (parsed === null) {
        return `Invalid prey-n value: ${value}`;
      }
      commandConfig.preyNeurons = parsed;
      changed = true;
      continue;
    }

    if (key === 'pred-n' || key === 'pred-neurons') {
      const parsed = parsePositiveInt(value);
      if (parsed === null) {
        return `Invalid pred-n value: ${value}`;
      }
      commandConfig.predNeurons = parsed;
      changed = true;
      continue;
    }

    if (key === 'p' || key === 'prey-p' || key === 'prey-photoreceptors') {
      const parsed = parsePositiveInt(value);
      if (parsed === null) {
        return `Invalid prey-p value: ${value}`;
      }
      commandConfig.preyPhotoreceptors = parsed;
      changed = true;
      continue;
    }

    if (key === 'pred-p' || key === 'pred-photoreceptors') {
      const parsed = parsePositiveInt(value);
      if (parsed === null) {
        return `Invalid pred-p value: ${value}`;
      }
      commandConfig.predPhotoreceptors = parsed;
      changed = true;
      continue;
    }

    if (key === 'prey-fov') {
      const parsed = parsePositiveFloat(value);
      if (parsed === null) {
        return `Invalid prey-fov value: ${value}`;
      }
      commandConfig.preyFovDeg = parsed;
      changed = true;
      continue;
    }

    if (key === 'pred-fov') {
      const parsed = parsePositiveFloat(value);
      if (parsed === null) {
        return `Invalid pred-fov value: ${value}`;
      }
      commandConfig.predFovDeg = parsed;
      changed = true;
      continue;
    }

    if (key === 'prey-speed') {
      const parsed = parsePositiveFloat(value);
      if (parsed === null) {
        return `Invalid prey-speed value: ${value}`;
      }
      commandConfig.preySpeedMul = parsed;
      changed = true;
      continue;
    }

    if (key === 'pred-speed') {
      const parsed = parsePositiveFloat(value);
      if (parsed === null) {
        return `Invalid pred-speed value: ${value}`;
      }
      commandConfig.predSpeedMul = parsed;
      changed = true;
      continue;
    }
  }

  if (!changed && paramTokens.length > 0) {
    return 'No supported parameters were recognized.';
  }

  const stats = simulation.reset(
    commandConfig.prey,
    commandConfig.pred,
    commandConfig.food,
    commandConfig.preyNeurons,
    commandConfig.predNeurons,
    commandConfig.preyPhotoreceptors,
    commandConfig.predPhotoreceptors,
    degToRad(commandConfig.preyFovDeg),
    degToRad(commandConfig.predFovDeg),
    commandConfig.preySpeedMul,
    commandConfig.predSpeedMul,
  );
  statsHistory.length = 0;
  historyList.textContent = '';
  renderCommandConfig();
  updateStatsPanel(stats, { recordHistory: false });
  return null;
}

function runCommand(line) {
  const trimmed = line.trim();
  if (!trimmed) {
    return;
  }

  const parts = trimmed.split(/\s+/);
  const cmd = parts[0].toLowerCase();
  const args = parts.slice(1);

  if (cmd === 'p' || cmd === 'pause') {
    isPaused = !isPaused;
    updatePauseButtonLabel();
    setCommandStatus(isPaused ? 'Simulation paused.' : 'Simulation resumed.');
    return;
  }

  if (cmd === 't' || cmd === 'train') {
    if (args.length > 1) {
      setCommandStatus('Usage: train [generations]');
      return;
    }
    const howMany = args.length === 0 ? 1 : parsePositiveInt(args[0]);
    if (howMany === null) {
      setCommandStatus(`Invalid train value: ${args[0]}`);
      return;
    }
    for (let i = 0; i < howMany; i++) {
      updateStatsPanel(simulation.fast_forward());
    }
    updateStatsPanel(simulation.current_stats(), { recordHistory: false });
    lastLiveStatsAtMs = performance.now();
    setCommandStatus(`Trained ${howMany} generation${howMany === 1 ? '' : 's'}.`);
    return;
  }

  if (cmd === 'r' || cmd === 'reset') {
    const err = applyResetParams(args);
    if (err) {
      setCommandStatus(err);
      return;
    }
    setCommandStatus('Simulation reset.');
    return;
  }

  setCommandStatus(`Unknown command: ${cmd}`);
}

updatePauseButtonLabel();
renderCommandConfig();

if (commandInput) {
  commandInput.addEventListener('keydown', function(event) {
    if (event.key === 'Enter') {
      const command = commandInput.value.trim();
      if (!command) {
        event.preventDefault();
        return;
      }
      runCommand(command);
      commandHistory.push(command);
      commandHistoryIndex = commandHistory.length;
      commandInput.value = '';
      event.preventDefault();
      return;
    }

    if (event.key === 'ArrowUp') {
      if (commandHistory.length === 0) {
        return;
      }
      if (commandHistoryIndex === commandHistory.length) {
        commandDraft = commandInput.value;
      }
      commandHistoryIndex = Math.max(0, commandHistoryIndex - 1);
      commandInput.value = commandHistory[commandHistoryIndex];
      event.preventDefault();
      return;
    }

    if (event.key === 'ArrowDown') {
      if (commandHistory.length === 0) {
        return;
      }
      commandHistoryIndex = Math.min(commandHistory.length, commandHistoryIndex + 1);
      if (commandHistoryIndex === commandHistory.length) {
        commandInput.value = commandDraft;
      } else {
        commandInput.value = commandHistory[commandHistoryIndex];
      }
      event.preventDefault();
    }
  });
}

if (cmdPauseBtn) {
  cmdPauseBtn.addEventListener('click', function() {
    runCommand('pause');
  });
}

if (cmdTrain1Btn) {
  cmdTrain1Btn.addEventListener('click', function() {
    runCommand('train 1');
  });
}

if (cmdTrain100Btn) {
  cmdTrain100Btn.addEventListener('click', function() {
    runCommand('train 100');
  });
}

if (cmdResetBtn) {
  cmdResetBtn.addEventListener('click', function() {
    commandConfig.prey = DEFAULT_COMMANDS.prey;
    commandConfig.pred = DEFAULT_COMMANDS.pred;
    commandConfig.food = DEFAULT_COMMANDS.food;
    commandConfig.preyNeurons = DEFAULT_COMMANDS.preyNeurons;
    commandConfig.predNeurons = DEFAULT_COMMANDS.predNeurons;
    commandConfig.preyPhotoreceptors = DEFAULT_COMMANDS.preyPhotoreceptors;
    commandConfig.predPhotoreceptors = DEFAULT_COMMANDS.predPhotoreceptors;
    commandConfig.preyFovDeg = DEFAULT_COMMANDS.preyFovDeg;
    commandConfig.predFovDeg = DEFAULT_COMMANDS.predFovDeg;
    commandConfig.preySpeedMul = DEFAULT_COMMANDS.preySpeedMul;
    commandConfig.predSpeedMul = DEFAULT_COMMANDS.predSpeedMul;
    runCommand('reset');
  });
}

if (extinctionResetBtn) {
  extinctionResetBtn.addEventListener('click', function() {
    extinctionOverlayDismissed = false;
    runCommand('r');
  });
}

if (extinctionOverlay) {
  extinctionOverlay.addEventListener('click', function(event) {
    if (event.target !== extinctionOverlay) {
      return;
    }
    extinctionOverlayDismissed = true;
    setExtinctionOverlayVisible(false);
  });
}

function updateStatsPanel(stats, { recordHistory = true } = {}) {
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

  if (recordHistory) {
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

      const genValueCell = document.createElement('span');
      genValueCell.className = 'num';
      genValueCell.textContent = entry.generation;

      const preyCell = document.createElement('span');
      preyCell.className = 'history-prey num';
      preyCell.textContent = entry.preyAvg;

      const predCell = document.createElement('span');
      predCell.className = 'history-pred num';
      predCell.textContent = entry.predAvg;

      const deathsValue = document.createElement('span');
      deathsValue.className = 'num';
      deathsValue.textContent = entry.preyDead;

      item.appendChild(genValueCell);
      item.appendChild(preyCell);
      item.appendChild(predCell);
      item.appendChild(deathsValue);
      historyList.appendChild(item);
    }
  }
}

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

  if (!isPaused) {
    const stats = simulation.step();
    if (stats) {
      updateStatsPanel(stats);
      // Immediately refresh live stats for the new generation.
      updateStatsPanel(simulation.current_stats(), { recordHistory: false });
      lastLiveStatsAtMs = performance.now();
    } else {
      const now = performance.now();
      if (now - lastLiveStatsAtMs >= LIVE_STATS_INTERVAL_MS) {
        updateStatsPanel(simulation.current_stats(), { recordHistory: false });
        lastLiveStatsAtMs = now;
      }
    }
  }

  const world = simulation.world();
  const preyAlive = world.animals.some((animal) => animal.alive);
  if (preyAlive) {
    extinctionOverlayDismissed = false;
  }
  setExtinctionOverlayVisible(!preyAlive && !extinctionOverlayDismissed);

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

updateStatsPanel(simulation.current_stats(), { recordHistory: false });
redraw();
