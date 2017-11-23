Run test in one thread:

```
cargo test -- --test-threads=1
```

Benchmark result:

```
test emd::bench::bench_emd_10  ... bench:       4,392 ns/iter (+/- 691)
test emd::bench::bench_emd_20  ... bench:       8,401 ns/iter (+/- 3,513)
test emd::bench::bench_emd_30  ... bench:      83,743 ns/iter (+/- 39,026)
test emd::bench::bench_emd_40  ... bench:      87,835 ns/iter (+/- 36,107)
test emd::bench::bench_emd_50  ... bench:     149,364 ns/iter (+/- 26,667)
test emd::bench::bench_emd_60  ... bench:     176,116 ns/iter (+/- 34,607)
test emd::bench::bench_emd_70  ... bench:     348,422 ns/iter (+/- 66,775)
test emd::bench::bench_emd_80  ... bench:     332,171 ns/iter (+/- 57,399)
test emd::bench::bench_emd_90  ... bench:     458,471 ns/iter (+/- 221,222)
test emd::bench::bench_emd_100 ... bench:     577,468 ns/iter (+/- 199,593)
```

just marshaling and prepare:

```
test emd::bench::bench_emd_10  ... bench:         730 ns/iter (+/- 119)
test emd::bench::bench_emd_20  ... bench:         986 ns/iter (+/- 449)
test emd::bench::bench_emd_30  ... bench:       1,423 ns/iter (+/- 252)
test emd::bench::bench_emd_40  ... bench:       1,708 ns/iter (+/- 263)
test emd::bench::bench_emd_50  ... bench:       2,025 ns/iter (+/- 279)
test emd::bench::bench_emd_60  ... bench:       2,276 ns/iter (+/- 1,014)
test emd::bench::bench_emd_70  ... bench:       2,849 ns/iter (+/- 480)
test emd::bench::bench_emd_80  ... bench:       2,796 ns/iter (+/- 1,167)
test emd::bench::bench_emd_90  ... bench:       3,857 ns/iter (+/- 1,761)
test emd::bench::bench_emd_100 ... bench:       3,862 ns/iter (+/- 1,792)
```
