FROM rust:1-bookworm AS base

# rust setup
RUN cargo install cargo-binstall
RUN rustup component add rust-analyzer
RUN rustup component add clippy
RUN rustup toolchain install nightly
RUN rustup component add rustfmt --toolchain nightly

# cargo utilities
RUN cargo binstall --no-confirm cargo-watch
RUN cargo binstall --no-confirm cargo-outdated
RUN cargo binstall --no-confirm cargo-audit

# node
RUN apt-get update && apt-get install -y \
nodejs \
npm

# tauri build dependencies
RUN apt-get update && apt-get install -y \
libwebkit2gtk-4.0-dev \
build-essential \
curl \
wget \
file \
libssl-dev \
libgtk-3-dev \
libayatana-appindicator3-dev \
librsvg2-dev

# tesseract
RUN apt-get update && apt-get install -y \
clang
RUN apt-get update && apt-get install -y \
libleptonica-dev \
libtesseract-dev

# test dependencies
RUN apt-get update && apt-get install -y \
webkit2gtk-driver \
xvfb
RUN cargo binstall --no-confirm tauri-driver
