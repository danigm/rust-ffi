FLAGS= -Wl,--gc-sections -lpthread
LIBDIR=target/debug/

all: src/example1.c src/example2.c target/debug/librustffi.a
	gcc $(FLAGS) src/example1.c -L$(LIBDIR) -lrustffi -o example1
	gcc $(FLAGS) src/example2.c -L$(LIBDIR) -lrustffi -o example2

target/debug/librustffi.a: src/lib.rs
	cargo build
