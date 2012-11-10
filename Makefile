game-code: engine-code
	rustc game/claymore.rc -L lib --out-dir build

engine-code:
	rustc engine/engine.rc -L lib --out-dir lib

clean:
	rm -Rf lib/* build/*

clean-engine:
	rm -R lib/libengine*

NAME=claymore
DIR=${claymore}-game

extern: clean-lib lmath glfw3 glcore stb-image

clean-lib:
	(cd lib && rm -Rf liblmath* libglfw3* libglcore* libstb*)

lmath:
	(cd ../lmath-rs && make clean && make && cp -R lib/* ../${DIR}/lib/)
	
glfw3:
	(cd ../glfw3-rs && make clean && make && cp -R lib/* ../${DIR}/lib/)

glcore:
	(cd ../glcore-rs && make clean && make osx-lion && cp -R lib/* ../${DIR}/lib/)

stb-image:
	(cd ../rust-stb-image && make clean && make && cp -R *.dylib* ../${DIR}/lib/)

demo:
	cp build/${NAME} .
	tar -czf demo.tar.gz engine game data ${NAME}
	rm ${NAME}
