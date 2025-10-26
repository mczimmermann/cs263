TARGET = cpp_bench

all: overflow_benchmark.cpp
	g++ -std=c++11 -O3 overflow_benchmark.cpp \
	-I$(shell brew --prefix google-benchmark)/include \
    -L$(shell brew --prefix google-benchmark)/lib \
	-lbenchmark -lpthread -o $(TARGET)

clean:
	rm -f $(TARGET)