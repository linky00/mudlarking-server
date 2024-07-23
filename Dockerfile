FROM rust

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

ENV PORT=4000

EXPOSE 4000

CMD ["app"]