FROM rust:1.70-slim AS builder
WORKDIR /root

# Create a new empty project
COPY Cargo.toml Cargo.lock ./

# Build dependencies only
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Build the actual project
COPY . .
RUN touch src/main.rs && \
  cargo build --release

FROM ubuntu
EXPOSE 53/udp
WORKDIR /root
COPY --from=builder ./root/target/release/updns .
ENV LOG=info,warn,error
CMD ["./updns"]
