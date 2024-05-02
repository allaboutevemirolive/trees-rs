
# Tree-rs

Tree-rs is a command-line tool developed in pure Rust, designed for displaying directory structures in ASCII format on the terminal or saving them to a text file. It serves as a seamless replacement for the [tree](https://github.com/Old-Man-Programmer/tree.git) utility written in C. The primary goal of this project is to provide a non-bloated and more functional alternative to the existing [tree](https://github.com/Old-Man-Programmer/tree.git).

## Command Line Options

Tree-rs offers several command line options:

```
Usage: tree-rs [OPTIONS]

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
  -h, --help                  Print help

```

## Example Usage


```
$ trs src -m

 drwxr-xr-x  25-04-2024 08:51  16-04-2024 12:42  01-05-2024 19:03     4096    src
 drwxr-xr-x  25-04-2024 08:51  16-04-2024 14:25  01-05-2024 19:03     4096    ├── canva
 .rw-r--r--  25-04-2024 08:51  12-04-2024 09:05  01-05-2024 19:03      67     │   ├── ansi.rs
 .rw-r--r--  25-04-2024 08:51  01-05-2024 19:10  01-05-2024 19:10     2609    │   ├── buffer.rs
 .rw-r--r--  25-04-2024 08:51  28-03-2024 16:24  01-05-2024 19:03      0      │   ├── mmap.rs
 .rw-r--r--  25-04-2024 08:51  16-04-2024 14:29  01-05-2024 19:03     417     │   ├── mod.rs
 drwxr-xr-x  25-04-2024 08:51  16-04-2024 14:09  01-05-2024 19:03     4096    │   └── which
 drwxr-xr-x  25-04-2024 08:51  16-04-2024 13:52  01-05-2024 19:03     4096    │       ├── attr
 .rw-r--r--  25-04-2024 08:51  01-05-2024 19:16  01-05-2024 19:16     1104    │       │   ├── atime.rs
 .rw-r--r--  25-04-2024 08:51  01-05-2024 19:17  01-05-2024 19:17     1093    │       │   ├── btime.rs
 .rw-r--r--  25-04-2024 08:51  16-04-2024 14:13  01-05-2024 19:03      72     │       │   ├── mod.rs
 .rw-r--r--  25-04-2024 08:51  01-05-2024 19:17  01-05-2024 19:17     1098    │       │   ├── mtime.rs
 .rw-r--r--  25-04-2024 08:51  16-04-2024 13:38  01-05-2024 19:03     2352    │       │   ├── pms.rs
 .rw-r--r--  25-04-2024 08:51  16-04-2024 13:50  01-05-2024 19:03     1412    │       │   └── size.rs
 drwxr-xr-x  25-04-2024 08:51  16-04-2024 14:25  01-05-2024 19:03     4096    │       ├── entree
 .rw-r--r--  25-04-2024 08:51  16-04-2024 14:14  01-05-2024 19:03      0      │       │   ├── dirr.rs
 .rw-r--r--  25-04-2024 08:51  16-04-2024 14:16  01-05-2024 19:03     1659    │       │   ├── filee.rs
 .rw-r--r--  25-04-2024 08:51  16-04-2024 14:25  01-05-2024 19:03     3419    │       │   ├── headd.rs
 .rw-r--r--  25-04-2024 08:51  16-04-2024 14:14  01-05-2024 19:03      44     │       │   └── mod.rs
 .rw-r--r--  25-04-2024 08:51  16-04-2024 14:11  01-05-2024 19:03      30     │       └── mod.rs
 drwxr-xr-x  25-04-2024 08:51  16-04-2024 12:42  01-05-2024 19:03     4096    ├── cli
 .rw-r--r--  25-04-2024 08:51  02-05-2024 05:03  02-05-2024 05:03     6618    │   ├── app.rs
 .rw-r--r--  25-04-2024 08:51  02-05-2024 05:06  02-05-2024 05:06     6456    │   ├── arg.rs
 .rw-r--r--  25-04-2024 08:51  04-04-2024 18:20  01-05-2024 19:03      39     │   ├── mod.rs
 .rw-r--r--  25-04-2024 08:51  16-04-2024 11:55  01-05-2024 19:03     651     │   └── opt.rs
 drwxr-xr-x  25-04-2024 08:51  16-04-2024 14:25  01-05-2024 19:03     4096    ├── config
 .rw-r--r--  25-04-2024 08:51  01-04-2024 02:36  01-05-2024 19:03      32     │   ├── mod.rs
 .rw-r--r--  25-04-2024 08:51  02-05-2024 07:38  02-05-2024 07:38     6472    │   ├── path.rs
 .rw-r--r--  25-04-2024 08:51  02-05-2024 05:04  02-05-2024 05:04     5101    │   └── registry.rs
 drwxr-xr-x  25-04-2024 08:51  30-04-2024 07:41  01-05-2024 19:03     4096    ├── error
 .rw-r--r--  25-04-2024 08:51  03-04-2024 10:14  01-05-2024 19:03      16     │   ├── mod.rs
 .rw-r--r--  30-04-2024 07:41  30-04-2024 07:41  01-05-2024 19:03     1860    │   └── simple.rs
 .rw-r--r--  25-04-2024 08:51  02-05-2024 06:49  02-05-2024 06:49     2047    ├── main.rs
 drwxr-xr-x  25-04-2024 08:51  16-04-2024 11:53  01-05-2024 19:03     4096    ├── report
 .rw-r--r--  25-04-2024 08:51  01-05-2024 19:11  01-05-2024 19:11     900     │   ├── mod.rs
 .rw-r--r--  25-04-2024 08:51  14-04-2024 03:30  01-05-2024 19:03     1675    │   └── tail.rs
 drwxr-xr-x  25-04-2024 08:51  16-04-2024 13:35  01-05-2024 19:03     4096    ├── sort
 .rw-r--r--  25-04-2024 08:51  02-05-2024 07:34  02-05-2024 07:34     988     │   ├── dent.rs
 .rw-r--r--  25-04-2024 08:51  31-03-2024 19:42  01-05-2024 19:03      14     │   └── mod.rs
 drwxr-xr-x  25-04-2024 08:51  16-04-2024 11:55  01-05-2024 19:03     4096    ├── tree
 .rw-r--r--  25-04-2024 08:51  16-04-2024 11:53  01-05-2024 19:03     1479    │   ├── branch.rs
 .rw-r--r--  25-04-2024 08:51  14-04-2024 03:34  01-05-2024 19:03     981     │   ├── level.rs
 .rw-r--r--  25-04-2024 08:51  16-04-2024 11:55  01-05-2024 19:03     1480    │   ├── mod.rs
 .rw-r--r--  25-04-2024 08:51  30-04-2024 12:48  01-05-2024 19:03     8999    │   └── node.rs
 drwxr-xr-x  25-04-2024 08:51  01-04-2024 22:16  01-05-2024 19:03     4096    ├── util
 .rw-r--r--  25-04-2024 08:51  01-04-2024 22:16  01-05-2024 19:03      0      │   └── mod.rs
 drwxr-xr-x  25-04-2024 08:51  16-04-2024 11:53  01-05-2024 19:03     4096    └── walk
 .rw-r--r--  25-04-2024 08:51  02-05-2024 07:26  02-05-2024 07:26     3149        ├── metada.rs
 .rw-r--r--  25-04-2024 08:51  02-05-2024 07:25  02-05-2024 07:25     4660        └── mod.rs

13 directories, 36 files, 0 hidden, 0.00 gigabytes 

```


```
$ trs src -f

src
├── src/canva
│   ├── src/canva/ansi.rs
│   ├── src/canva/buffer.rs
│   ├── src/canva/mmap.rs
│   ├── src/canva/mod.rs
│   └── src/canva/which
│       ├── src/canva/which/attr
│       │   ├── src/canva/which/attr/atime.rs
│       │   ├── src/canva/which/attr/btime.rs
│       │   ├── src/canva/which/attr/mod.rs
│       │   ├── src/canva/which/attr/mtime.rs
│       │   ├── src/canva/which/attr/pms.rs
│       │   └── src/canva/which/attr/size.rs
│       ├── src/canva/which/entree
│       │   ├── src/canva/which/entree/dirr.rs
│       │   ├── src/canva/which/entree/filee.rs
│       │   ├── src/canva/which/entree/headd.rs
│       │   └── src/canva/which/entree/mod.rs
│       └── src/canva/which/mod.rs
├── src/cli
│   ├── src/cli/app.rs
│   ├── src/cli/arg.rs
│   ├── src/cli/mod.rs
│   └── src/cli/opt.rs
├── src/config
│   ├── src/config/mod.rs
│   ├── src/config/path.rs
│   └── src/config/registry.rs
├── src/error
│   ├── src/error/mod.rs
│   └── src/error/simple.rs
├── src/main.rs
├── src/report
│   ├── src/report/mod.rs
│   └── src/report/tail.rs
├── src/sort
│   ├── src/sort/dent.rs
│   └── src/sort/mod.rs
├── src/tree
│   ├── src/tree/branch.rs
│   ├── src/tree/level.rs
│   ├── src/tree/mod.rs
│   └── src/tree/node.rs
├── src/util
│   └── src/util/mod.rs
└── src/walk
    ├── src/walk/metada.rs
    └── src/walk/mod.rs

13 directories, 36 files, 0 hidden, 0.00 gigabytes 

```
