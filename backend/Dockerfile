FROM rust:1.95

WORKDIR /app

COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
COPY src ./src

RUN cargo build --release --bin simple-chat-backend

EXPOSE 8000

CMD ["/app/target/release/simple-chat-backend"]