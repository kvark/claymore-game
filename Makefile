NAME=claymore
DIR=${NAME}-game
RUST=/usr/local/bin/rustc

# game/engine/demos code
all: engine-code game-code

game-code:
	${RUST} game/claymore.rc -L lib --out-dir build

engine-code:
	${RUST} engine/engine.rc -L lib --out-dir lib

demo-03:
	${RUST} sample/demo03-materials.rs -L lib

clean:
	rm -Rf lib/* build/*

clean-engine:
	rm -R lib/libengine*


# external libraries section
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

# demo packing
demo-pack:
	cp build/${NAME} .
	tar -czf demo.tar.gz engine game data ${NAME}
	rm ${NAME}
