const { hello, Game } = wasm_bindgen;

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

    window.requestAnimationFrame(update);
}

function update(ts) {
    game.render();
    game.update(ts);
    window.requestAnimationFrame(update);
}

wasm_bindgen("./rinvaders_bg.wasm")
    .then(run)
    .catch(console.error);