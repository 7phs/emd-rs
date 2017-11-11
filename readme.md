Run test in one thread:

```
cargo test -- --test-threads=1
```

Benchmark result:

```
test emd::bench::bench_emd_10  ... bench:       3,777 ns/iter (+/- 1,662)
test emd::bench::bench_emd_20  ... bench:      14,753 ns/iter (+/- 6,822)
test emd::bench::bench_emd_30  ... bench:      41,329 ns/iter (+/- 12,292)
test emd::bench::bench_emd_40  ... bench:      64,886 ns/iter (+/- 24,790)
test emd::bench::bench_emd_50  ... bench:     174,350 ns/iter (+/- 35,305)
test emd::bench::bench_emd_60  ... bench:     271,060 ns/iter (+/- 52,347)
test emd::bench::bench_emd_70  ... bench:     207,159 ns/iter (+/- 31,732)
test emd::bench::bench_emd_80  ... bench:     514,544 ns/iter (+/- 86,013)
test emd::bench::bench_emd_90  ... bench:     456,785 ns/iter (+/- 88,043)
test emd::bench::bench_emd_100 ... bench:     592,605 ns/iter (+/- 97,452)
```