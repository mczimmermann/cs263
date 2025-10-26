// overflow_benchmark.cpp
// Compile: g++ -std=c++11 -O3 overflow_benchmark.cpp -lbenchmark -lpthread -o cpp_bench
// Run: ./cpp_bench

#include <benchmark/benchmark.h>
#include <cstdint>
#include <cstdlib>

// Vulnerable: No overflow checking (undefined behavior for signed)
uint32_t calculate_buffer_size_unsafe(uint32_t width, uint32_t height) {
    return width * height * 4; // Can overflow silently
}

// Safe: Manual overflow checking
uint32_t calculate_buffer_size_safe(uint32_t width, uint32_t height) {
    // Check for overflow before multiplication
    if (width > UINT32_MAX / height / 4) {
        return 0; // Indicate error
    }
    return width * height * 4;
}

static void BM_Unsafe(benchmark::State& state) {
    uint32_t width = 1024;
    uint32_t height = 768;
    
    for (auto _ : state) {
        uint32_t size = calculate_buffer_size_unsafe(width, height);
        benchmark::DoNotOptimize(size);
    }
}

static void BM_Safe(benchmark::State& state) {
    uint32_t width = 1024;
    uint32_t height = 768;
    
    for (auto _ : state) {
        uint32_t size = calculate_buffer_size_safe(width, height);
        benchmark::DoNotOptimize(size);
    }
}

BENCHMARK(BM_Unsafe);
BENCHMARK(BM_Safe);

BENCHMARK_MAIN();