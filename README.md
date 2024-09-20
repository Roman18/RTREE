# RTREE
## Rusty version of tree unix tool


### Build
```cmd
cargo build --release --quiet
```

### Help
```cmd
./rtree --help
Rusty version of tree unix tool

Usage: rtree.exe [OPTIONS] [NAME]

Arguments:
  [NAME]  [default: .]

Options:
  -d, --depth <DEPTH>  [default: 2]
  -a, --all
  -h, --help           Print help
```

### Usage
```cmd
./rtree src -d 2
F: "argparse.rs"
F: "main.rs"
```