FROM debian:latest

RUN apt update
RUN apt install -y curl build-essential procps
RUN curl https://sh.rustup.rs -sSf |sh -s -- -y
ENV PATH $PATH:/root/.cargo/bin

WORKDIR /home/
COPY . /home/
RUN cargo build --release

EXPOSE 3000

CMD ["./target/release/iron"]
