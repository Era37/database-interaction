FROM rust:1.66
WORKDIR /app
COPY . /app
CMD ["cargo", "run", "--release"]
