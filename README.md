# TrailBase Benchmarks

A set of micro-benchmarks comparing the performance of TrailBase, PocketBase,
SupaBase and raw SQLite (drizzle).

Generally, benchmarks are tricky, both to do well and to interpret.
Benchmarks never show how fast something can theoretically go but merely how
fast the author managed to make it go.
Micro-benchmarks, especially, offer a selective key-hole insights, which may be
biased and may or may not apply to your workload.

We tried our hardest to give all contenders the best chance to go fast.
If you spot any issues or have ideas to make anyone go faster, we want to know.
We hope to improve the methodology over time, make the numbers more broadly
applicable, and as fair as an apples-to-oranges comparison can be.
With that said, we hope that the results can provide at least some insights
into what to expect when taken with a grain of salt.
Ultimately, nothing beats benchmarking your own workload and setup.

## Reproduction

We're not quite there yet to set up and run all benchmarks automatically.
Except for SupaBase, which currently requires some manual setup,
setting up both TrailBase and PocketBase is fairly straight forward with the
checked-in runtime directories: `/setups`.
That said, there may be subtleties such as how the backend binaries under test
were built. For example, we found that building PocketBase with `CGO_ENABLED=1`
and `GOAMD64=v4`, yields roughly 20% better results.


Independently, the benchmark drivers under `/benchmarks` have some additional,
best-effort documentation with clear room  for improvement :hide:.
