# ベースとなるイメージ
FROM rust:latest

# コンテナ内での作業ディレクトリ
WORKDIR /app

# ホットリロード用ツール "cargo-watch" をインストール
RUN cargo install cargo-watch

# ソースコードをコピー
COPY . .

# サーバーを起動
CMD ["cargo", "watch", "-x", "run"]
