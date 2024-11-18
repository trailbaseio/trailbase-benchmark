# Payload CMS Benchmark

Using default sqlite integration, WAL is **not** enabled leading to:

Inserted 100000 messages, took 1989.648s (limit=64)
92.82user 10.26system 33:10.53elapsed 5%CPU (0avgtext+0avgdata 513388maxresident)k
0inputs+40outputs (0major+201155minor)pagefaults 0swaps


With manually enabling WAL:

```
sqlite> PRAGMA journal_mode       = WAL;
sqlite> PRAGMA journal_size_limit = 200000000;
sqlite> PRAGMA busy_timeout       = 10000;
```

Inserted 10000 messages, took 43.858s (limit=64)
7.66user 0.80system 0:44.73elapsed 18%CPU (0avgtext+0avgdata 194372maxresident)k
0inputs+72outputs (0major+94251minor)pagefaults 0swaps

Inserted 100000 messages, took 647.907s (limit=64)
64.78user 8.49system 10:48.84elapsed 11%CPU (0avgtext+0avgdata 738448maxresident)k
0inputs+8outputs (0major+265835minor)pagefaults 0swaps

Inserted 100000 messages, took 656.088s (limit=64)
68.49user 9.36system 10:57.22elapsed 11%CPU (0avgtext+0avgdata 736692maxresident)k
9872inputs+72outputs (0major+289774minor)pagefaults 0swaps
