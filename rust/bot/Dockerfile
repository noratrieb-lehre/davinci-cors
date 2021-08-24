FROM rust as build
WORKDIR /app
COPY . ./
WORKDIR /app/bot
RUN cargo build --release --quiet

FROM ubuntu
COPY --from=build /app/target/release/bot /usr/local/bin/bot
CMD ["/user/local/bin/bot"]