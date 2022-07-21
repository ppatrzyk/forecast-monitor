FROM rust:1.62.1-bullseye as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path . --root /usr/local/bin/myapp

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/bin/myapp /myapp
COPY config.yaml /config.yaml
CMD ["/myapp/bin/forecast-monitor"]
