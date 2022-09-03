FROM rust as build
WORKDIR /app
COPY . ./
WORKDIR /app/server
RUN cargo build --release


FROM ubuntu:latest
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=build /app/target/release/server /usr/local/bin/server
ENV RUST_LOG=info
CMD ["server"]