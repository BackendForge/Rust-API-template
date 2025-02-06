# Use Rust official image as the base
FROM rust:latest AS builder
WORKDIR /app
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release
COPY certs /certs
COPY keys /keys
RUN chown -R nobody:nogroup /certs /keys
RUN chmod -R 600 /keys
RUN chmod -R 644 /certs

FROM alpine:latest AS upx
RUN apk add --no-cache libc6-compat cmake make gcc g++ musl-dev busybox-extras git
ARG REPO_BRANCH=v4.2.4
RUN git clone --depth=1 --branch ${REPO_BRANCH} https://github.com/upx/upx.git /usr/upx
WORKDIR /usr/upx
RUN git submodule update --init --depth=1
RUN make

FROM upx AS compress
WORKDIR /usr/upx
COPY --from=builder /app/target/release/api /usr/api
RUN ./build/release/upx --lzma -o /usr/api-min-sized /usr/api

# Stage for the API application
FROM gcr.io/distroless/cc:latest AS main
# COPY --from=builder /app/target/release/api /usr/api
COPY --from=compress /usr/api-min-sized /usr/api
COPY --from=builder /certs /certs
COPY --from=builder /keys /keys
EXPOSE ${HTTP_PORT}
EXPOSE ${HTTPS_PORT}
EXPOSE ${METRICS_PORT}
USER nobody
ENTRYPOINT ["/usr/api"]