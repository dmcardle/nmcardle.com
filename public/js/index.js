function buildThunks() {
    let animationPaused = false;

    let thunks = {};
    thunks.onload = function() {
        const kSvgNs = "http://www.w3.org/2000/svg";
        let svg = document.getElementById("gameSvg");
        let blobs = [];

        function newStar(elem) {
            return {
                x: 50,
                y: 50,
                r: Math.random()/2 + 0.1,
                vx: Math.random() - 0.5,
                vy: Math.random() - 0.5,
                elem: elem,
            };
        }

        function randColor() {
            let s = "#";
            for (let i=0; i<3; i++) {
                s += (Math.floor(Math.random()*256)).toString(16);
            }
            return s;
        }

        function animate() {
            const kMaxNumStars = 128;
            if (blobs.length < kMaxNumStars && Math.random() < 0.25) {
                let circle = document.createElementNS(kSvgNs, 'circle');
                let star = newStar(circle);
                circle.setAttributeNS(null, 'stroke-width', '0.1%');
                circle.setAttributeNS(null, 'stroke', '#fff');
                circle.setAttributeNS(null, 'fill', randColor());
                svg.appendChild(circle);
                blobs.unshift(newStar(circle));
            }

            blobs.forEach((blob, i, arr) => {
                blob.x += blob.vx;
                blob.y += blob.vy;

                blob.vx *= 1.05;
                blob.vy *= 1.02;

                blob.r *= 1.005;

                const x = blob.x;
                const y = blob.y;
                const r = blob.r;
                if (x + r < 0 || x - r > 100 || y - r < 0 || y + r > 100) {
                    arr[i] = newStar(blob.elem);
                }

                blob.elem.setAttributeNS(null, 'cx', x);
                blob.elem.setAttributeNS(null, 'cy', y);
                blob.elem.setAttributeNS(null, 'r', r);
            });
            window.requestAnimationFrame(animate);
        }
        window.requestAnimationFrame(animate);
    };
    return thunks;
}

window.onload = function() {
    let thunks = buildThunks();
    window.onload = thunks.onload;
}
