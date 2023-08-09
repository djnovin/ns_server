# ---------------------------------------------------
# 1 - Build Stage
#
# Use official rust image to for application build
# ---------------------------------------------------

FROM rust:1.67 as builder

# Setup working directory
WORKDIR /usr/src/ns-api
COPY . .
COPY .env .env

# Install dependencies
RUN cargo install --path .

# ---------------------------------------------------
# 2 - Deploy Stage
#
# Use a distroless image for minimal container size
# - Copy application files into the image
# ---------------------------------------------------

FROM gcr.io/distroless/cc-debian10

# Set the architecture arguement (arm64, i.e. aaarch64 as default)
# For amd64, i.e. x86_64, you can append a flag when invoking the build `... --build-arg "ARCH=x86_64"`
ARG ARCH=aarch64

# Application files
COPY --from=builder /usr/local/cargo/bin/ns-api /usr/local/bin/ns-api
COPY --from=builder /usr/src/ns-api/.env /usr/local/bin/.env

CMD ["ns-api"]