# Complex sequences.

## Introduction
Represent and draw complex sequences, creating beautiful fractals.
Using `imgui`.

There are different types:
- parametric fractals, like the Mandelbrot's,
- seed-fractals, like Julia's or Newton's.

Here they are divided in families:
- Divergence based fractals: we study the speed of a recursive sequence
	- Mandelbrot;
	- Julia;
	- Exponent swap.
- Root based fractals: we try to find root or specified values.
	- Newton.

## Sources.
3Blue1Brown 
> Holomorphic dynamics series [Youtube](https://www.youtube.com/watch?v=LqbZpur38nw).

## Using the tool.
The `Settings` window.
- Choose the fractal family, type.
- Colorize with HSV or gray scale.
- Change viewport parameters.
- Customize the fractal.

Mouse:
- Middle click to center on cursor.
- Scroll to zoom (exponentially).

## Build and run.
Running on Windows:
```bash
./complex_sequences_<your os>_v<your version>.exe
```

Running on Linux:
```bash
./complex_sequences_<your os>_v<your version>
```

Building it yourself. In the root of the project:
```
cargo build -r
```

## Notes.
Multithreading:
- program will determine by itself the threads it should use with:
```rust
std::thread::available_parallelism();
```
- most of the time it will use **all of machine cores**, other apps will lag.

Error on data sizes mismatch. If this occurs:
- a message like `(X) Expected size and `data` size mismatch.` will be printed;
- pixels might get popped out of the pixels array;
- black pixels might be appended at the end of the pixel array;
