FROM rust:1.72 as builder
WORKDIR /app
COPY . .
RUN cargo build --release


FROM ubuntu:22.04 as runner
COPY --from=builder /app/target/release/the-maze /usr/local/bin/the_maze
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["the_maze"]