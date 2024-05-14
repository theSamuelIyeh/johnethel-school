# stage 1 - generate a recipe for file for dependencies
FROM rust:1.78-slim-buster as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# stage 2 - build our dependencies from recope.json
FROM rust:1.78-slim-buster as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
# RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev clang llvm build-essential
RUN cargo chef cook --release --recipe-path recipe.json

# stage 3 - build our executable
FROM rust:1.78-slim-buster as builder
COPY . /app
WORKDIR /app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
# COPY ./templates /app/templates
# RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev clang llvm build-essential
# RUN rustup component remove rustc && rustup component add rustc
RUN cargo build --release # --target=x86_64-unknown-linux-gnu

# stage 4 - final stage: runtime image
FROM gcr.io/distroless/cc-debian11
COPY --from=builder /app/target/release/johnethel-school /app/johnethel-school
WORKDIR /app
COPY ./static /app/static
# RUN ls .
CMD [ "./johnethel-school" ]
