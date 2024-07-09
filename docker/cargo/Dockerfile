FROM rust:1.79-alpine3.20

# 必要なビルドツールをインストール
RUN apk add --no-cache \
    build-base \
    musl-dev \
    gcc \
    clang

# 必要なコンポーネントをインストール
RUN rustup update \
    && rustup component add rustfmt clippy
