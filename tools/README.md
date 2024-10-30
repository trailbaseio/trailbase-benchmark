# Benchmark Tools

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
