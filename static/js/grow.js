"use strict";

const kMaxU16 = 0xffff;
const kMaxNumObjects = 1 << 16;
const kMaxTtl = 1 << 9;

const kGrowthMagnitude = 1;
const kSplitMagnitude = 1;

const kSignalAttack = 1 << 11;
// It's *much* easier for cells to grow longer than it is for them to split.
const kGrowSignalThresh = 1 << 15;
const kSplitSignalThresh = 1 << 11;
const kMaxSignalLog = Math.log(kMaxU16);
const kSignalSquareSideLen = 100;

class AbtractGlWrapper {
    constructor(gl, vertexShader, fragmentShader) {
        this.gl = gl;
        if (!gl) {
            return;
        }

        const vertexShaderObj = AbtractGlWrapper.#createShader(gl, gl.VERTEX_SHADER, vertexShader);
        const fragmentShaderObj = AbtractGlWrapper.#createShader(gl, gl.FRAGMENT_SHADER, fragmentShader);
        const program = AbtractGlWrapper.#createProgram(gl, vertexShaderObj, fragmentShaderObj);

        this.vao = gl.createVertexArray();
        this.program = program;
    }

    renderTimeConfigShaderParams() { }

    isEnabled() {
        return this.gl !== null;
    }

    render(numObjects) {
        const gl = this.gl;

        gl.useProgram(this.program);
        gl.bindVertexArray(this.vao);

        this.renderTimeConfigShaderParams(numObjects);
    }

    configureShaderParam(name, type, numPerVertex, glBuffer, array) {
        const gl = this.gl;

        gl.bindBuffer(gl.ARRAY_BUFFER, glBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, array, gl.STATIC_DRAW);

        const paramIndex = gl.getAttribLocation(this.program, name);
        gl.vertexAttribPointer(
            paramIndex,
            /*size=*/numPerVertex,
            type,
            /*normalize=*/false,
            /*stride=*/0,
            /*offset=*/0,
        );
        gl.enableVertexAttribArray(paramIndex);
    }

    static #createShader(gl, type, source) {
        const shader = gl.createShader(type);
        gl.shaderSource(shader, source);
        gl.compileShader(shader);
        const success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);
        if (!success) {
            const log = gl.getShaderInfoLog(shader);
            gl.deleteShader(shader);
            throw new Error("createShader() failed:\n" + log);
        }
        return shader;
    }

    static #createProgram(gl, vertexShader, fragmentShader) {
        const program = gl.createProgram();
        gl.attachShader(program, vertexShader);
        gl.attachShader(program, fragmentShader);
        gl.linkProgram(program);
        const success = gl.getProgramParameter(program, gl.LINK_STATUS);
        if (!success) {
            const log = gl.getProgramInfoLog(program);
            gl.deleteProgram(program);
            throw new Error("createProgram() failed:\n" + log);
        }
        return program;
    }
}

class GlPlantCells extends AbtractGlWrapper {
    static #kNumPositionFields = 4; // x1, y1, x2, y2

    #positionArray = new Float32Array(kMaxNumObjects * GlPlantCells.#kNumPositionFields);
    #ttlArray = new Float32Array(kMaxNumObjects * 2);
    #positionBuffer;
    #ttlBuffer;
    #baseColorVec3 = [0, 0, 0];

    constructor(gl) {
        super(gl, GlPlantCells.#kVertexShaderSource, GlPlantCells.#kFragmentShaderSource);
        this.#positionBuffer = gl.createBuffer();
        this.#ttlBuffer = gl.createBuffer();
    }

    setBaseColor(baseColorVec3) {
        this.#baseColorVec3 = baseColorVec3;
    }

    renderTimeConfigShaderParams(numObjects) {
        const baseColorVec3 = this.#baseColorVec3;
        const uBaseColorIndex = this.gl.getUniformLocation(this.program, "uBaseColor");
        this.gl.uniform3f(uBaseColorIndex, baseColorVec3[0], baseColorVec3[1], baseColorVec3[2]);

        // Connect `positionArray` to the `vertexPos` shader parameter.
        this.configureShaderParam("vertexPos", this.gl.FLOAT, 2, this.#positionBuffer, this.#positionArray);
        this.configureShaderParam("vertexTtl", this.gl.FLOAT, 1, this.#ttlBuffer, this.#ttlArray);
        this.gl.drawArrays(this.gl.LINES, /*first=*/0, /*count=*/numObjects * 2);
    }

    setCell(i, cell) {
        const positionArray = this.#positionArray;
        const ttlArray = this.#ttlArray;

        positionArray[GlPlantCells.#kNumPositionFields * i + 0] = cell.x1;
        positionArray[GlPlantCells.#kNumPositionFields * i + 1] = cell.y1;
        positionArray[GlPlantCells.#kNumPositionFields * i + 2] = cell.x2;
        positionArray[GlPlantCells.#kNumPositionFields * i + 3] = cell.y2;

        ttlArray[2 * i + 0] = cell.ttl;
        ttlArray[2 * i + 1] = cell.ttl;
    }

    static #kVertexShaderSource = `#version 300 es

in vec2 vertexPos;
in float vertexTtl;

out float ttl;
out vec2 pos;

void main() {
  gl_Position = vec4(vertexPos[0] / 500.0 - 1.0,
                     (2.0 - vertexPos[1] / 500.0) - 1.0,
                     0,
                     1);
  ttl = vertexTtl;
  pos = vec2(gl_Position[0], gl_Position[1]);
}
`;

    static #kFragmentShaderSource = `#version 300 es

precision highp float;

uniform vec3 uBaseColor;

in float ttl;
in vec2 pos;
out vec4 outColor;

void main() {
  const float kMaxTtl = ${kMaxTtl.toFixed(3)};

  float scalar = (ttl * ttl) / (kMaxTtl * kMaxTtl);

  // Over a line's lifetime, the ttlScaled decreases from a maximum of 1 to a
  // minimum of 0.

  float r = uBaseColor[0] / 255.0 * scalar;
  float g = uBaseColor[1] / 255.0 * scalar + ((pos[0] + 1.0) / 2.0) * 0.8;
  float b = uBaseColor[2] / 255.0 * scalar + ((pos[1] + 1.0) / 2.0) * 0.8;

  r = max(min(r, 1.0), 0.001);
  g = max(min(g, 1.0), 0.001);
  b = max(min(b, 1.0), 0.001);

  outColor = vec4(r, g, b, 1);
}
`;
}

class GlPlantSignals extends AbtractGlWrapper {
    #isCleared = false;
    #positionArray = new Float32Array(kSignalSquareSideLen ** 2 * 6); // (x, y) coordinates for each triangle
    #signalsArray = new Float32Array(kSignalSquareSideLen ** 2 * 3);
    #positionBuffer;
    #signalsBuf;

    constructor(gl) {
        super(gl, GlPlantSignals.#kVertexShaderSource, GlPlantSignals.#kFragmentShaderSource);

        this.#positionBuffer = gl.createBuffer();
        this.#signalsBuf = gl.createBuffer();
    }

    renderTimeConfigShaderParams(numObjects) {
        this.configureShaderParam("vertexPos", this.gl.FLOAT, 2, this.#positionBuffer, this.#positionArray);
        this.configureShaderParam("vertexSignal", this.gl.FLOAT, 1, this.#signalsBuf, this.#signalsArray);
        this.gl.drawArrays(this.gl.TRIANGLES, /*first=*/0, /*count=*/numObjects * 3);
    }

    setSignal(i, x, y, signalLogScaled) {
        this.#isCleared = false;

        const positionArray = this.#positionArray;
        const signalsArray = this.#signalsArray;

        positionArray[6 * i + 0] = x;
        positionArray[6 * i + 1] = y;
        positionArray[6 * i + 2] = x + 1;
        positionArray[6 * i + 3] = y;
        positionArray[6 * i + 4] = x;
        positionArray[6 * i + 5] = y + 1;

        signalsArray[3 * i + 0] = signalLogScaled;
        signalsArray[3 * i + 1] = signalLogScaled;
        signalsArray[3 * i + 2] = signalLogScaled;
    }

    clear() {
        if (this.#isCleared) {
            return;
        }
        this.#positionArray.forEach((_, i) => { this.#positionArray[i] = 0; });
        this.#signalsArray.forEach((_, i) => { this.#signalsArray[i] = 0; });
        this.#isCleared = true;
    }

    static #kVertexShaderSource = `#version 300 es

in vec2 vertexPos;
in float vertexSignal;

out float signal;

void main() {
  gl_Position = vec4((vertexPos[0] / 100.0) * 2.0 - 1.0,
                     (1.0 - vertexPos[1] / 100.0) * 2.0 - 1.0,
                     0,
                     1);
  signal = vertexSignal;
}
`;

    static #kFragmentShaderSource = `#version 300 es

precision highp float;

in float signal;
out vec4 outColor;

void main() {
  outColor = vec4(signal, 0, 0.25, 1);
}
`;
}

class PlantCell {
    constructor(ctx2d, adjustableVariables, angle, r, g, b, x1, y1, x2, y2) {
        this.ttl = kMaxTtl;

        this.ctx2d = ctx2d;
        this.adjustableVariables = adjustableVariables;
        this.angle = angle;
        this.r = r;
        this.g = g;
        this.b = b;
        this.x1 = x1;
        this.y1 = y1;
        this.x2 = x2;
        this.y2 = y2;
    }

    draw2d() {
        const brightness = this.ttl ** 2 / kMaxTtl ** 2;
        const ctx2d = this.ctx2d;
        const r = Math.floor(this.r * brightness);
        const g = Math.floor(this.g * brightness);
        const b = Math.floor(this.b * brightness);
        const strokeStyle = `rgb(${r} ${g} ${b})`;
        ctx2d.strokeStyle = strokeStyle;

        ctx2d.lineWidth = 2;

        ctx2d.beginPath();
        ctx2d.moveTo(this.x1, this.y1);
        ctx2d.lineTo(this.x2, this.y2);
        ctx2d.closePath();
        ctx2d.stroke();
    }

    // Makes this individual cell either grow or split. The `cells` parameter
    // is a reference to an array of other cells. Splitting will insert two new
    // instances into the `cells` array.
    act(cells, signalMatrix) {
        if (Math.random() < this.adjustableVariables.growChance) {
            const signalMatrixIndex = PlantCell.getIsOccupiedIndex(this.x2, this.y2);
            if (signalMatrix[signalMatrixIndex] <= kGrowSignalThresh) {
                this.x2 += kGrowthMagnitude * Math.cos(this.angle);
                this.y2 += kGrowthMagnitude * Math.sin(this.angle);

                if (this.isInBounds()) {
                    // Increase the signal at the new endpoint.
                    console.assert(signalMatrix[signalMatrixIndex] <= kMaxU16);
                    if (signalMatrix[signalMatrixIndex] <= kMaxU16 - kSignalAttack) {
                        signalMatrix[signalMatrixIndex] += kSignalAttack;
                    }
                } else {
                    this.ttl = 0;
                }
            }
        }

        if (Math.random() < this.adjustableVariables.splitChance) {
            // Choose the child's angle based on our angle.
            const maxTurn = this.adjustableVariables.maxTurn / 360 * 2 * Math.PI;
            const newAngle = this.angle + (2 * Math.random() - 1) * maxTurn;

            const child = new PlantCell(
                this.ctx2d,
                this.adjustableVariables,
                newAngle,
                /*r=*/ 0,
                /*g=*/ 255,
                /*b=*/ 0,
                /*x1=*/ this.x2,
                /*y1=*/ this.y2,
                /*x2=*/ this.x2 + kSplitMagnitude * Math.cos(newAngle),
                /*y2=*/ this.y2 + kSplitMagnitude * Math.sin(newAngle),
            );

            const signalMatrixIndex = PlantCell.getIsOccupiedIndex(child.x2, child.y2);
            if (child.isInBounds() && signalMatrix[signalMatrixIndex] <= kSplitSignalThresh) {
                cells.push(child);

                if (signalMatrix[signalMatrixIndex] <= kMaxU16 - kSignalAttack) {
                    signalMatrix[signalMatrixIndex] += kSignalAttack;
                }
            }
        }
    }

    static getIsOccupiedIndex(x, y) {
        const clampTo = (lo, hi, a) => {
            if (a < lo) return lo;
            if (a > hi) return hi;
            return a;
        }
        const clampedX = clampTo(0, 1000, x);
        const clampedY = clampTo(0, 1000, y);

        // Convert coordinates to a `kSignalSquareSideLen**2` grid.
        const scaledX = Math.floor(clampedX / 10);
        const scaledY = Math.floor(clampedY / 10);

        const index = scaledX * kSignalSquareSideLen + scaledY;
        const clampedIndex = clampTo(0, kSignalSquareSideLen ** 2, index);
        return clampedIndex;
    }

    isInBounds() {
        return (this.x1 >= 0 && this.x1 <= 1000) &&
            (this.y1 >= 0 && this.y1 <= 1000) &&
            (this.x2 >= 0 && this.x2 <= 1000) &&
            (this.y2 >= 0 && this.y2 <= 1000);
    }
}

// Parse a hex string of the form "#[0-9]{6}" in regex notation.
function parseHexString(s) {
    if (s.length === 0) {
        return null;
    }
    if (s[0] !== '#') {
        return null;
    }
    const hexString = s.slice(1);
    if (hexString.length % 2 != 0) {
        return null;
    }
    const bytes = [];
    for (let i=0; i < hexString.length; i += 2) {
        const hexByte = hexString.slice(i, i+2);
        const byteValue = parseInt(hexByte, 16);
        bytes.push(byteValue);
    }
    return bytes;
}

class AdjustableVariables {
    resetSignalsRequested = false;
    resetUniverseRequested = false;
    pauseUniverseRequested = false;
    baseColorBytes;

    constructor() {
        const obj = this;
        const divisors = {
            growChance: 100,
            splitChance: 100,
            maxTurn: 1,
            dormancyAgePercentile: 100,
            signalDecayRate: 1,
        };
        for (const attrName in divisors) {
            const rangeElem = document.getElementById(attrName + "Range");
            console.assert(rangeElem !== null);

            const labelElem = document.getElementById(attrName + "Label");
            console.assert(labelElem !== null);

            const updateFunc = () => {
                console.info(`Updated value for ${attrName}: ${rangeElem.value}`);
                labelElem.innerText = rangeElem.value;
                obj[attrName] = parseFloat(rangeElem.value) / divisors[attrName];
            };
            updateFunc();
            rangeElem.oninput = updateFunc;
        }

        for (const attrName of ["showSignals"]) {
            const checkboxElem = document.getElementById(attrName + "Checkbox");
            console.assert(checkboxElem !== null);
            const updateFunc = () => {
                console.log(checkboxElem);
                console.info(`Updated value for ${attrName}: ${checkboxElem.checked}`);
                obj[attrName] = checkboxElem.checked;
            };
            updateFunc();
            checkboxElem.oninput = updateFunc;
        }

        const resetSignalsButton = document.getElementById("resetSignalsButton");
        console.assert(resetSignalsButton);
        resetSignalsButton.addEventListener("click", () => {
            this.resetSignalsRequested = true;
        });

        const resetUniverseButton = document.getElementById("resetUniverseButton");
        console.assert(resetUniverseButton);
        resetUniverseButton.addEventListener("click", () => {
            this.resetUniverseRequested = true;
        });

        const pauseUniverseButton = document.getElementById("pauseUniverseButton");
        console.assert(pauseUniverseButton);
        pauseUniverseButton.addEventListener("click", () => {
            this.pauseUniverseRequested = !this.pauseUniverseRequested;

            if (!this.pauseUniverseRequested) {
                thunks.startAnimationFunc();
            }
        });

        const baseColorPicker = document.getElementById("baseColorPicker");
        console.assert(baseColorPicker);
        baseColorPicker.addEventListener("input", (event) => {
            console.log("New color:", baseColorPicker.value)
            this.baseColorBytes = parseHexString(baseColorPicker.value);
        });
        const originalColorValue = baseColorPicker.value;
        this.baseColorBytes = parseHexString(originalColorValue);

        const resetColorButton = document.getElementById("resetColorButton");
        console.assert(resetColorButton);
        resetColorButton.addEventListener("click", () => {
            baseColorPicker.value = originalColorValue;
            this.baseColorBytes = parseHexString(originalColorValue);
        });
    }
}

function buildThunks() {
    const paramsString = window.location.search;
    const searchParams = new URLSearchParams(paramsString);
    const renderParam = searchParams.get("render");

    let forceBothCanvases = false;
    let forceCanvas2d = false;
    switch (renderParam) {
        case null:
            break;
        case "both":
            forceBothCanvases = true;
            break;
        case "2d":
            forceCanvas2d = true;
            break;
        default:
            console.error("Unknown ?render value: " + renderParam);
    }

    const canvas2d = document.getElementById("game");
    let ctx2d = canvas2d.getContext("2d");

    const canvasGl = document.getElementById("gameGl");
    let gl = canvasGl.getContext("webgl2", { antialias: true });

    if (forceBothCanvases) {
        canvas2d.style.setProperty("display", "block");
        canvasGl.style.setProperty("display", "block");
        canvasGl.style.setProperty("border", "1px solid red");
    } else if (!gl || forceCanvas2d) {
        console.log("Using Canvas 2D");
        canvas2d.style.setProperty("display", "block");
        canvasGl.style.setProperty("display", "none");
        gl = null;
    } else {
        console.log("Using WebGL");
        canvas2d.style.setProperty("display", "none");
        canvasGl.style.setProperty("display", "block");
        ctx2d = null;
    }

    const glCells = new GlPlantCells(gl);
    const glSignals = new GlPlantSignals(gl);

    const adjustableVariables = new AdjustableVariables();
    const signalMatrix = new Uint16Array(kSignalSquareSideLen ** 2);

    let cells = [];

    function animateFunc() {
        if (adjustableVariables.pauseUniverseRequested) {
            return;
        }

        if (adjustableVariables.resetSignalsRequested) {
            adjustableVariables.resetSignalsRequested = false;

            for (let i = 0; i < signalMatrix.length; ++i) {
                signalMatrix[i] = 0;
            }
        }
        if (adjustableVariables.resetUniverseRequested) {
            adjustableVariables.resetUniverseRequested = false;

            for (let i = 0; i < signalMatrix.length; ++i) {
                signalMatrix[i] = 0;
            }

            cells = [];
        }

        // Maybe initialize.
        if (cells.length === 0) {
            const first = new PlantCell(
                ctx2d,
                adjustableVariables,
                /*angle=*/(3 * Math.PI) / 2,
                /*r=*/ 0,
                /*g=*/ 255,
                /*b=*/ 0,
                /*x1=*/ 500,
                /*y1=*/ 1000,
                /*x2=*/ 500,
                /*y2=*/ 980,
            );

            for (let i = 0; i < signalMatrix.length; ++i) {
                signalMatrix[i] = 0;
            }

            signalMatrix[PlantCell.getIsOccupiedIndex(first.x2, first.y2)] = 1;

            cells = [first];
        }

        // Decay the signal in `signalMatrix`.
        for (let i = 0; i < signalMatrix.length; ++i) {
            const decayRate = Math.floor(2 ** adjustableVariables.signalDecayRate);
            if (signalMatrix[i] > decayRate) {
                signalMatrix[i] -= decayRate;
            } else {
                signalMatrix[i] = 0;
            }
        }

        if (ctx2d) {
            ctx2d.clearRect(0, 0, 1000, 1000);
        }

        if (adjustableVariables.showSignals) {
            for (let i = 0; i < signalMatrix.length; ++i) {
                const x = Math.floor(i / kSignalSquareSideLen);
                const y = i % kSignalSquareSideLen;

                console.assert(x >= 0);
                console.assert(x <= kSignalSquareSideLen);
                console.assert(y >= 0);
                console.assert(y <= kSignalSquareSideLen);

                const signal = signalMatrix[i];
                const signalLog = signal === 0 ? 0 : Math.log(signal);
                const signalLogScaled = signalLog / kMaxSignalLog;

                if (ctx2d) {
                    const scaledR = Math.floor(255 * signalLogScaled);
                    const scaledB = Math.floor(255 * signalLogScaled);
                    const color = `rgb(${scaledR} 0 ${scaledB} / 90%)`;

                    ctx2d.fillStyle = color;
                    ctx2d.fillRect(x * 10, y * 10, 10, 10);
                }

                if (glSignals.isEnabled()) {
                    glSignals.setSignal(i, x, y, signalLogScaled);
                }
            }
        } else if (gl) {
            glSignals.clear();
        }

        // The youngest cells are always at the end of the array.
        const firstActiveCellIndex = Math.floor((1 - adjustableVariables.dormancyAgePercentile) * cells.length);
        cells.slice(firstActiveCellIndex).forEach((c) => c.act(cells, signalMatrix));
        cells = cells.filter((b) => b.ttl > 0 && b.isInBounds());
        cells = cells.slice(0, kMaxNumObjects);

        glCells.setBaseColor(adjustableVariables.baseColorBytes);

        cells.forEach((b, i) => {
            b.ttl--;

            if (glCells.isEnabled()) {
                glCells.setCell(i, b)
            }

            if (ctx2d && b.ttl % 16 === 0) {
                b.r = Math.min(64, (b.r + 2));
                b.g = Math.max(0, (b.g - 12));
                b.b = Math.min(64, (b.b + 1));
            }
        });

        if (gl) {
            gl.clearColor(0, 0, 0, 0); // RGBA
            gl.clear(gl.COLOR_BUFFER_BIT);

            // Signals should be rendered before the lines for visual appeal.
            if (adjustableVariables.showSignals) {
                glSignals.render(kSignalSquareSideLen ** 2);
            }

            glCells.render(cells.length);
        }


        if (ctx2d) {
            cells.forEach((p) => { p.draw2d(); });
        }

        window.requestAnimationFrame(animateFunc);
    }

    function startAnimationFunc() {
        window.requestAnimationFrame(animateFunc);
    }

    function onKeyDownFunc(event) {
        console.log(event);

        if (event.code === "Space") {
            event.preventDefault();

            adjustableVariables.pauseUniverseRequested =
                !adjustableVariables.pauseUniverseRequested;

            if (!adjustableVariables.pauseUniverseRequested) {
                window.requestAnimationFrame(startAnimationFunc);
            }
        }
    }

    return {
        startAnimationFunc,
        onKeyDownFunc,
    };
}

let thunks = buildThunks();
window.onload = thunks.startAnimationFunc;
window.onkeydown = thunks.onKeyDownFunc;
