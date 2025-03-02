.PHONY:build

build:
	cmake . -Bbuild
	cmake --build ./build -j $(nproc)

