game-code: engine-code
	rustc game/claymore.rc -L lib --out-dir build

engine-code:
	rustc engine/engine.rc -L lib --out-dir lib
