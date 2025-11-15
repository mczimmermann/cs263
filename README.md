# cs263

This project is focused on testing the performance differences between known vulnerable C++ code and their memory-safe counterparts in Rust. 

I will be making the comparisons using multiple Common Vulnerabilities & Exposures (CVEs) in [gnu coreutils](https://github.com/coreutils/coreutils) (see [versions here](https://gitweb.git.savannah.gnu.org/gitweb/?p=coreutils.git)), as it is an open source, very widely used program, and there is a current [popular reimplementation of it using Rust](https://github.com/uutils/coreutils), so I can easily compare the two. 
Here are some of the examples that I will be using:
* [CVE-2024-0684](https://www.cve.org/CVERecord?id=CVE-2024-0684), heap overflow in split (version 9.2), with proof of concept available [here](https://github.com/Valentin-Metz/writeup_split/tree/main)
* [CVE-2025-5278](https://www.cve.org/CVERecord?id=CVE-2025-5278), heap buffer under-read in sort (versions from 7.2 onwards)
* [CVE-2015-4042](https://www.cve.org/CVERecord?id=CVE-2015-4042), integer overflow in keycompare_mb() function in sort

The current plan is to use `google-benchmark` in order to test the performance of the C++ code, and use `criterion` to test the performance of the Rust code. For both languages, I will benchmark a "normal", non-exploit run, and for examples where a proof of concept is available, I will also benchmark the exploit. Note that overwhelmingly, users will only experience a normal execution path so this is sufficient. 
Furthermore, optimal performance is theoretically not necessary for an attacker that wants to trigger the vulnerabilities. 

### basic_idea 
This folder contains a small proof of concept for the larger project, done using a basic integer overflow. In order to run the C++ benchmark, you will need to run make all inside basic_idea/cplusplus. To run the rust benchmark, you will need to run cargo bench inside basic_idea/rust/benches. 

### cve-2024-0684
This folder contains Rust and C benchmarks of the coreutils function split(), which had a heap overflow vulnerability in coreutils version [9.2](https://gitweb.git.savannah.gnu.org/gitweb/?p=coreutils.git;a=commit;h=82bb131c551039971d6668dedc1975a770df88b9). In order to run the C benchmark correctly (because GNU coreutils usually creates a child process instead of linking like a Rust library), I have edited split.c to expose its main function and included it in this folder as well. ***Note that I have already committed the results from running this on my local, which will get overwritten if you run the following commands.***

In order to build and run the docker:

`docker build -t split_benchmark .`

`docker run --rm split_benchmark > results/output.log 2>&1`

This will create a log of the benchmark results. 


### cve-2025-5278
This folder contains Rust and C benchmarks of the coreutils function sort(), which had a heap buffer under-read vulnerability in coreutils versions [7.2](https://gitweb.git.savannah.gnu.org/gitweb/?p=coreutils.git;a=commit;h=3dcf7a4d23f328e1eb6dac3c1232b98ddc833354) onwards, until it was fixed in [this commit](https://cgit.git.savannah.gnu.org/cgit/coreutils.git/commit/?id=8c9602e3a145e9596dc1a63c6ed67865814b6633) in 2025. To keep this slightly more up to date, I will benchmark the same version (9.2) as cve-2024-0684.