# spotification

## これは何
- spotifydなどで再生している曲のメタデータから曲名、歌手を抜き出してファイルに吐き出すもの
- OBSなどでファイルを指定して表示する機能と組み合わせると、操作する必要なく楽曲情報を共有できる
- Rust初心者の実装なので、イマイチよくわからず動いているところがあるので注意

## 使い方 (Systemd環境向け)

### 準備
- 生成したバイナリを適当なところに置く
  - ex: `~/bin/spotification`
- [ここ](https://github.com/Marco3jp/spotification/tree/master/examples/systemd) のファイルを `~/.config/systemd/user/` あたりに投げ込む
- コピーしたserviceファイルを開いて、バイナリのパスを調整する

### 実行
- `systemctl start --user spotifycation.service` で単発実行できる（テスト等で使える）
- `systemctl start --user spotifycation.timer` でタイマー実行できる
  - timerファイル内を調整して実行間隔を調整できる
  - `systemctl stop --user spotifycation.timer` で止められる
