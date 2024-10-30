# Insertion benchmark

Interestingly, the performance seems to be very similar to the javascript
version. The unofficial rust pocketbase SDK doesn't use reqwest and is sync but
we should still be faster unless we're maxing out pocketbase?

Got:

    Inserted 160000 rows in 102.193818264s

, as opposed to:


    Inserted 100000 messages, took 61.33s (limit=64)

in javascript. Compared to ferrobase with:

    Inserted 160000 rows in 6.574632503s

, this would put ferrobase at 15x faster.
