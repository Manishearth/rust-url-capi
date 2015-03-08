all:
	cd .. && cargo build
	gcc -Wall -o test test.c ../target/librust-url-capi* -ldl -lpthread -lrt -lgcc_s -lpthread -lc -lm
	./test