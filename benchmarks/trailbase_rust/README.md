# Insertion benchmark

Same as the typescript version, just wanted to see what the limiting factor is:
server or single javascript thread or IO?

There's a difference but it's not too dramatic (an earlier version had a quiet
403 error, which made the gap appear much wider).

    Inserted 160000 rows in 21.382570671s

Which is roughly 134us per insert. We're not maxing out the CPU, we're
bottlenecked somewhere else. Maybe I/O but it doesn't look like it. It's more
likely sqlite lock congestion. Would be interesting how much impact logging
has.

As opposed to javascript's:

    Inserted 100000 messages, took 21.518s (limit=64)

or 215us per insert.

Is the 3+x performance lead we see for ferrobase over pocketbase (and supabase)
soley client-side overhead by the javascript client library?
