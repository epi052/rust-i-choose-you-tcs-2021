CC=gcc
# the rustc command below was used to determine what other static libraries were needed to link in order to
# statically link our rust library
# 	rustc --print native-static-libs --crate-type=staticlib include/data_manager/src/lib.rs
LIBS=-lbug_tracker -lgcc_s -lutil -lrt -lpthread -lm -ldl -lc
BUILD_DIR=./build
CC_FLAGS=-L $(BUILD_DIR) $(LIBS)
EXE=rust-caller
STATIC_LIB=libbug_tracker.a
HEADER=$(BUILD_DIR)/bug_tracker.h
REL_TYPE=release

# generating c headers from rust requires cbindgen, install with the command below
# 	cargo install --force cbindgen
# --force just makes it update to the latest cbindgen if it's already installed

build: lib.rs main.o
	$(CC) $(BUILD_DIR)/main.o -o $(EXE) $(CC_FLAGS)

main.o: main.c
	$(CC) -c main.c -o $(BUILD_DIR)/main.o

lib.rs:
	mkdir -p $(BUILD_DIR)
	cbindgen --config include/cbindgen.toml --output $(BUILD_DIR)/bug_tracker.h include/bug_tracker
	cargo build --$(REL_TYPE)
	cp ../target/$(REL_TYPE)/$(STATIC_LIB) $(BUILD_DIR)

.PHONY: clean

clean:
	rm -rf $(EXE) $(BUILD_DIR)