# Use multi-stage build to reduce image size
FROM quay.io/stegos/rust:nightly-2019-11-25 AS source
# FROM rustlang/rust:nightly-stretch-slim AS source

RUN apt-get -y install libpq-dev
LABEL maintainer="Vladimir Motylenko <me@vldm.cc>"

#### START CARGO HACK THAT PREVENT REBUILD
WORKDIR /usr/src/
# create a new empty  project
RUN USER=root cargo new --lib explorer_backend
WORKDIR /usr/src/explorer_backend

# add fetcher bin
RUN mkdir -p src/bin/fetcher
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/bin/fetcher/main.rs

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release  --features="fetcher" 
RUN rm src/ -r

#### END CARGO HACK
# build for release

RUN rm ./target/release/deps/explorer_backend*
RUN rm ./target/release/deps/libexplorer_backend*

COPY . /usr/src/explorer_backend


RUN cargo install --locked --path . --features="fetcher" --root /usr/local
RUN /usr/bin/strip /usr/local/bin/api
RUN /usr/bin/strip /usr/local/bin/fetcher

FROM quay.io/stegos/rust:nightly-2019-11-25
LABEL maintainer="Vladimir Motylenko <me@vldm.cc>"
# copy stegosd dependencies
COPY --from=source /usr/local/bin/api /usr/local/bin/api
COPY --from=source /usr/local/bin/fetcher /usr/local/bin/fetcher
COPY --from=source /usr/lib/x86_64-linux-gnu/libstdc++.so.6 /usr/lib/x86_64-linux-gnu/libstdc++.so.6
COPY --from=source /usr/lib/x86_64-linux-gnu/libgmp.so.10 /usr/lib/x86_64-linux-gnu/libgmp.so.10
COPY --from=source /lib/x86_64-linux-gnu/libdl.so.2 /lib/x86_64-linux-gnu/libdl.so.2
COPY --from=source /lib/x86_64-linux-gnu/librt.so.1 /lib/x86_64-linux-gnu/librt.so.1
COPY --from=source /lib/x86_64-linux-gnu/libpthread.so.0 /lib/x86_64-linux-gnu/libpthread.so.0
COPY --from=source /lib/x86_64-linux-gnu/libgcc_s.so.1 /lib/x86_64-linux-gnu/libgcc_s.so.1
COPY --from=source /lib/x86_64-linux-gnu/libc.so.6 /lib/x86_64-linux-gnu/libc.so.6
COPY --from=source /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2
COPY --from=source /lib/x86_64-linux-gnu/libm.so.6 /lib/x86_64-linux-gnu/libm.so.6

# copy our dependencies (to reproduce use ldd and multicursor) Most of dependencies are actix, and can be removed using other features.
COPY --from=source /lib/x86_64-linux-gnu/libcrypto.so.1.0.0 /lib/x86_64-linux-gnu/libcrypto.so.1.0.0
COPY --from=source /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2
COPY --from=source /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2 /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2
COPY --from=source /usr/lib/x86_64-linux-gnu/libkrb5.so.3 /usr/lib/x86_64-linux-gnu/libkrb5.so.3
COPY --from=source /usr/lib/x86_64-linux-gnu/libk5crypto.so.3 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3
COPY --from=source /lib/x86_64-linux-gnu/libcom_err.so.2 /lib/x86_64-linux-gnu/libcom_err.so.2
COPY --from=source /usr/lib/x86_64-linux-gnu/libkrb5support.so.0 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0
COPY --from=source /usr/lib/x86_64-linux-gnu/liblber-2.4.so.2 /usr/lib/x86_64-linux-gnu/liblber-2.4.so.2
COPY --from=source /lib/x86_64-linux-gnu/libresolv.so.2 /lib/x86_64-linux-gnu/libresolv.so.2
COPY --from=source /usr/lib/x86_64-linux-gnu/libsasl2.so.2 /usr/lib/x86_64-linux-gnu/libsasl2.so.2
COPY --from=source /usr/lib/x86_64-linux-gnu/libgssapi.so.3 /usr/lib/x86_64-linux-gnu/libgssapi.so.3
COPY --from=source /usr/lib/x86_64-linux-gnu/libgnutls.so.30 /usr/lib/x86_64-linux-gnu/libgnutls.so.30
COPY --from=source /lib/x86_64-linux-gnu/libkeyutils.so.1 /lib/x86_64-linux-gnu/libkeyutils.so.1
COPY --from=source /usr/lib/x86_64-linux-gnu/libheimntlm.so.0 /usr/lib/x86_64-linux-gnu/libheimntlm.so.0
COPY --from=source /usr/lib/x86_64-linux-gnu/libkrb5.so.26 /usr/lib/x86_64-linux-gnu/libkrb5.so.26
COPY --from=source /usr/lib/x86_64-linux-gnu/libasn1.so.8 /usr/lib/x86_64-linux-gnu/libasn1.so.8
COPY --from=source /usr/lib/x86_64-linux-gnu/libhcrypto.so.4 /usr/lib/x86_64-linux-gnu/libhcrypto.so.4
COPY --from=source /usr/lib/x86_64-linux-gnu/libroken.so.18 /usr/lib/x86_64-linux-gnu/libroken.so.18
COPY --from=source /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1
COPY --from=source /usr/lib/x86_64-linux-gnu/libp11-kit.so.0 /usr/lib/x86_64-linux-gnu/libp11-kit.so.0
COPY --from=source /usr/lib/x86_64-linux-gnu/libidn.so.11 /usr/lib/x86_64-linux-gnu/libidn.so.11
COPY --from=source /usr/lib/x86_64-linux-gnu/libtasn1.so.6 /usr/lib/x86_64-linux-gnu/libtasn1.so.6
COPY --from=source /usr/lib/x86_64-linux-gnu/libnettle.so.6 /usr/lib/x86_64-linux-gnu/libnettle.so.6
COPY --from=source /usr/lib/x86_64-linux-gnu/libhogweed.so.4 /usr/lib/x86_64-linux-gnu/libhogweed.so.4
COPY --from=source /usr/lib/x86_64-linux-gnu/libgmp.so.10 /usr/lib/x86_64-linux-gnu/libgmp.so.10
COPY --from=source /usr/lib/x86_64-linux-gnu/libwind.so.0 /usr/lib/x86_64-linux-gnu/libwind.so.0
COPY --from=source /usr/lib/x86_64-linux-gnu/libheimbase.so.1 /usr/lib/x86_64-linux-gnu/libheimbase.so.1
COPY --from=source /usr/lib/x86_64-linux-gnu/libhx509.so.5 /usr/lib/x86_64-linux-gnu/libhx509.so.5
COPY --from=source /usr/lib/x86_64-linux-gnu/libsqlite3.so.0 /usr/lib/x86_64-linux-gnu/libsqlite3.so.0
COPY --from=source /lib/x86_64-linux-gnu/libcrypt.so.1 /lib/x86_64-linux-gnu/libcrypt.so.1
COPY --from=source /usr/lib/x86_64-linux-gnu/libffi.so.6 /usr/lib/x86_64-linux-gnu/libffi.so.6
COPY --from=source /lib/x86_64-linux-gnu/libssl.so.1.0.0 /lib/x86_64-linux-gnu/libssl.so.1.0.0

COPY --from=source /usr/lib/x86_64-linux-gnu/libpq.so.5 /usr/lib/x86_64-linux-gnu/libpq.so.5

COPY --from=source /bin/sh /bin/sh

EXPOSE 3000
