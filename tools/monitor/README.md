# Record process utilization using `top`

This is a glorified wrapper around top and is somewhat brittle. Previously, we
were simply using `watch` + `top` with offline parsing of the output.
However, this led to inaccurate timing. Generally there's some variance with
`top` since it needs to sample (unlike `ps` et al, which simply show aggregate
stats).
