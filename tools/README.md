# Benchmark Tools

Tools for facilitating TrailBase benchmarks. Currently there are two tools:

* process.ts `DEPRECATED`
* monitor

`monitor` replaces the deprecated `process.ts`. It combines both the watch-like
execution and parsing in one. It also adds an explicit `elapsed` time column to
the data points. Previously, we had to infer the wall time, which was very
unreliable given that `top` itself has to sample every time it is being called
leading to variable tick rates.

## process.ts `DEPRECATED`

Currently only a tool for parsing utilization measurements and turning them
into JSON.

Input is except in the form of `top` output, which can be gathered as follows:

```bash
$ watch -n 0.5 "top -n 1 -b -p $(pgrep trail) | tail -n +8 | tee -a out.txt"
```

Afterwards, the JSON can be generated running

```bash
$ node --experimental-strip-types process.ts <path>/out.txt out.json
```
