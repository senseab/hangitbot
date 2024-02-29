FROM alpine as build

ENV RUSTFLAGS="-C target-feature=-crt-static"
WORKDIR /usr/src/saysthbot
COPY . .
RUN apk add --no-cache rustup openssl-dev build-base && \
    rustup-init -y --default-toolchain nightly-2024-02-04 && \
    source ${HOME}/.cargo/env && cargo build --release

FROM alpine

RUN apk add --no-cache ca-certificates openssl libgcc
ENV TGBOT_TOKEN="" DATABASE_URI="" WRAPPER=""
CMD ["-c", "${WRAPPER} ./hangitbot ${OPTIONS}"]
ENTRYPOINT [ "/bin/sh" ]

COPY --from=build /usr/src/saysthbot/target/release/hangitbot ./
