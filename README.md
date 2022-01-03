off-rs
===

Parses `.off` files ([Object File Format](https://en.wikipedia.org/wiki/OFF_(file_format))).
This implementation follows the spec from the Princeton Shape Benchmark [the spec](https://people.sc.fsu.edu/~jburkardt/data/off/off.html)
Sample file:

```off
# this file header has to be the first instruction
OFF
# cube.off
# A cube

# 8 vertices, 6 faces, 12 edges
8 6 12

# vetex coordinates: x, y, z
  1.632993   0.000000   1.154701
  0.000000   1.632993   1.154701
 -1.632993   0.000000   1.154701
  0.000000  -1.632993   1.154701
  1.632993   0.000000  -1.154701
  0.000000   1.632993  -1.154701
 -1.632993   0.000000  -1.154701
  0.000000  -1.632993  -1.154701

# face indicies & RGBA color data: n, v1, v2, v3, v4, r, g, b, a
  4  0 1 2 3  1.000 0.000 0.000 0.75
  4  7 4 0 3  0.300 0.400 0.000 0.75
  4  4 5 1 0  0.200 0.500 0.100 0.75
  4  5 6 2 1  0.100 0.600 0.200 0.75
  4  3 2 6 7  0.000 0.700 0.300 0.75
  4  6 5 4 7  0.000 1.000 0.000 0.75
```
