# cs263

This project is focused on testing the performance differences between known vulnerable C++ code and their memory-safe counterparts in Rust. 

I will be making the comparisons using multiple Common Vulnerabilities & Exposures (CVEs) in [gnu coreutils](https://github.com/coreutils/coreutils), as it is an open source, very widely used program, and there is a current [popular reimplementation of it using Rust](https://github.com/uutils/coreutils), so I can easily compare the two. 
Here are some of the examples that I will be using:
* [CVE-2024-0684](https://www.cve.org/CVERecord?id=CVE-2024-0684), heap overflow in split, with proof of concept available [here](https://github.com/Valentin-Metz/writeup_split/tree/main)
* [CVE-2025-5278](https://www.cve.org/CVERecord?id=CVE-2025-5278), heap buffer under-read in chroot
* [CVE-2015-4042](https://www.cve.org/CVERecord?id=CVE-2015-4042), integer overflow in keycompare_mb() function in sort

The current plan is to use `google-benchmark` in order to test the performance of the C++ code, and use `criterion` to test the performance of the Rust code. For both languages, I will benchmark a "normal", non-exploit run, and for examples where a proof of concept is available, I will also benchmark the exploit. Note that overwhelmingly, users will only experience a normal execution path so this is sufficient. 
Furthermore, optimal performance is theoretically not necessary for an attacker that wants to trigger the vulnerabilities. 
