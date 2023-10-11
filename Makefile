CWD=$(shell pwd)
NAME=cosmos
TARGET=x86_64-${NAME}

IMAGE_PATH=$(CWD)/target/$(TARGET)/debug/bootimage-$(NAME).bin
KERNEL_PATH=$(CWD)/target/$(TARGET)/debug/$(NAME)

QEMU=qemu-system-x86_64
QEMU_FLAGS=-serial mon:stdio -monitor telnet:127.0.0.1:1234,server,nowait 

UNAME:=$(shell uname -s)
ifeq ($(UNAME),Linux)
	QEMU_FLAGS+=-curses
endif

.PHONY: all run clean

all: 
	cargo bootimage --target $(TARGET).json

build: 
	cargo build --target $(TARGET).json

run: $(IMAGE_PATH)
	$(QEMU) $(QEMU_FLAGS) -drive format=raw,file=$(IMAGE_PATH)

clean:
	@rm -rf ./target

