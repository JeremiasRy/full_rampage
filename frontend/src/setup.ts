import { app } from ".";

export async function setup() {
    const gameWindow = document.getElementById("game-window");
    if (!gameWindow) {
        throw Error("Game window not found :(")
    }
    await app.init({background: '#beef', resizeTo:  gameWindow});
    gameWindow.appendChild(app.canvas);
}