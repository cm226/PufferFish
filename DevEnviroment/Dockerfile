ARG NASM_VERSION
ARG RUST_VERSION="latest"

FROM rust:${RUST_VERSION}

ARG NASM_VERSION

RUN apt-get update &&\
    apt-get -y install libsdl2-dev

RUN wget https://www.nasm.us/pub/nasm/releasebuilds/${NASM_VERSION}/nasm-${NASM_VERSION}.tar.bz2	&& \
    tar xfj ./nasm-${NASM_VERSION}.tar.bz2

RUN cd nasm-${NASM_VERSION}/ && \
    ./autogen.sh && \
    ./configure --prefix=/usr/local/ && \
    make && \
    make install