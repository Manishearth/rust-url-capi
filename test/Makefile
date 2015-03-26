all:
	cd .. && cargo build
	g++ -Wall -o test test.cpp ../target/debug/librust-url-capi-a86c106dff114e83.a -ldl -lpthread -lrt -lgcc_s -lpthread -lc -lm
	./test
