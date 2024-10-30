# TrailBase-Dart Benchmark Driver

## Some Results

###  AOT

Inserted 100000 messages, took 0:00:11.161471 (limit=64)
9.31user 2.54system 0:14.24elapsed 83%CPU (0avgtext+0avgdata 149376maxresident)k
10048inputs+0outputs (2major+36286minor)pagefaults 0swaps

### JIT

Inserted 100000 messages, took 0:00:09.662269 (limit=64)
Inserted 100000 messages, took 0:00:09.424714 (limit=64)

#### Old
Inserted 100000 messages, took 0:00:10.053480 (limit=64)
11.55user 2.64system 0:15.62elapsed 90%CPU (0avgtext+0avgdata 371756maxresident)k
192736inputs+0outputs (25major+117365minor)pagefaults 0swaps

With PGO:

Inserted 100000 messages, took 0:00:10.054263 (limit=64)

### Server's /usr/bin/time output

18.79user 13.74system 0:43.17elapsed 75%CPU (0avgtext+0avgdata 101512maxresident)k
448inputs+7156304outputs (0major+34589minor)pagefaults 0swaps

### Read Benchmark (2024-10-12)

Inserted 10000 messages, took 0:00:01.654810 (limit=64)
Latencies:
        p50=8107us
        p75=10897us
        p90=15327us
        p95=19627us
read message sanity check: {data: a message 0, id: AZKALvAKfmGoD_1kQxcCfQ==, room: AZH8mYTFd5OexZn4K10jCA==}
Read 1000000 messages, took 0:00:57.952120 (limit=64)
Latencies:
        p50=3504us
        p75=3947us
        p90=4393us
        p95=4725us

## Server utilization

Note that we're using top since `ps` outputs cumulative CPU usage.

```bash
$ watch -n 0.5 "top -n 1 -b -p $(pgrep trail) | tail -n +8 | tee -a out.txt"
```

The output can be processed using <repo>/tools via:

```bash
$ cd tools
$ node --experimental-strip-types process.ts ../benchmarks/trailbase_dart/out.txt trailbase.json
```
