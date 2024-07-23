FROM rust

WORKDIR /app
COPY . .

RUN cargo install --path .

ENV PORT=5555

EXPOSE 5555

CMD ["mudlarking-server"]