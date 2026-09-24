# bucket

A devastatingly simple bucket list CLI and `.bucketlist` file format.

Install from this repo with [Rust](https://www.rust-lang.org/tools/install):

```sh
cargo install --path .
```

```sh
bucket init                         # create ~/.bucketlist
bucket add "Ride a hot-air balloon"
bucket list                         # show numbered items
bucket kick 1                       # mark item 1 complete
bucket remove 1                     # delete item 1
```

Your list lives in `~/.bucketlist`. It's plain UTF-8 text: one item per line,
with `,x` at the end when it's done. Edit it by hand if you like.

```text
Ride a hot-air balloon
Visit Luxembourg,x
Learn how to sail
```

See [SPEC.md](SPEC.md) for the file format. MIT licensed.
