# PocketBase-Dart Benchmark Driver

admin: admin@bar.com
password: 1234567890

user: user@bar.com
password: 1234567890

## Some Results

$ /usr/bin/time ./bin/benchmark.exe
Inserted 100000 messages, took 0:01:02.813645 (limit=64)
16.06user 11.14system 1:03.01elapsed 43%CPU (0avgtext+0avgdata 144188maxresident)k
0inputs+0outputs (0major+35431minor)pagefaults 0swaps

$ dart run bin/benchmark.dart
Inserted 100000 messages, took 0:01:01.686601 (limit=64)


### Read Benchmark (2024-10-12)

Inserted 10000 messages, took 0:00:07.759677 (limit=64)
Latencies:
        p50=28160us
        p75=58570us
        p90=108325us
        p95=157601us
read message sanity check: {"id":"3haffu3xpya1dfi","created":"2024-10-12 10:19:48.756Z","updated":"2024-10-12 10:19:48.756Z","collectionId":"ng5nv7k52oysks3","collectionName":"message","expand
":{},"data":"a message 0","owner":"f0vvdewar3zh56f","room":"cnrhts2sjqo3h2p"}
Read 100000 messages, took 0:00:20.273054 (limit=64)
Latencies:
        p50=12740us
        p75=13718us
        p90=14755us
        p95=15495us
