FROM rust:bullseye as builder

WORKDIR /usr/src/netop

COPY . .

RUN apt-get update && apt-get install -y libpcap-dev build-essential
# RUN apk update && apk add alpine-sdk libpcap libpcap-dev

RUN cargo install --path .


FROM debian:bullseye-slim

# ENV GLIBC_REPO=https://github.com/sgerrand/alpine-pkg-glibc
# ENV GLIBC_VERSION=2.30-r0

RUN apt-get update && apt-get install -y libpcap-dev && rm -rf /var/lib/apt/lists/*
# RUN apk update && apk add libpcap-dev

COPY --from=builder /usr/local/cargo/bin/netop /usr/local/bin/netop

# RUN set -ex && \
#     apk --update add libstdc++ curl ca-certificates libpcap libpcap-dev && \
#     for pkg in glibc-${GLIBC_VERSION} glibc-bin-${GLIBC_VERSION}; \
#     do curl -sSL ${GLIBC_REPO}/releases/download/${GLIBC_VERSION}/${pkg}.apk -o /tmp/${pkg}.apk; done && \
#     apk add --allow-untrusted /tmp/*.apk && \
#     rm -v /tmp/*.apk && \
#     /usr/glibc-compat/sbin/ldconfig /lib /usr/glibc-compat/lib

ENTRYPOINT [ "netop" ]