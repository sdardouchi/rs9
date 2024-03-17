FROM rust:latest
WORKDIR /app
COPY . /app
RUN cargo install --path=/app 
RUN cargo clean
CMD [ "rs9" ]
