# Drizzle Benchmark Setup

Note that unlike PocketBase's, SupaBase's and TrailBase's setup this is
in-process sqlite meant as a point of reference, how fast things could go w/o
the IPC, feature, implementation, ... overhead.


## Some Result

/usr/bin/time pnpm run start

> drizzle@1.0.0 start /home/sebastian/projects/trailbase-benchmarks/benchmarks/drizzle
> tsc && node dist/index.js

Inserted 100000 messages, took 8.803s (limit=64)
Cross-check 200000 messages in DB
6.26user 4.70system 0:10.32elapsed 106%CPU (0avgtext+0avgdata 727164maxresident)k
0inputs+2900736outputs (0major+236204minor)pagefaults 0swaps
