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
	${RUST} sample/demo03-materials.rs	-L lib --out-dir build
demo-04:
	${RUST} sample/demo04-skeleton.rs	-L lib --out-dir build
demo-05:
	${RUST} sample/demo05-text.rs		-L lib --out-dir build

clean:
	rm -Rf lib/* build/*

clean-engine:
	rm -R lib/libengine*


# external libraries section
libs: clean-libs numeric lmath glfw3 glcore openal stb-image freetype

clean-libs:
	(cd lib && rm -Rf liblmath* libglfw3* libglcore* openal* libstb*)

numeric:
	(cd ../numeric-rs && rustc src/numeric.rc --out-dir ../${DIR}/lib/)

lmath: numeric
	(cd ../lmath-rs && make clean && make && cp -R lib/* ../${DIR}/lib/)
	
glfw3:
	(cd ../glfw3-rs && make clean && make && cp -R lib/* ../${DIR}/lib/)

glcore:
	(cd ../glcore-rs && make clean && make osx-lion && cp -R lib/* ../${DIR}/lib/)

openal:
	(cd ../openal-rs && rustc src/openal.rc --out-dir ../${DIR}/lib/)

stb-image:
	(cd ../rust-stb-image && make clean && make && cp -R *.dylib* ../${DIR}/lib/)

freetype:
	(cd ../rust-freetype && make clean && make && cp -R *.dylib* ../${DIR}/lib/)

# demo packing
demo-pack:
	cp build/${NAME} .
	tar -czf demo.tar.gz engine game data ${NAME}
	rm ${NAME}
