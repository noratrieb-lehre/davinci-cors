FROM rust as build
WORKDIR /app
COPY . ./
WORKDIR /app/server
RUN cargo build --release --quiet

FROM ubuntu
COPY --from=build /app/target/release/server /usr/local/bin/server
CMD ["/usr/local/bin/server"]