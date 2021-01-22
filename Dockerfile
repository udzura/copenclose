FROM ubuntu:groovy
RUN apt -q -y update && \
    apt -q -y install libelf-dev libgcc-s1 clang \
              llvm libbpf-dev curl make
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/install.sh && \
    sh /tmp/install.sh -y
RUN /root/.cargo/bin/rustup toolchain install 1.49.0
RUN mkdir /build
ADD . /build
RUN . /root/.cargo/env && \
      cd /build && \
      cargo install libbpf-cargo && \
      cargo libbpf build && \
      cargo libbpf gen && \
      cargo build

FROM ubuntu:groovy
RUN apt -q -y update && apt -q -y install libelf1 libgcc-s1 && apt -q -y clean
COPY --from=0 /build/target/debug/copenclose /usr/sbin
CMD ["/usr/sbin/copenclose"]
