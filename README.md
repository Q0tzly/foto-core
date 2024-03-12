# foto-core

## About
Version 0.1.0

## Thinking
raw画像をjpegとかにエンコードしてからHash化しようと考えていたが、
Release0.x.xではrawも関係なく、そのままHash化することにする。

写真のメタデータはExif情報を確認する。
Exif情報が含まれていない場合は、ファイルをHash化した時に、わかるようにファイル名の先頭に目印の後にHash値。

画像が重複していないか確認するアルゴリズムは、Exif情報に含まれる日付をまず見てからその日時で同じのを探す。

## Usese Crates
- [rawloader](https://crates.io/crates/rawloader)
- [bayer](https://crates.io/crates/bayer)

## Document

### Image
[new]
make Image(Self) from file's path(String).

[hash]
rename image file to hash num from Image(Self).

[add]
add Images() from Image(Self).

[rm]
remove Images() from Image(Self).

## License
