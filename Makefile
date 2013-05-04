NAME=claymore
DIR=${NAME}-game
TRACE=RUST_LOG=rustc=1,::rt::backtrace
#RUST		?=${TRACE} rustc
RUST		?=rustc
#LIBMASK=*.dylib*
LIBMASK		?=*.so
#GLTARGET=osx
GLTARGET	?=linux

# game/engine/demos code
.PHONY: game engine

all: game

run: game
	build/${NAME}
run-trace:	game
	${TRACE} build/${NAME}


game: build/claymore

build/claymore:	lib/engine.dummy lib/numeric.dummy lib/lmath.dummy lib/cgmath.dummy game/*.rs game/render/*.rs
	${RUST} game/claymore.rs -L lib --out-dir build

engine: lib/engine.dummy

lib/engine.dummy: lib/lmath.dummy lib/openal.dummy lib/freetype.dummy lib/stb-image.dummy engine/*.rs
	${RUST} engine/engine.rs -L lib --out-dir lib
	touch $@


demo-03: lib/engine.dummy sample/$@*.rs
	${RUST} sample/demo03-materials.rs	-L lib --out-dir build
demo-04: lib/engine.dummy sample/$@*.rs
	${RUST} sample/demo04-skeleton.rs	-L lib --out-dir build
demo-05: lib/engine.dummy sample/$@*.rs
	${RUST} sample/demo05-text.rs		-L lib --out-dir build

clean:
	rm -Rf lib/* build/*

clean-engine:
	rm -R lib/libengine* lib/engine.dummy


# external libraries section
libs: lib/*.dummy

clean-libs:
	(cd lib && rm -Rf liblmath* libglfw3* libglcore* openal* libstb* *.dummy)

lib/numeric.dummy: ../numeric-rs/src/*.rs
	(cd ../numeric-rs && rustc src/numeric.rs --out-dir ../${DIR}/lib/)
	touch $@

lib/lmath.dummy: lib/numeric.dummy ../lmath-rs/src/*.rs
	(cd ../lmath-rs && rustc src/lmath.rs -L ../${DIR}/lib/ --out-dir ../${DIR}/lib/)
	touch $@

lib/cgmath.dummy: lib/lmath.dummy ../cgmath-rs/src/*.rs
	(cd ../cgmath-rs && rustc src/cgmath.rs -L ../${DIR}/lib/ --out-dir ../${DIR}/lib/)
	touch $@
	
lib/glfw3.dummy: ../glfw3-rs/src/*.rs ../glfw3-rs/src/support/*.rs
	(cd ../glfw3-rs && make && cp -Ru lib/* ../${DIR}/lib/)
	touch $@

lib/glcore.dummy: ../glcore-rs/src/*.r?
	(cd ../glcore-rs && && make ${GLTARGET} && cp -Ru lib/* ../${DIR}/lib/)
	touch $@

lib/openal.dummy: ../openal-rs/src/*.rs
	(cd ../openal-rs && rustc src/openal.rs --out-dir ../${DIR}/lib/)
	touch $@

lib/stb-image.dummy: ../rust-stb-image/*.dummy
	(cd ../rust-stb-image && make && cp -Ru ${LIBMASK} *.a ../${DIR}/lib/)
	touch $@

lib/freetype.dummy: ../rust-freetype/*.dummy
	(cd ../rust-freetype && make && cp -Ru ${LIBMASK} ../${DIR}/lib/)
	touch $@

# demo packing
demo-pack: demo.tar.gz

demo.tar.gz: build/claymore
	cp build/${NAME} .
	tar -czf $@ engine game data ${NAME}
	rm ${NAME}
