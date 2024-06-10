FROM rust:1.78.0-bullseye as builder
WORKDIR /usr/src/tfg-backend
COPY . .
RUN rustup default nightly && cargo install --path .

FROM debian:bullseye-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/tfg-backend/target/release/rocket /rocket
EXPOSE 8000
CMD ["/rocket"]

