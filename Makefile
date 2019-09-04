ifeq ($(OS),Windows_NT)
	HOST_OS := windows
	PROGRAM := program.exe
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        HOST_OS := linux
    endif
    ifeq ($(UNAME_S),Darwin)
        HOST_OS := darwin
    endif
	PROGRAM := program
endif

ifndef VERSION
	VERSION := $(shell sed -n 3p ./Cargo.toml)
endif

.PHONY: update upgrade build run

update:
	cargo update

upgrade:
	cargo upgrade

build:
	echo ${VERSION}
	cargo build

run:
	RUST_LOG=info ./target/debug/${PROGRAM}
