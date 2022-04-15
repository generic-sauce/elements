FROM archlinux:base-20201213.0.11146

ADD rust-toolchain rust-toolchain

RUN pacman --noconfirm -Sy archlinux-keyring && pacman --noconfirm -Syu && pacman --noconfirm -S gcc pkgconf
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y -q --default-toolchain nightly
