# Bruh i don't know what i'm doing
# this dude saved me https://github.com/cross-rs/cross/issues/260#issuecomment-1132237489

FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:main
RUN apt-get update && apt-get install -y wget
RUN mkdir -m777 /opt/rust /opt/cargo
ENV RUSTUP_HOME=/opt/rust CARGO_HOME=/opt/cargo PATH=/opt/cargo/bin:$PATH
RUN wget --https-only --secure-protocol=TLSv1_2 -O- https://sh.rustup.rs | sh /dev/stdin -y
RUN rustup target add aarch64-unknown-linux-gnu
RUN printf '#!/bin/sh\nexport CARGO_HOME=/opt/cargo\nexec /bin/sh "$@"\n' >/usr/local/bin/sh
RUN chmod +x /usr/local/bin/sh