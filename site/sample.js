
Window.wasm = {};
fetch('app.wasm')
	.then(response => response.arrayBuffer())
	.then(bytes => WebAssembly.instantiate(bytes, { env: {} }))
	.then(result => {
		Window.wasm = result;
		Window.app = Window.wasm.instance.exports.start();
	});


