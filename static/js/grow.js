"use strict";

const kMaxU16 = 0xffff;
const kMaxNumObjects = 1 << 17;
const kMaxTtl = 1 << 9;

const kGrowthMagnitude = 1;
const kSplitMagnitude = 2;

const kGrowSignalThresh = 1 << 9;
const kSplitSignalThresh = 1 << 8;
const kSignalAttack = 1 << 4;
const kMaxSignalLog = Math.log(kMaxU16);
const kSignalSquareSideLen = 100;

const kNumPositionFields = 4; // x1, y1, x2, y2

class GlState {
    constructor() {
        const gl = document.getElementById("gameGl").getContext("webgl2");
        if (!gl) {
            return;
        }

        gl.canvas.width = gl.canvas.clientWidth;
        gl.canvas.height = gl.canvas.clientHeight;
        gl.viewport(0, 0, gl.canvas.clientWidth, gl.canvas.clientHeight);

        const vertexShader = GlState.#createShader(gl, gl.VERTEX_SHADER, GlState.#vertexShaderSource);
        const fragmentShader = GlState.#createShader(gl, gl.FRAGMENT_SHADER, GlState.#fragmentShaderSource);
        const program = GlState.#createProgram(gl, vertexShader, fragmentShader);

        const vao = gl.createVertexArray();
        gl.bindVertexArray(vao);

        this.gl = gl;
        this.program = program;
        this.positionBuffer = gl.createBuffer();
        this.ttlBuffer = gl.createBuffer();
        this.positionArray = new Float32Array(kMaxNumObjects * kNumPositionFields);
        this.ttlArray = new Float32Array(kMaxNumObjects * 2);
    }

    isEnabled() {
        return this.gl !== null;
    }

    render(numLines) {
        const gl = this.gl;

        gl.clearColor(0, 0, 0, 0); // RGBA
        gl.clear(gl.COLOR_BUFFER_BIT);

        gl.useProgram(this.program);

        // Connect `positionArray` to the `vertexPos` shader parameter.
        this.#configureShaderParam("vertexPos", gl.FLOAT, 2, this.positionBuffer, this.positionArray);
        this.#configureShaderParam("vertexTtl", gl.FLOAT, 1, this.ttlBuffer, this.ttlArray);

        gl.drawArrays(gl.LINES,
                      /*first=*/0,
                      /*count=*/numLines * 2);
    }

    #configureShaderParam(name, type, numPerVertex, glBuffer, array) {
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

    static #vertexShaderSource = `#version 300 es
 
in vec2 vertexPos;
in float vertexTtl;

out vec4 aColor;
 
void main() {
  gl_Position = vec4(vertexPos[0], vertexPos[1], 0, 1);

  // Over a line's lifetime, the ttlScaled decreases from a maximum of 1 to a
  // minimum of 0.
  float ttlScaled = (vertexTtl * vertexTtl) / (512.0 * 512.0);

  float r = 0.3 * (1.0 - ttlScaled);
  float g = ttlScaled;
  float b = 0.0;

  aColor = vec4(r, g, b, 1);
}
`;

    static #fragmentShaderSource = `#version 300 es
 
precision highp float;
 
in vec4 aColor;
out vec4 outColor;
 
void main() {
  outColor = aColor;
}
`;

}

class CoralPolyp {
    constructor(ctx2d, glState, adjustableVariables, angle, r, g, b, x1, y1, x2, y2) {
        this.ttl = kMaxTtl;

        this.ctx2d = ctx2d;
        this.glState = glState;
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
        const ctx = this.ctx2d;
        const r = Math.floor(this.r * brightness);
        const g = Math.floor(this.g * brightness);
        const b = Math.floor(this.b * brightness);
        const strokeStyle = `rgb(${r} ${g} ${b})`;
        ctx.strokeStyle = strokeStyle;

        ctx.lineWidth = 2;

        ctx.beginPath();
        ctx.moveTo(this.x1, this.y1);
        ctx.lineTo(this.x2, this.y2);
        ctx.closePath();
        ctx.stroke();
    }

    // Makes this individual polyp either grow or split. The `polyps` parameter
    // is a reference to an array of other polyps. Splitting will insert two new
    // instances into the `polyps` array.
    act(polyps, signalMatrix) {
        if (Math.random() < this.adjustableVariables.growChance) {
            // Speculatively grow in the current direction, but undo it if we
            // grow out of bounds.
            let oldX2 = this.x2;
            let oldY2 = this.y2;
            this.x2 += kGrowthMagnitude * Math.cos(this.angle);
            this.y2 += kGrowthMagnitude * Math.sin(this.angle);

            const signalMatrixIndex = CoralPolyp.getIsOccupiedIndex(this.x2, this.y2);
            if (this.isInBounds() && signalMatrix[signalMatrixIndex] <= kGrowSignalThresh) {
                // Increase the signal at the new endpoint.
                console.assert(signalMatrix[signalMatrixIndex] <= kMaxU16);
                if (signalMatrix[signalMatrixIndex] <= kMaxU16 - kSignalAttack) {
                    signalMatrix[signalMatrixIndex] += kSignalAttack;
                }
            } else {
                this.x2 = oldX2;
                this.y2 = oldY2;
            }
        }

        if (Math.random() < this.adjustableVariables.splitChance) {
            // Choose the child's angle based on our angle.
            const maxTurn = this.adjustableVariables.maxTurn / 360 * 2 * Math.PI;
            const newAngle = this.angle + (2 * Math.random() - 1) * maxTurn;

            const child = new CoralPolyp(
                this.ctx2d,
                this.glState,
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

            const signalMatrixIndex = CoralPolyp.getIsOccupiedIndex(child.x2, child.y2);
            if (child.isInBounds() && signalMatrix[signalMatrixIndex] <= kSplitSignalThresh) {
                polyps.push(child);

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

        // Convert coordinates to a `signalSquareSideLen**2` grid.
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

class AdjustableVariables {
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
    }
}

function buildThunks() {
    const paramsString = window.location.search;
    const searchParams = new URLSearchParams(paramsString);
    const contextParam = searchParams.get("context");

    const adjustableVariables = new AdjustableVariables();
    const signalMatrix = new Uint16Array(kSignalSquareSideLen ** 2);

    let animationPaused = false;

    let polyps = [];

    const glState = new GlState();
    console.assert(glState.isEnabled());

    const ctx = contextParam === "no-2d" ? null : document.getElementById("game").getContext("2d");

    function animateFunc() {
        if (animationPaused) {
            return;
        }

        // Decay the signal in `signalMatrix`.
        for (let i = 0; i < signalMatrix.length; ++i) {
            const decayRate = Math.floor(adjustableVariables.signalDecayRate);
            if (signalMatrix[i] > decayRate) {
                signalMatrix[i] -= decayRate;
            } else {
                signalMatrix[i] = 0;
            }
        }

        if (ctx) {
            ctx.clearRect(0, 0, 1000, 1000);
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
                const scaledR = Math.floor(255 * signalLogScaled);
                const scaledB = Math.floor(255 * signalLogScaled);
                const color = `rgb(${scaledR} 0 ${scaledB} / 90%)`;

                ctx.fillStyle = color;
                ctx.fillRect(x * 10, y * 10, 10, 10);
            }
        }

        // Maybe initialize.
        if (polyps.length === 0) {
            const first = new CoralPolyp(
                ctx,
                glState,
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

            signalMatrix[CoralPolyp.getIsOccupiedIndex(first.x2, first.y2)] = 1;

            polyps = [first];
        }

        // The youngest polyps are always at the end of the array.
        for (
            let i = Math.floor((1 - adjustableVariables.dormancyAgePercentile) * polyps.length);
            i < polyps.length;
            ++i
        ) {
            polyps[i].act(polyps, signalMatrix);
        }

        polyps = polyps.filter((b) => b.ttl > 0 && b.isInBounds());

        // Cull the oldest polyps when we've reached capacity.
        if (polyps.length > kMaxNumObjects) {
            polyps = polyps.slice(polyps.length - kMaxNumObjects);
        }

        polyps.forEach((b, i) => {
            b.ttl--;

            if (b.ttl % 16 === 0) {
                b.r = Math.min(64, (b.r + 2));
                b.g = Math.max(0, (b.g - 12));
                b.b = Math.min(64, (b.b + 1));
            }

            if (glState.isEnabled()) {
                const positionArray = glState.positionArray;
                const ttlArray = glState.ttlArray;

                // Scale coordinates to [-1, 1].
                // FIXME Do this with a projection matrix?
                positionArray[kNumPositionFields * i + 0] = b.x1 / 500 - 1;
                positionArray[kNumPositionFields * i + 1] = (2 - b.y1 / 500) - 1;
                positionArray[kNumPositionFields * i + 2] = b.x2 / 500 - 1;
                positionArray[kNumPositionFields * i + 3] = (2 - b.y2 / 500) - 1;

                ttlArray[2 * i + 0] = b.ttl;
                ttlArray[2 * i + 1] = b.ttl;
            }
        });

        if (glState.isEnabled()) {
            glState.render(/*numLines=*/polyps.length);
        }

        if (ctx) {
            polyps.forEach((p) => { p.draw2d(); });
        }

        window.requestAnimationFrame(animateFunc);
    }

    function onload() {
        window.requestAnimationFrame(animateFunc);
    }

    function onkeydown(event) {
        console.log(event);

        if (event.code === "Space") {
            event.preventDefault();

            animationPaused = !animationPaused;
            if (!animationPaused) {
                window.requestAnimationFrame(animateFunc);
            }
        }
    }

    return {
        onload: onload,
        onkeydown: onkeydown,
    };
}

let thunks = buildThunks();
window.onload = thunks.onload;
window.onkeydown = thunks.onkeydown;
window.onmousemove = thunks.onmousemove;
window.ontouchmove = thunks.ontouchmove;
