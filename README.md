# foto-core

## About
Version 0.1.0

## Thinking
raw画像をjpegとかにエンコードしてからHash化しようと考えていたが、
Release0.x.xではrawも関係なく、そのままHash化することにする。

## Usese Crates
- [rawloader](https://crates.io/crates/rawloader)
- [bayer](https://crates.io/crates/bayer)

## Document

### Image
[new]
make Image(Self) from file's path(String).

[rename]
rename image file from Image(Self).

[add]
add Images() from Image(Self).

[rm]
remove Images() from Image(Self).
