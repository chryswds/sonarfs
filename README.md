# sonarfs is a disk management written in Rust

using this project as a way to learn rust 
all code written by me no AI

## Try it

To try it you will need cargo and rust installed

```bash
git clone https://github.com/chryswds/sonarfs
cd sonarfs
cargo run -- /path/to/directory
```

by default it scans the given directory and returns its content

## Flags

so far i've added the flags --top, --depth, --min-size and --ext

`--top <N>` | Show the N heaviest items (largest files/folders) in the directory | `cargo run -- /path --top 5`


`--depth <N>` | Show items nested N levels deep within the directory | `cargo run -- /path --depth 2`

`--min-size <N>` | Show only items bigger than threshold ( it works for both human readable (K, B, G) and bytes | `cargo run -- /path --min-size 300M` or `cargo run -- /path --min-size 300000000`

`--ext <N>` | Show only items that match the extension ( it works with multiple extensions as well )| `cargo run -- /path --ext rs or cargo run -- /path --ext rs,js,toml `

Flags can be combined:
 
```bash
cargo run -- /path/to/directory --top 5 --depth 2 --min-size 300M --ext mp4
```
