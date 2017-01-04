# Benchmarking JS FFI

Testing complex struct types against json-serde.

## running

you need a rust stable setup. Then run:

```
$ npm start
```

It will compile the rust example code for you and run the benchmark, looking something like this (_Note_: the higher the numbers, the better.):


```
> json-performance@1.0.0 start /home/ben/dev/safenet/json-performance
> cargo build && node index.js

    Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
Simple with JSON x 17,687 ops/sec ±2.56% (84 runs sampled)
Complex Types x 15,874 ops/sec ±3.17% (81 runs sampled)
Fastest is Simple with JSON
```


## Explainer

Running the `Simple with JSON` (see `simple.js` for source code) against using `Complex Types` (see `complex.js`) doing the same thing: building everything to get ready to make an Auth-Request into an auth-url (not actually doing that).