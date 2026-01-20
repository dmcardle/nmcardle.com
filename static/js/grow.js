const kMaxNumObjects = 1 << 14
const kMaxTtl = 1 << 9;
const kSvgNs = "http://www.w3.org/2000/svg";

class CoralPolyp {
    constructor(svg, adjustableVariables, angle, r, g, b, x1, y1, x2, y2) {
        const line = document.createElementNS(kSvgNs, "line");

        // Permanent attributes that won't be updated by `flushLine()`.
        line.setAttribute("stroke-width", ".2px");

        svg.appendChild(line);

        this.svg = svg;
        this.adjustableVariables = adjustableVariables;
        this.elem = line;
        this.ttl = kMaxTtl;
        this.angle = angle;

        // Used to reduce the number of expensive `setAttribute()` calls for
        // color attributes.
        this.needFlush = true;

        // Register getters and setters for fields that defer setting the
        // `needsFlush` bit as an optimization. These fields don't need to be
        // updated as frequently because they are less visually important.
        for (const fieldName of ["r", "g", "b"]) {
            const backingFieldName = "_" + fieldName;
            Object.defineProperty(this, fieldName, {
                get() {
                    return this[backingFieldName];
                },
                set(x) {
                    this[backingFieldName] = x;
                    if (this.ttl % 32 === 0) {
                        this.needsFlush = true;
                    }
                },
            });
        }

        // Register getters and setters for fields that immediately set the
        // needsFlush bit.
        for (const fieldName of ["x1", "y1", "x2", "y2"]) {
            const backingFieldName = "_" + fieldName;
            Object.defineProperty(this, fieldName, {
                get() {
                    return this[backingFieldName];
                },
                set(x) {
                    this[backingFieldName] = x;
                    this.elem.setAttribute(fieldName, x);
                },
            });
        }

        (this.x1 = x1), (this.y1 = y1), (this.x2 = x2), (this.y2 = y2);

        this.r = r;
        this.g = g;
        this.b = b;

        this.flush();
    }

    // Copies information from `line`'s top-level convenience attributes to its
    // SVG element. For instance, `line.x1` will be copied to the `x1` attribute
    // of `line.elem`.
    flush() {
        // The needsFlush bit helps us avoid calling the expensive `setAttribute()`
        // function when it's unnecessary. It's slow, but still faster than
        // `setAttributeNS(null, ...)`.
        if (!this.needsFlush) {
            return;
        }

        const brightness = this.ttl * this.ttl / (kMaxTtl * kMaxTtl);
        this.elem.setAttribute(
            "stroke",
            `rgb(${this.r * brightness}, ${this.g * brightness}, ${this.b * brightness})`,
        );

        this.needsFlush = false;
    }

    // Makes this individual polyp either grow or split. The `polyps` parameter
    // is a reference to an array of other polyps. Splitting will insert two new
    // instances into the `polyps` array.
    act(polyps, isOccupiedMatrix) {
        const kChanceGrow = this.adjustableVariables.growChance;
        const kChanceSplit = this.adjustableVariables.splitChance;
        const kMaxTurn = this.adjustableVariables.maxTurn / 360 * 2 * Math.PI;

        if (Math.random() < kChanceGrow) {
            let oldX2 = this.x2;
            let oldY2 = this.y2;

            this.x2 += 0.1 * Math.cos(this.angle);
            this.y2 += 0.1 * Math.sin(this.angle);

            if (!this.isInBounds()) {
                this.x2 = oldX2;
                this.y2 = oldY2;
            }

            const isOccupiedIndex = CoralPolyp.getIsOccupiedIndex(this.x2, this.y2);
            console.assert(isOccupiedMatrix[isOccupiedIndex] < 1 << 16);
            if (isOccupiedMatrix[isOccupiedIndex] < 1 << 16) {
                isOccupiedMatrix[isOccupiedIndex] += Math.floor(kMaxTtl / 16);
            }
        }

        if (Math.random() < kChanceSplit) {
            const kNumChildren = 1;
            for (let i = 0; i < kNumChildren; ++i) {
                const angle = this.angle + (2 * Math.random() - 1) * kMaxTurn;
                const newX2 = this.x2 + 2 * Math.cos(angle);
                const newY2 = this.y2 + 2 * Math.sin(angle);

                const isOccupiedIndex = CoralPolyp.getIsOccupiedIndex(newX2, newY2);
                if (isOccupiedMatrix[isOccupiedIndex] > kMaxTtl / 32) {
                    continue;
                }
                console.assert(isOccupiedMatrix[isOccupiedIndex] < 1 << 16);
                isOccupiedMatrix[isOccupiedIndex] += Math.floor(kMaxTtl / 16);

                let newPolyp = new CoralPolyp(
                    this.svg,
                    this.adjustableVariables,
                    angle,
                    0,
                    255,
                    0,
                    this.x2,
                    this.y2,
                    this.x2 + 0.4 * Math.cos(angle),
                    this.y2 + 0.4 * Math.sin(angle),
                );

                polyps.push(newPolyp);

                if (!newPolyp.isInBounds()) {
                    newPolyp.ttl = 0;
                }
            }
        }
    }

    static getIsOccupiedIndex(x, y) {
        const clampedX2 = Math.max(0, Math.min(Math.round(x), 99));
        const clampedY2 = Math.max(0, Math.min(Math.round(y), 99));
        const isOccupiedIndex = clampedX2 * 100 + clampedY2;
        return Math.max(0, Math.min(isOccupiedIndex, 100 * 100 - 1));
    }

    isInBounds() {
        for (const fieldName of ["_x1", "_y1", "_x2", "_y2"]) {
            const value = this[fieldName];
            if (value < 0 || value > 100) {
                return false;
            }
        }
        return true;
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
    const svg = document.getElementById("gameSvg");

    // const paramsString = window.location.search;
    // const searchParams = new URLSearchParams(paramsString);
    // const debugMode = searchParams.get("debug");
    // if (debugMode) {
    //     const text = document.createElementNS(kSvgNs, "text");
    //     text.setAttribute("x", 0);
    //     text.setAttribute("y", 0);
    //     text.setAttribute("stroke", "red");
    //     text.innerText = "DEBUG";
    //     svg.appendChild(text);
    // }

    let animationPaused = false;

    let polyps = [];

    const adjustableVariables = new AdjustableVariables();

    const isOccupiedMatrix = new Uint16Array(100 * 100);

    const isOccupiedRects = new Array();

    for (let x = 0; x < 100; ++x) {
        for (let y = 0; y < 100; ++y) {
            const rect = document.createElementNS(kSvgNs, "rect");
            rect.setAttribute("x", x);
            rect.setAttribute("y", y);
            rect.setAttribute("width", 1);
            rect.setAttribute("height", 1);

            isOccupiedRects.push(rect);
            svg.appendChild(rect);
        }
    }

    let frameCount = 0;
    let didHideSignals = false;

    function animate() {
        if (animationPaused) {
            return;
        }

        frameCount = (frameCount + 1) & (1 << 12);

        if (frameCount === 0) {
            let maxValue;
            if (adjustableVariables.showSignals) {
                maxValue = Math.max(...isOccupiedMatrix);
            }

            const decayRate = Math.floor(adjustableVariables.signalDecayRate);
            for (let i = 0; i < isOccupiedMatrix.length; ++i) {
                if (isOccupiedMatrix[i] > decayRate) {
                    isOccupiedMatrix[i] -= decayRate;
                } else {
                    isOccupiedMatrix[i] = 0;
                }

                if (adjustableVariables.showSignals) {
                    const value = isOccupiedMatrix[i];
                    const scaledR = 255 * (value / maxValue);
                    const scaledB = 128 * (value / maxValue);

                    isOccupiedRects[i].setAttribute(
                        "stroke",
                        `rgb(${scaledR}, 0, ${scaledB})`,
                    );
                } else if (!didHideSignals) {
                    isOccupiedRects[i].setAttribute("stroke", "rgb(0, 0, 0)");
                }
            }

            didHideSignals = !adjustableVariables.showSignals;
        }

        // Maybe initialize.
        if (polyps.length === 0) {
            const first = new CoralPolyp(
                svg,
                adjustableVariables,
                /*angle=*/(3 * Math.PI) / 2,
                /*r=*/ 0,
                /*g=*/ 255,
                /*b=*/ 0,
                /*x1=*/ 50,
                /*y1=*/ 100,
                /*x2=*/ 50,
                /*y2=*/ 98,
            );

            for (let i = 0; i < isOccupiedMatrix.length; ++i) {
                isOccupiedMatrix[i] = 0;
            }

            isOccupiedMatrix[CoralPolyp.getIsOccupiedIndex(first.x2, first.y2)] = 1;

            polyps = [first];
        }

        // The youngest polyps are always at the end of the array.
        for (
            let i = Math.floor((1 - adjustableVariables.dormancyAgePercentile) * polyps.length);
            i < polyps.length;
            ++i
        ) {
            polyps[i].act(polyps, isOccupiedMatrix);
        }

        polyps = polyps.filter((b) => {
            if (b.ttl <= 0 || !b.isInBounds()) {
                svg.removeChild(b.elem);
                return false;
            }
            return true;
        });

        // Cull the oldest polyps when we've reached capacity.
        if (polyps.length > kMaxNumObjects) {
            const kNumToKill = polyps.length - kMaxNumObjects;
            for (let i = 0; i < kNumToKill; ++i) {
                svg.removeChild(polyps[i].elem);
            }
            polyps = polyps.slice(polyps.length - kMaxNumObjects);
        }

        // Update each polyp's color and flush its attributes to the
        // corresponding SVG element.
        polyps.forEach((b) => {
            b.ttl -= 1;

            if (b.ttl % 32 == 0) {
                b.r = Math.min(b.r + 32, 255);
                b.g = Math.max(b.g - 32, 0);
                b.b = Math.max(50, 255 - Math.floor((255 * b.ttl) / kMaxTtl));
            }

            b.flush();
        });

        window.requestAnimationFrame(animate);
    }

    function onload() {
        window.requestAnimationFrame(animate);
    }

    function onkeydown(event) {
        console.log(event);

        if (event.code === "Space") {
            event.preventDefault();

            animationPaused = !animationPaused;
            if (!animationPaused) {
                window.requestAnimationFrame(animate);
            }
        }
    }

    // Transform mouse coordinates into SVG coordinates.
    function transformMouseToSvgCoordinates(x, y, svg) {
        let point = svg.createSVGPoint();
        point.x = event.clientX;
        point.y = event.clientY;
        return point.matrixTransform(svg.getScreenCTM().inverse());
    }

    function handleMouseOrTouchMove(event) {
        let mousePoint = transformMouseToSvgCoordinates(
            event.clientX,
            event.clientY,
            svg,
        );
        if (!isFinite(mousePoint.x) || !isFinite(mousePoint.y)) {
            return;
        }
    }

    return {
        onload: onload,
        onkeydown: onkeydown,
        onmousemove: handleMouseOrTouchMove,
        ontouchmove: handleMouseOrTouchMove,
    };
}

let thunks = buildThunks();
window.onload = thunks.onload;
window.onkeydown = thunks.onkeydown;
window.onmousemove = thunks.onmousemove;
window.ontouchmove = thunks.ontouchmove;
