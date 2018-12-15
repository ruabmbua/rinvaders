const { hello, Game } = wasm_bindgen;

gamepads = [];

function run() {
    hello("world");

    let canvas = document.getElementById("main_canvas");

    game = new Game(canvas);

    document.onkeydown = (key) => {
        game.keyboard_event(true, key);
    };
    document.onkeyup = (key) => {
        game.keyboard_event(false, key);
    }

    window.addEventListener("gamepadconnected", (e) => {
        var gamepad = e.gamepad;
        console.log("Gamepad " + gamepad.id + " connected!");
        console.log(gamepad);
        gamepads.push(gamepad);
    }, false);

    window.requestAnimationFrame(update);
}



function update(ts) {
    var left = false;
    var right = false;
    var shoot = false;

    for(var i = 0; i < gamepads.length; i++) {
        left |= gamepads[i].axes[6] < -0.5;
        right |= gamepads[i].axes[6] > 0.5;
        shoot |= gamepads[i].buttons[0].pressed;    
    }

    game.set_gamepad_state(left, right, shoot);
    game.update(ts);
    game.render();
    window.requestAnimationFrame(update);
}

wasm_bindgen("./rinvaders_bg.wasm")
    .then(run)
    .catch(console.error);