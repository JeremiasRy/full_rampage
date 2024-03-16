import { Application } from 'pixi.js'
const app = new Application();
setup();

async function setup() {
    await app.init({background: '#beef', resizeTo: window});
    document.body.appendChild(app.canvas);
}