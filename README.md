
# Trees-rs

<div>
  <img src="https://github.com/allaboutevemirolive/trees-rs/blob/master/icon/trees.png" width="100%" />
</div>


Trees-rs is a command-line tool developed in pure Rust, designed for displaying directory structures in ASCII format on the terminal or saving them to a text file. It serves as a seamless replacement for the [tree](https://github.com/Old-Man-Programmer/tree.git) utility written in C. The primary goal of this project is to provide a non-bloated and more functional alternative to the existing [tree](https://github.com/Old-Man-Programmer/tree.git).


## Benchmark

### System information for this benchmark

<details>
<summary>Click to expand!</summary>

```
OS version: Debian GNU/Linux 12 (bookworm)
RAM available: 14Gi (GiB, or gibibytes)
Hard disk: 28G (GiB)
Intel version: Intel(R) Core(TM) i5-8250U CPU @ 1.60GHz
SSD model: Samsung SSD 860 EVO 500GB
```

</details>


Our benchmark focuses on a set of forks of the Rust compiler project:

```
.../rustc_fork$ ls
cargo  miri  regex  rust  rust-analyzer  rust-clippy  rustfmt  rustup  wasmtime
```

This directory contains the following items (information gathered using our `Trees-rs` tool):

```
directories: 7234, files: 76915, hidden_files: 198, 
symlinks: 117, special_files: 0, total_items: 84266, size: 4.203 gigabytes
```

We will run a simple benchmark to compare the performance of the tools. To ensure accurate results, clear the system cache by running the following command:


```bash
sudo sync && sudo echo 3 | sudo tee /proc/sys/vm/drop_caches  
```

Run command :

```
time eza --tree rustc_fork
```

```
time tree rustc_fork
```

```
time trees-rs rustc_fork
```

### Results

| Tool | Run | Real Time | User Time | Sys Time |
|------|-----|-----------|-----------|----------|
| **[Eza](https://github.com/eza-community/eza)** | 1 | 0m6.433s | 0m4.703s | 0m2.665s |
|      | 2 | 0m6.277s | 0m4.647s | 0m2.622s |
|      | 3 | 0m6.701s | 0m5.041s | 0m2.713s |
|      | 4 | 0m6.651s | 0m5.038s | 0m2.725s |
|      | 5 | 0m6.312s | 0m4.440s | 0m2.611s |
| **[Tree](https://github.com/Old-Man-Programmer/tree)** | 1 | 0m3.273s | 0m0.925s | 0m0.941s |
|      | 2 | 0m3.229s | 0m0.881s | 0m0.980s |
|      | 3 | 0m3.113s | 0m0.755s | 0m0.978s |
|      | 4 | 0m4.349s | 0m0.836s | 0m0.992s |
|      | 5 | 0m3.168s | 0m0.783s | 0m0.955s |
| **[Trees-rs](https://github.com/allaboutevemirolive/trees-rs)** | 1 | 0m3.018s | 0m0.349s | 0m1.133s |
|      | 2 | 0m3.088s | 0m0.388s | 0m1.133s |
|      | 3 | 0m3.107s | 0m0.344s | 0m1.182s |
|      | 4 | 0m3.095s | 0m0.352s | 0m1.172s |
|      | 5 | 0m3.060s | 0m0.371s | 0m1.068s |


Detailed percentages and time differences:

| Comparison      | Real Time Difference | Real Time Percentage | User Time Difference | User Time Percentage | Sys Time Difference | Sys Time Percentage |
|-----------------|-----------------------|----------------------|----------------------|----------------------|---------------------|---------------------|
| Tree vs. Eza    | 3.049s                | 47.1% faster         | 3.938s               | 82.5% faster         | 1.698s              | 63.7% faster        |
| Trees-rs vs. Eza| 3.401s                | 52.5% faster         | 4.413s               | 92.4% faster         | 1.529s              | 57.4% faster        |
| Trees-rs vs. Tree| 0.352s               | 10.3% faster         | 0.475s               | 56.8% faster         | -0.169s             | -17.4% slower       |

### Summary

- **Overall, Trees-rs is the fastest in terms of real and user time but slightly slower than Tree in sys time.**
- **Tree is significantly faster than Eza in all three categories.**


## Installation

Run:

```
cargo install trees-rs
```
or you can build from source with:

```
cargo build --release
```

## Command Line Options

Tree-rs offers several command line options:

```
$ trees-rs -h
Usage: trees-rs [OPTIONS]

Options:
  -V, --version               Print current version of Tree-rs.
  -A, --absolute              Print file/dir name along with it absolute path
  -f, --relative              Print file/dir name along with it relative path
  -r, --reverse               Sort entires in ascending order.
  -S, --no-sort               No entries sort.
  -s, --sort                  Sort entries.
  -F, --filesfirst            Sort files first.
  -c, --color                 Print entries with color.
  -C, --color-less            Print entries without color.
      --visible               Print visible entries only.
      --all                   Print all entries.
      --folder                Print directoris only.
  -m, --meta                  Print all default entry's metadata.
  -p, --permission            Print entires attribute.
      --btime                 Print the date that the entry was created.
      --mtime                 Print the date that the entry was modified.
      --atime                 Print the date that the entry was last time accessed.
      --size                  Print entires's size.
  -L, --level <level-bounds>  Print tree until certain depth. Default depth: 5000
  -y, --yield                 Print exhaustive report
  -B, --nobranch              Discard branch's stick from the output
  -h, --help                  Print help


```

## Example Usage

### $ trees-rs src -m

<details>
<summary>Click to expand!</summary>

```
$ trees-rs src -m

drwxr-xr-x  25-04-2024 08:51  19-06-2024 18:37  19-06-2024 18:38     4096    src
 drwxr-xr-x  25-04-2024 08:51  19-06-2024 18:37  19-06-2024 18:38     4096    ├── cli
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     7286    │   ├── app.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 19:05  19-06-2024 19:05     6465    │   ├── arg.rs
 .rw-r--r--  15-05-2024 18:19  15-05-2024 18:19  19-06-2024 11:58      26     │   └── mod.rs
 drwxr-xr-x  25-04-2024 08:51  19-06-2024 18:37  19-06-2024 18:38     4096    ├── config
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     7009    │   ├── color.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     2726    │   ├── inspect.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37      81     │   ├── mod.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     5970    │   ├── registry.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     6205    │   ├── root.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     871     │   └── sorting.rs
 drwxr-xr-x  25-04-2024 08:51  30-04-2024 07:41  19-06-2024 13:16     4096    ├── error
 .rw-r--r--  25-04-2024 08:51  03-04-2024 10:14  19-06-2024 11:58      16     │   ├── mod.rs
 .rw-r--r--  30-04-2024 07:41  30-04-2024 07:41  19-06-2024 11:58     1860    │   └── simple.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     892     ├── main.rs
 drwxr-xr-x  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:38     4096    ├── render
 drwxr-xr-x  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:38     4096    │   ├── attr
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     1129    │   │   ├── atime.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     1120    │   │   ├── btime.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37      72     │   │   ├── mod.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     1129    │   │   ├── mtime.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     2286    │   │   ├── pms.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     869     │   │   └── size.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     1690    │   ├── buffer.rs
 drwxr-xr-x  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:38     4096    │   ├── entree
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     993     │   │   ├── dirr.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     1056    │   │   ├── filee.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     3045    │   │   ├── headd.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37      63     │   │   ├── mod.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     1051    │   │   └── symlinked.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37      46     │   └── mod.rs
 drwxr-xr-x  25-04-2024 08:51  19-06-2024 18:37  19-06-2024 18:38     4096    ├── report
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     378     │   ├── depth.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37      59     │   ├── mod.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37      82     │   ├── size.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     3645    │   ├── tail.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     152     │   └── widest.rs
 drwxr-xr-x  25-04-2024 08:51  19-06-2024 18:37  19-06-2024 18:38     4096    ├── tree
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     1846    │   ├── branch.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 19:05  19-06-2024 19:05     1453    │   ├── level.rs
 .rw-r--r--  25-04-2024 08:51  05-05-2024 13:50  19-06-2024 11:58      45     │   ├── mod.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     9747    │   └── node.rs
 drwxr-xr-x  25-04-2024 08:51  19-06-2024 18:37  19-06-2024 18:38     4096    └── walk
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37      31         ├── mod.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     6987        ├── trctxt.rs
 .rw-r--r--  19-06-2024 18:37  19-06-2024 18:37  19-06-2024 18:37     3017        └── visit.rs

directories: 10, files: 37, hidden_files: 0, symlinks: 0, special_files: 0, total_items: 47, size: 122358 bytes
```

</details>

### $ trees-rs src -f

<details>
<summary>Click to expand!</summary>


```
$ trees-rs src -f

src
├── src/cli
│   ├── src/cli/app.rs
│   ├── src/cli/arg.rs
│   └── src/cli/mod.rs
├── src/config
│   ├── src/config/color.rs
│   ├── src/config/inspect.rs
│   ├── src/config/mod.rs
│   ├── src/config/registry.rs
│   ├── src/config/root.rs
│   └── src/config/sorting.rs
├── src/error
│   ├── src/error/mod.rs
│   └── src/error/simple.rs
├── src/main.rs
├── src/render
│   ├── src/render/attr
│   │   ├── src/render/attr/atime.rs
│   │   ├── src/render/attr/btime.rs
│   │   ├── src/render/attr/mod.rs
│   │   ├── src/render/attr/mtime.rs
│   │   ├── src/render/attr/pms.rs
│   │   └── src/render/attr/size.rs
│   ├── src/render/buffer.rs
│   ├── src/render/entree
│   │   ├── src/render/entree/dirr.rs
│   │   ├── src/render/entree/filee.rs
│   │   ├── src/render/entree/headd.rs
│   │   ├── src/render/entree/mod.rs
│   │   └── src/render/entree/symlinked.rs
│   └── src/render/mod.rs
├── src/report
│   ├── src/report/depth.rs
│   ├── src/report/mod.rs
│   ├── src/report/size.rs
│   ├── src/report/tail.rs
│   └── src/report/widest.rs
├── src/tree
│   ├── src/tree/branch.rs
│   ├── src/tree/level.rs
│   ├── src/tree/mod.rs
│   └── src/tree/node.rs
└── src/walk
    ├── src/walk/mod.rs
    ├── src/walk/trctxt.rs
    └── src/walk/visit.rs

directories: 10, files: 37, hidden_files: 0, symlinks: 0, special_files: 0, total_items: 47, size: 122358 bytes

```

</details>


## Code of Conduct

See [CODE_OF_CONDUCT.md](https://github.com/allaboutevemirolive/trees-rs/blob/master/CODE_OF_CONDUCT.md).

## Licensing

TreeCraft is released under the [MIT License](https://github.com/allaboutevemirolive/trees-rs/blob/master/LICENSE.md). You are free to use, modify, and distribute this software in accordance with the terms of the license.


