FROM rust:1.69.0 AS build-stage

RUN mkdir -p /cpp-backend
WORKDIR /cpp-backend

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=build-stage /cpp-backend/target/release/cpp-backend /

CMD ["/cpp-backend"]
