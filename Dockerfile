

FROM python:latest as py
WORKDIR /app
RUN pip install yt-dlp -U

FROM rust:latest as build

WORKDIR /app

COPY . /app

ENV CARGO_BUILD_RUSTFLAGS="-C target-feature=+crt-static" 
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM build as cache
FROM py

COPY --from=cache /app/target/x86_64-unknown-linux-gnu/release/deletos /app/


CMD [ "/app/deletos" ]