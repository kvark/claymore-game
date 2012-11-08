game-code: engine-code
	rustc game/claymore.rc -L lib --out-dir build

engine-code:
	rustc engine/engine.rc -L lib --out-dir lib

clean-engine:
	rm -R lib/libengine*

DIR=claymore-game

extern: lmath glfw3 glcore

clean-extern:
	(cd lib && rm -Rf lmath* glfw3* glcore*)

lmath:
	(cd ../lmath-rs && make && cp -R lib/* ../${DIR}/lib/)
	
glfw3:
	(cd ../glfw3-rs && make && cp -R lib/* ../${DIR}/lib/)

glcore:
	(cd ../glcore-rs && make osx-lion && cp -R lib/* ../${DIR}/lib/)
