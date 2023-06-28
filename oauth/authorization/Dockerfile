FROM ubuntu:latest as build

RUN apt-get -qq update
RUN apt-get install -y -q build-essential curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN apt-get install libglib2.0-0 libglib2.0-data libicu70 libxml2 shared-mime-info xdg-user-dirs pkg-config
RUN apt-get install libssl-dev
ENV PATH="/root/.cargo/bin:${PATH}"

RUN User=root cargo new --bin authorization 
WORKDIR ./build
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release
RUN rm src/*.rs

ADD ./src ./src
RUN rm ./target/release/deps/authorization*
RUN cargo build --release

FROM ubuntu:latest

WORKDIR /usr/bin/app
COPY --from=build ./build/target/release/authorization.

EXPOSE 8080

ENTRYPOINT ["/usr/bin/app/authorization"]