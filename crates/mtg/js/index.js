import init, { Game } from "mtg";

async function main() {
	console.log("Loading WebAssembly...");
	await init();
	console.log("WebAssembly loaded!");

	let game = new Game();
	console.log(game.players());
}

main();