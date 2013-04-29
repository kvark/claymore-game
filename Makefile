NAME=claymore
DIR=${NAME}-game
TRACE=RUST_LOG=rustc=1,::rt::backtrace
#RUST=${TRACE} rustc -S
RUST=rustc
#LIBMASK=*.dylib*
LIBMASK=*.so *.a
#GLTARGET=osx
GLTARGET=linux

# game/engine/demos code
all: engine-code game-code

run:
	build/claymore
run-trace:
	${TRACE} build/claymore

game-code:
	${RUST} game/claymore.rs -L lib --out-dir build

engine-code:
	${RUST} engine/engine.rs -L lib --out-dir lib

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
libs: clean-libs numeric lmath cgmath glfw3 glcore openal stb-image freetype

clean-libs:
	(cd lib && rm -Rf liblmath* libglfw3* libglcore* openal* libstb*)

numeric:
	(cd ../numeric-rs && rustc src/numeric.rs --out-dir ../${DIR}/lib/)

lmath:
	(cd ../lmath-rs && rustc src/lmath.rs -L ../${DIR}/lib/ --out-dir ../${DIR}/lib/)

cgmath:
	(cd ../cgmath-rs && rustc src/cgmath.rs -L ../${DIR}/lib/ --out-dir ../${DIR}/lib/)
	
glfw3:
	(cd ../glfw3-rs && make clean && make && cp -R lib/* ../${DIR}/lib/)

glcore:
	(cd ../glcore-rs && make clean && make ${GLTARGET} && cp -R lib/* ../${DIR}/lib/)

openal:
	(cd ../openal-rs && rustc src/openal.rs --out-dir ../${DIR}/lib/)

stb-image:
	(cd ../rust-stb-image && make clean && make && cp -R ${LIBMASK} ../${DIR}/lib/)

freetype:
	(cd ../rust-freetype && make clean && make && cp -R ${LIBMASK} ../${DIR}/lib/)

# demo packing
demo-pack:
	cp build/${NAME} .
	tar -czf demo.tar.gz engine game data ${NAME}
	rm ${NAME}
