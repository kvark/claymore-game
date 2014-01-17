NAME=claymore
DIR=${NAME}-game
CODATA=util/codata
TRACE=RUST_LOG=rustc=1,::rt::backtrace
#RUST		?=${TRACE} rustc
RUST		?=rustc -Z debug-info
#RUST		?=../rust/x86_64-unknown-linux-gnu/stage0/bin/rustc
#LIBMASK=*.dylib*
LIBMASK		?=*.so
#GLTARGET=osx
GLTARGET	?=linux

# game/engine/demos code
.PHONY: game engine

all: game

run: game
	build/${NAME}
run-trace: game
	${TRACE} build/${NAME}
run-debug: game
	gdb build/${NAME}
run-memtest: game
	valgrind --leak-check=full --track-origins=yes build/${NAME} 2>build/memtest.log

grab-scene: asset/claymore/claymore-2a.rs asset/battle/battle-test.rs
	cp asset/claymore/claymore-2a.rs ${CODATA}/scene/chared/main.rs
	cp asset/claymore/claymore-2a/* data/scene/claymore-2a/
	cp asset/battle/battle-test.rs ${CODATA}/scene/battle/main.rs
	cp asset/battle/battle-test/* data/scene/battle-test/


game: build/claymore

build/claymore:	lib/engine.dummy lib/codata-scene.dummy lib/codata-hud.dummy lib/glfw.dummy game/*.rs game/battle/*.rs game/hud/*.rs game/render/*.rs game/scene/*.rs 
	${RUST} game/claymore.rs -L lib --out-dir build

engine: lib/engine.dummy

lib/engine.dummy: lib/gl.dummy lib/cgmath.dummy lib/openal.dummy lib/freetype.dummy lib/stb-image.dummy engine/*.rs engine/gr_low/*.rs engine/gr_mid/*.rs
	${RUST} engine/engine.rs -L lib --out-dir lib
	touch $@


codata: lib/codata-scene.dummy lib/codata-hud.dummy

lib/codata-scene.dummy: ${CODATA}/scene/*.rs ${CODATA}/scene/chared/*.rs ${CODATA}/scene/battle/*.rs
	${RUST} ${CODATA}/scene/scene.rs --out-dir lib
	touch $@

lib/codata-hud.dummy: ${CODATA}/hud/*.rs
	${RUST} ${CODATA}/hud/hud.rs --out-dir lib
	touch $@	


demo-03: lib/engine.dummy util/sample/$@*.rs
	${RUST} util/sample/demo03-materials.rs	-L lib --out-dir build
demo-04: lib/engine.dummy sample/$@*.rs
	${RUST} util/sample/demo04-skeleton.rs	-L lib --out-dir build
demo-05: lib/engine.dummy sample/$@*.rs
	${RUST} util/sample/demo05-text.rs		-L lib --out-dir build

clean:
	rm -Rf lib/* build/*

clean-engine:
	rm -R lib/libengine* lib/engine.dummy


# external libraries section
libs: lib/*.dummy

clean-libs:
	(cd lib && rm -Rf liblmath* libglfw3* libgl* openal* libstb* *.dummy)

lib/cgmath.dummy: ../cgmath-rs/src/cgmath/*.rs
	(cd ../cgmath-rs && rustpkg build cgmath && cp -Ru build/*/cgmath/lib* ../${DIR}/lib)
	touch $@
	
lib/glfw.dummy: ../glfw-rs/src/*.rs
	(cd ../glfw-rs/build && cmake .. && make && cp -Ru lib/lib* ../../${DIR}/lib/)
	touch $@

lib/gl.dummy: ../gl-rs/src/gen/*.r? ../gl-rs/src/gl/*.r?
	(cd ../gl-rs && rustpkg build gl && cp -Ru build/*/gl/lib* ../${DIR}/lib)
	touch $@

lib/openal.dummy: ../openal-rs/src/openal/*.rs
	(cd ../openal-rs && rustpkg build openal && cp -Ru build/*/openal/lib* ../${DIR}/lib)
	touch $@

lib/stb-image.dummy: ../rust-stb-image/*.rs ../rust-stb-image/*.c ../rust-stb-image/Makefile
	(cd ../rust-stb-image && make clean && make && cp -Ru ${LIBMASK} *.a ../${DIR}/lib/)
	touch $@

lib/freetype.dummy: ../rust-freetype/*.rs ../rust-freetype/Makefile
	(cd ../rust-freetype && make clean && make && cp -Ru ${LIBMASK} ../${DIR}/lib/)
	touch $@

# demo packing
demo-pack: demo.tar.gz

demo.tar.gz: build/claymore
	cp build/${NAME} .
	tar -czf $@ engine game data ${NAME}
	rm ${NAME}
