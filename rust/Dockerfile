# Server dockerfile

FROM rust:1.53 as build

WORKDIR /usr/src/davinci

COPY dto dto
COPY server server

RUN apt install -y libpq-dev

WORKDIR server
RUN cargo build --release

FROM scratch
COPY --from=build /usr/src/davinci/target/release/server /usr/local/bin/cors-server

CMD ["cors-server"]