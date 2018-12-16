/**
 * This file is the main entry point to rust invaders. It grabs some elements (the canvas) from
 * the index.html, sets up the games rust wasm module, and instantiates an animation loop in
 * the browser, in which the rust type module will be updated.
 */

// Get access to functions / datatypes of wasm module.
// ---------------------------------------------------
const { hello, Game } = wasm_bindgen;

// Gamepad array (they are appended on plug and play).
// ---------------------------------------------------
gamepads = [];
gamepad_mappings = [];

// Main function where game is initialized.
// ----------------------------------------
function run() {
    // Call a rust function from the wasm module (just to test it).
    // ------------------------------------------------------------
    hello("world");

    // Get the canvas, into which the game will be drawn.
    // --------------------------------------------------
    let canvas = document.getElementById("main_canvas");

    // Instantiate a game object from the rust module Game type.
    // ---------------------------------------------------------
    game = new Game(canvas);

    // Register onkeydown & onkeyup with callback that formwards the key to
    // the rust module keyboard_event() function.
    // --------------------------------------------------------------------
    document.onkeydown = (key) => {
        game.keyboard_event(true, key);
    };
    document.onkeyup = (key) => {
        game.keyboard_event(false, key);
    }

    // Register handler for gamepadconnected event. The handler will add the
    // the new gamepad to the global gamepad list, which is polled by the game
    // for input.
    // -----------------------------------------------------------------------
    window.addEventListener("gamepadconnected", (e) => {
        var gamepad = e.gamepad;
        console.log("Gamepad " + gamepad.id + " connected!");
        console.log(gamepad);

        if (gamepad.mapping == "default") {
            console.log("Has default mapping.");
            gamepad_mappings.push("default");
            gamepads.push(gamepad);
        } else if (gamepad.id.indexOf("Logitech Gamepad F710") !== -1) {
            console.log("Has Logitech F710 mapping")
            gamepad_mappings.push("logitech_f710");
            gamepads.push(gamepad);
        } else {
            console.log("Unsupported mapping");
        }
    }, false);

    // Request the first game loop (animation frame).
    // ----------------------------------------------
    window.requestAnimationFrame(update);
}

function default_gamepad(state, gamepad) {
    state.left |= gamepad.buttons[14].pressed;
    state.right |= gamepad.buttons[15].pressed;
    state.shoot |= gamepad.buttons[0].pressed;
}

function logitech_f710_gamepad(state, gamepad) {
    state.left |= gamepad.axes[6] < -0.5;
    state.right |= gamepad.axes[6] > 0.5;
    state.shoot |= gamepad.buttons[0].pressed;
}

// Regular update function. Will be called by requestAnimationFrame.
// -----------------------------------------------------------------
function update(ts) {
    // Gamepad input processing (was to lazy to do this in the rust code).
    // -------------------------------------------------------------------
    var gamepad_state = {
        left: false,
        right:  false,
        shoot: false
    }

    for (var i = 0; i < gamepads.length; i++) {
        switch (gamepad_mappings[i]) {
            case "default":
                default_gamepad(gamepad_state, gamepads[i])
                break;
            case "logitech_f710":
                logitech_f710_gamepad(gamepad_state, gamepads[i]);
                break;
            default:
                console.error("Not supported gamepad mapping got added.");
                break;
        }
    }

    game.set_gamepad_state(gamepad_state.left, gamepad_state.right, gamepad_state.shoot);

    // Update, and then render the game. The update function gets to know the
    // current high res timestamp of the running animation.
    // ----------------------------------------------------------------------
    game.update(ts);
    game.render();

    // Request the next animation frame from the browser.
    // --------------------------------------------------
    window.requestAnimationFrame(update);
}

// Load the wasm-bindgen generated module, wait untils it is compiled and
// instantiated, then execute the run() function.
// ---------------------------------------------------------------------- 
wasm_bindgen("./rinvaders_bg.wasm")
    .then(run)
    .catch(console.error);