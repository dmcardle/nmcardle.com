function buildThunks() {
    function newStar(elem) {
        let star = {
            x: Math.random() * 100,
            y: Math.random() * 100,
            r: Math.random()/2 + 0.1,
            vx: Math.random() - 0.5,
            vy: Math.random() - 0.5,
            elem: elem,
        };
        return star;
    }

    let thunks = {};

    let animationPaused = false;
    const kSvgNs = "http://www.w3.org/2000/svg";
    const svg = document.getElementById("gameSvg");


    let mouseCircle = document.createElementNS(kSvgNs, 'circle');
    mouseCircle.setAttributeNS(null, 'stroke-width', '0.1%');
    mouseCircle.setAttributeNS(null, 'stroke', '#fff');
    mouseCircle.setAttributeNS(null, 'fill', '#f00');
    svg.appendChild(mouseCircle);
    const mouseBlob = newStar(mouseCircle);
    mouseBlob.r = 0;
    let mouseEnabled = false;

    let blobs = [ mouseBlob ];

    function randColor() {
        let s = "#";
        for (let i=0; i<3; i++) {
            s += (Math.floor(Math.random()*256)).toString(16);
        }
        return s;
    }

    function animate() {
        if (animationPaused) {
            return;
        }

        const kMaxNumStars = 50;
        if (blobs.length < kMaxNumStars && Math.random() < 0.05) {
            let circle = document.createElementNS(kSvgNs, 'circle');
            circle.setAttributeNS(null, 'stroke-width', '0.1%');
            circle.setAttributeNS(null, 'stroke', '#fff');
            circle.setAttributeNS(null, 'fill', randColor());
            svg.appendChild(circle);
            blobs.unshift(newStar(circle));
        }

        blobs.forEach((blob, i, arr) => {
            // Never apply forces to the mouse.
            if (blob === mouseBlob) {
                blob.elem.setAttributeNS(null, 'cx', blob.x);
                blob.elem.setAttributeNS(null, 'cy', blob.y);
                // The mouse blob is 10 times denser than other blobs.
                blob.elem.setAttributeNS(null, 'r', blob.r / 10);
                return;
            }

            blob.x += blob.vx;
            blob.y += blob.vy;

            blobs.forEach((otherBlob, j, arr) => {
                // Assume objects do not interact with themselves.
                if (i === j) {
                    return;
                }

                // Assume mass is proportional to radius.
                const kGravConstant = 0.7;
                const distSquared = Math.pow(blob.x - otherBlob.x, 2) +
                      Math.pow(blob.y - otherBlob.y, 2);

                if (distSquared < Math.pow(blob.r + otherBlob.r, 2) && blob.r > otherBlob.r) {
                    let p1x = blob.r * blob.vx;
                    let p1y = blob.r * blob.vy;

                    let p2x = otherBlob.r * otherBlob.vx;
                    let p2y = otherBlob.r * otherBlob.vy;

                    let px = p1x + p2x;
                    let py = p1y + p2y;

                    let vx = px / (blob.r + otherBlob.r);
                    let vy = py / (blob.r + otherBlob.r);

                    blob.r = Math.sqrt(blob.r * blob.r + otherBlob.r * otherBlob.r);

                    const kFudge = 0.8;
                    blob.vx = kFudge * vx;
                    blob.vy = kFudge * vy;

                    otherBlob = newStar(otherBlob.elem);
                    arr[j] = otherBlob;
                }

                const force = kGravConstant * blob.r * otherBlob.r / distSquared;

                // Force is symmetric, so apply to both blobs.
                const angle = Math.atan2(otherBlob.y - blob.y, otherBlob.x - blob.x);
                blob.vx += Math.cos(angle) * force;
                blob.vy += Math.sin(angle) * force;
            });

            const x = blob.x;
            const y = blob.y;
            const r = blob.r;
            // if (x + r < 0 || x - r > 100) {
            //     console.assert(blob != mouseBlob);
            //     arr[i] = newStar(blob.elem);
            // }
            if (x + r < 0) {
                blob.x = r;
                blob.vx *= -1;
            }
            if (x - r > 100) {
                if (Math.random() < 0.5) {
                    blob.x = 100 - blob.r;
                    blob.vx *= -1;
                } else {
                    arr[i] = newStar(blob.elem);
                }
            }
            if (y - r < 0) {
                blob.y = blob.r;
                blob.vy *= -1;
            }
            if (y + r > 100) {
                blob.y = 100 - blob.r;
                blob.vy *= -1;
            }

            blob.elem.setAttributeNS(null, 'cx', x);
            blob.elem.setAttributeNS(null, 'cy', y);
            blob.elem.setAttributeNS(null, 'r', r);
        });
        window.requestAnimationFrame(animate);
    }

    thunks.onload = function() {
        window.requestAnimationFrame(animate);
    };

    thunks.onkeydown = (event) => {
        console.log(event);
        if (event.code === "Space") {
            animationPaused = !animationPaused;

            if (!animationPaused) {
                window.requestAnimationFrame(animate);
            }
        } else if (event.code === "KeyM") {
            // TODO: Apply mass to the cursor.
            mouseEnabled = !mouseEnabled;
            console.log("mouse enabled? " + mouseEnabled);

            if (mouseEnabled) {
                mouseBlob.r = 10;
            } else {
                mouseBlob.r = 0;
            }
        }
    };

    // Transform mouse coordinates into SVG coordinates.
    function transformMouseToSvgCoordinates(x, y, svg) {
        let point = svg.createSVGPoint();
        point.x = event.clientX;
        point.y = event.clientY;
        return point.matrixTransform(svg.getScreenCTM().inverse());
    }

    function handleMouseOrTouchMove(event) {
        let mousePoint = transformMouseToSvgCoordinates(event.clientX,
                                                        event.clientY,
                                                        svg);
        mouseBlob.x = mousePoint.x;
        mouseBlob.y = mousePoint.y;
    }

    thunks.onmousemove = handleMouseOrTouchMove;
    thunks.ontouchmove = handleMouseOrTouchMove;

    return thunks;
}

let thunks = buildThunks();
window.onload = thunks.onload;
window.onkeydown = thunks.onkeydown;
window.onmousemove = thunks.onmousemove;
window.ontouchmove = thunks.ontouchmove;
