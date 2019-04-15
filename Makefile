arch ?= x86_64
target ?= $(arch)-venix_kernel
kernel := build/kernel-$(arch)

linker_script := linker.ld
asm_source_files := $(wildcard src/*.asm)
asm_object_files := $(patsubst src/%.asm, \
	build/%.o, $(asm_source_files))

rust_source_files := $(wildcard src/*.rs)

rust_os := target/$(target)/debug/libecs_kernel.a

.PHONY: all clean

all: $(kernel)

clean:
	-@cargo clean
	-@rm -r build

$(kernel): $(rust_os) $(asm_object_files) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) $(asm_object_files) $(rust_os)

$(rust_os): $(rust_source_files)
	@cargo xbuild --target $(target).json

build/%.o: src/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
