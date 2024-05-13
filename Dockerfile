FROM rust:latest

LABEL authors="Andrew Kozmin"

WORKDIR /usr/src/the_game/

ADD Cargo.toml ./
ADD Cargo.lock ./

CMD ["cargo", "run"]
