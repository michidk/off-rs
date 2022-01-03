off-rs
===

Parses `.off` files ([Object File Format](https://en.wikipedia.org/wiki/OFF_(file_format))).
This implementation follows [the spec](https://shape.cs.princeton.edu/benchmark/documentation/off_format.html) from the Princeton Shape Benchmark which is also explained [here](https://people.sc.fsu.edu/~jburkardt/data/off/off.html).

Sample file:

```off
OFF
# cube.off
# A cube

8 6 12
 1.0  0.0 1.4142
 0.0  1.0 1.4142
-1.0  0.0 1.4142
 0.0 -1.0 1.4142
 1.0  0.0 0.0
 0.0  1.0 0.0
-1.0  0.0 0.0
 0.0 -1.0 0.0
4  0 1 2 3  255 0 0 #red
4  7 4 0 3  0 255 0 #green
4  4 5 1 0  0 0 255 #blue
4  5 6 2 1  0 255 0
4  3 2 6 7  0 0 255
4  6 5 4 7  255 0 0
```
