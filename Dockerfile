FROM rust

WORKDIR /usr/src/mudlarking-server
COPY . .

RUN cargo install --path .

ENV PORT=5555

EXPOSE 5555

CMD ["mudlarking-server"]