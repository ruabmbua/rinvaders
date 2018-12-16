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
        gamepads.push(gamepad);
    }, false);

    // Request the first game loop (animation frame).
    // ----------------------------------------------
    window.requestAnimationFrame(update);
}

// Regular update function. Will be called by requestAnimationFrame.
// -----------------------------------------------------------------
function update(ts) {
    // Gamepad input processing (was to lazy to do this in the rust code).
    // -------------------------------------------------------------------
    var left = false;
    var right = false;
    var shoot = false;

    for(var i = 0; i < gamepads.length; i++) {
        left |= gamepads[i].axes[6] < -0.5;
        right |= gamepads[i].axes[6] > 0.5;
        shoot |= gamepads[i].buttons[0].pressed;    
    }

    game.set_gamepad_state(left, right, shoot);

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