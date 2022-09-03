FROM rust as build
WORKDIR /app
COPY . ./
WORKDIR /app/bot
RUN cargo build --release

FROM ubuntu:latest
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=build /app/target/release/bot /usr/local/bin/bot
CMD ["bot"]