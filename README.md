# cs263

This project is focused on testing the performance differences between known exploits in C++ and their memory-safe counterparts in Rust. 

I will be making the comparisons using multiple Common Vulnerabilities & Exposures (CVEs) in [gnu coreutils](https://github.com/coreutils/coreutils), as it is an open source, very widely used program, and there is a current [popular reimplementation of it using Rust](https://github.com/uutils/coreutils), so I can easily compare the two. 
Here are some of the examples that I will be using:
* [CVE-2024-0684](https://www.cve.org/CVERecord?id=CVE-2024-0684), heap overflow in split, with proof of concept available [here](https://github.com/Valentin-Metz/writeup_split/tree/main)
* [CVE-2025-5278](https://www.cve.org/CVERecord?id=CVE-2025-5278), heap buffer under-read in chroot
* [CVE-2015-4042](https://www.cve.org/CVERecord?id=CVE-2015-4042), integer overflow in keycompare_mb() function in sort

### basic_idea 
This folder contains a small proof of concept for the larger project, done using a basic integer overflow. In order to run the C++ benchmark, you will need to run make all inside basic_idea/cplusplus. To run the rust benchmark, you will need to run cargo bench inside basic_idea/rust/benches. 
