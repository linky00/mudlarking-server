FROM rust

WORKDIR /usr/src/mudlarking-server
COPY . .

RUN cargo install --path .

ENV PORT=4000

EXPOSE 4000

CMD ["mudlarking-server"]