# 1. Intro

This is just a micro-service to that generates messages that are proxied to sms/email broker service. It uses the powerful [Actix Web](https://actix.rs/) framework.

# 2. How to run?

Easiest way to spin up a working sample of this repo is via Docker Compose

## 2.1 Docker Compose

To spin up the entire stack on local, just run the following command (depending on your target architecture).

1. arm64 / aarch64, e.g. M1 Mac,

```bash
# It is supported by default. You can simply run
docker-compose up

# You may also specify it explicitly
docker-compose build --build-arg "ARCH=aarch64"
docker-compose up
```

2. amd84 / x86_64, e.g. Intel chips,

```bash
# You need to specify the ARCH argument accordingly
docker-compose build --build-arg "ARCH=x86_64"
docker-compose up
```

You need to have [Docker](https://www.docker.com/products/docker-desktop/) installed on your machine.

The components included in the compose file are

- API (Actix Web)

## 2.2 `.env` file

You can just make a copy from `.env.example` (the sample env file) and rename it as `.env`.

## 2.3 For Local Development

To develop the Actix Web application itself, you need to first stop the `ns-api` service if you did spun it up using docker-compose in the previous steps.

After that, you can start the application via the following command

```
cargo run
```

It works just like any other Rust applications that use [Cargo](https://doc.rust-lang.org/cargo/).

# 3. Things to do

| Items                                                                                     | Status               |
| ----------------------------------------------------------------------------------------- | -------------------- |
| Actix Routes                                                                              | :white_check_mark:   |
| Unit Tests                                                                                | :white_large_square: |
| Error Handling                                                                            | :white_large_square: |
| Containerization                                                                          | :white_check_mark:   |
| OpenAPI - with [utoipa](https://github.com/juhaku/utoipa)                                 | :white_large_square: |
| Auth                                                                                      | :white_large_square: |
| CI/CD                                                                                     | :white_large_square: |
| ?                                                                                         | :white_large_square: |
