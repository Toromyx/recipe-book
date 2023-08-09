FROM rust:1.70-bookworm AS base

# cargo utilities
RUN cargo install cargo-watch
RUN cargo install cargo-outdated

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
