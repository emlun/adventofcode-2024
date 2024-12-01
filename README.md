# Advent of Code 2024 solutions

To run the solutions:

```
$ cargo run
```

This assumes [Cargo][cargo] is installed, and that the input files are placed at
`inputs/dayXX.in` relative to the current working directory.

To run an individual day, specify the day as a command line argument:

```
$ cargo run 1
```

To run with a different input, specify a file name as a command line argument.
The file name `-` means standard input:

```
$ cargo run 1 foo.txt
$ cargo run 1 - < foo.txt
```

To run the benchmarks:

```
$ cargo bench
```


## License

[GNU General Public License][gpl], version 3 or later.


[cargo]: https://doc.rust-lang.org/stable/cargo/
[gpl]: https://www.gnu.org/licenses/#GPL
