Results:

```
Every iteration calls the function 10k times, so time per call is (ns/iter)/10000.

// normal usage of cursor+byteorder
test tests::bench_byteorder_cursor           ... bench:      75,715 ns/iter (+/- 5,123)

// manually inlined range checks
test tests::bench_byteorder_cursor_v2        ... bench:      53,481 ns/iter (+/- 2,844)

// manually inlined range checks + removed implicit range checks with `unsafe`
test tests::bench_byteorder_cursor_v2_unsafe ... bench:      41,539 ns/iter (+/- 2,740)


// normal usage of slice+byteorder
test tests::bench_byteorder_slice            ... bench:      84,032 ns/iter (+/- 3,409)

// manually inlined range checks
test tests::bench_byteorder_slice_v2         ... bench:      49,097 ns/iter (+/- 2,885)
```