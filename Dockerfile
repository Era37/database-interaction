FROM rust:1.66
WORKDIR /app
COPY . /app
RUN cargo build --release
CMD ["cargo", "run", "--release"]
