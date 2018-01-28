The Rust wrapper of EMD to calculate vectors distance.

The primary purpose of the project is an experiment for a bridge between Rust and native.

[EMD idea and source code](http://ai.stanford.edu/%7Erubner/emd/default.htm).


Run test in one thread:

```
cargo test -- --test-threads=1
cargo test --features "dumb" -- --test-threads=1
```

Benchmark result:

```
cargo bench -- --test-threads=1

test emd::bench::bench_emd_10  ... bench:       1,901 ns/iter (+/- 314)
test emd::bench::bench_emd_20  ... bench:      12,725 ns/iter (+/- 1,965)
test emd::bench::bench_emd_30  ... bench:      81,693 ns/iter (+/- 13,987)
test emd::bench::bench_emd_40  ... bench:      63,049 ns/iter (+/- 11,429)
test emd::bench::bench_emd_50  ... bench:     155,406 ns/iter (+/- 34,618)
test emd::bench::bench_emd_60  ... bench:     186,968 ns/iter (+/- 36,237)
test emd::bench::bench_emd_70  ... bench:     280,873 ns/iter (+/- 44,314)
test emd::bench::bench_emd_80  ... bench:     275,569 ns/iter (+/- 115,778)
test emd::bench::bench_emd_90  ... bench:     482,147 ns/iter (+/- 89,098)
test emd::bench::bench_emd_100 ... bench:     833,342 ns/iter (+/- 145,012)
```

just marshaling and prepare:

```
cargo bench --features "dumb" -- --test-threads=1

test emd::bench::bench_emd_10  ... bench:         573 ns/iter (+/- 246)
test emd::bench::bench_emd_20  ... bench:         741 ns/iter (+/- 311)
test emd::bench::bench_emd_30  ... bench:         986 ns/iter (+/- 429)
test emd::bench::bench_emd_40  ... bench:       1,106 ns/iter (+/- 181)
test emd::bench::bench_emd_50  ... bench:       1,259 ns/iter (+/- 238)
test emd::bench::bench_emd_60  ... bench:       1,400 ns/iter (+/- 254)
test emd::bench::bench_emd_70  ... bench:       1,352 ns/iter (+/- 579)
test emd::bench::bench_emd_80  ... bench:       1,395 ns/iter (+/- 622)
test emd::bench::bench_emd_90  ... bench:       1,521 ns/iter (+/- 294)
test emd::bench::bench_emd_100 ... bench:       1,659 ns/iter (+/- 328)
```
