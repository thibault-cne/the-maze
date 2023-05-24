FROM rust:1.69.0 as builder
WORKDIR /app
COPY . .
RUN rustup default nightly
RUN cargo build --release


FROM debian:buster-slim as runner
COPY --from=builder /app/target/release/the-maze /usr/local/bin/the_maze
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["the_maze"]