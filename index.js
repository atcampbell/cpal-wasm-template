import { createScope } from './js/scope.js';

import('./pkg').catch(console.error).then((rust_module) => {
    // Rough [min, max] of each Lorenz axis, used to fit it on the scope.
    const AXIS_RANGE = {
        x: [-25, 25],
        y: [-30, 30],
        z: [0, 55],
    };
    // Which axes to plot horizontally and vertically: any of 'x', 'y', 'z'.
    const SCOPE_H = 'x';
    const SCOPE_V = 'y';

    const scope = createScope(document.getElementById('scope'), AXIS_RANGE[SCOPE_H], AXIS_RANGE[SCOPE_V]);

    // The synth is created on the first start and then kept: stop pauses the
    // audio rather than destroying it, so the synth's state survives.
    let handle = null;
    let playing = false;

    const play_button = document.getElementById('play');
    play_button.addEventListener('click', () => {
        if (handle == null) {
            // Created inside a click because browsers only allow audio to
            // start from a user gesture.
            handle = rust_module.start(
                param_inputs.map((input) => input.dataset.param),
                Float64Array.from(param_inputs, param_value),
            );
            playing = true;
        } else if (playing) {
            handle.pause();
            playing = false;
        } else {
            handle.play();
            playing = true;
        }
        play_button.value = playing ? 'stop' : 'start';
    });

    // Every element with a `data-param="name"` attribute in index.html is sent
    // to Rust under that name. An element with the id `<name>-value`, if there
    // is one, shows the current value.
    const param_inputs = [...document.querySelectorAll('[data-param]')];
    const param_value = (input) => parseFloat(input.value);

    for (const input of param_inputs) {
        const name = input.dataset.param;
        const display = document.getElementById(`${name}-value`);
        input.addEventListener('input', () => {
            if (display != null) {
                display.textContent = input.value;
            }
            if (handle != null) {
                handle.set_param(name, param_value(input));
            }
        });
    }

    const animate = () => {
        scope.fade();
        if (handle != null && playing) {
            const points = handle.take_points(); // has .x, .y, .z Float32Arrays
            scope.draw(points[SCOPE_H], points[SCOPE_V]);
            points.free(); // release the Rust-side memory
        }
        requestAnimationFrame(animate);
    };
    requestAnimationFrame(animate);
});
