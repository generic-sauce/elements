FROM ubuntu:groovy-20200812

ADD src /root/src
ADD res /root/res
ADD Cargo.toml /root/Cargo.toml
ADD Cargo.lock /root/Cargo.lock
ADD rust-toolchain /root/rust-toolchain

WORKDIR /root/

RUN apt update && apt install -y curl gcc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /root/rustup.sh \
	&& sh /root/rustup.sh -y --default-toolchain=nightly-2018-08-01 \
	&& rm /root/rustup.sh \
	&& /root/.cargo/bin/cargo build --release --no-default-features \
	&& rm -r /root/.rustup \
	&& rm -r /root/.cargo

CMD /root/target/release/elements2
