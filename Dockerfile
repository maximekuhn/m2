# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.91.0
ARG APP_NAME=m2

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev git curl

# sqlx requires migration files to be available at build time in order to
# embed them into the binary.
# https://docs.rs/sqlx/0.8.6/sqlx/macro.migrate.html
COPY ./migrations ./migrations

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release && \
cp ./target/release/$APP_NAME /bin/server

################################################################################
# Create a stage for running the application.

FROM alpine:3.18 AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/home/m2" \
    --shell "/sbin/nologin" \
    --uid "${UID}" \
    appuser
USER appuser

WORKDIR /home/m2

COPY --from=build /bin/server /bin/

EXPOSE 55432

CMD ["/bin/server", "config.yml"]
