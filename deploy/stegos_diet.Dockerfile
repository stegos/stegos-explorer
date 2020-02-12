# This docker file is supposed to maximum reduce size.
# use the same tag that commit in our dependencies.

FROM stegosd:79bd4263 AS source 
FROM ubuntu:xenial AS striper
RUN apt-get update
RUN apt-get install binutils
COPY --from=source /usr/local/bin/stegos /usr/local/bin/stegos
COPY --from=source /usr/local/bin/stegosd /usr/local/bin/stegosd
RUN /usr/bin/strip /usr/local/bin/stegos
RUN /usr/bin/strip /usr/local/bin/stegosd

FROM scratch

COPY --from=striper /usr/local/bin/stegos /usr/local/bin/stegos
COPY --from=striper /usr/local/bin/stegosd /usr/local/bin/stegosd
COPY --from=source /usr/lib/x86_64-linux-gnu/libstdc++.so.6 /usr/lib/x86_64-linux-gnu/libstdc++.so.6
COPY --from=source /usr/lib/x86_64-linux-gnu/libgmp.so.10 /usr/lib/x86_64-linux-gnu/libgmp.so.10
COPY --from=source /lib/x86_64-linux-gnu/libdl.so.2 /lib/x86_64-linux-gnu/libdl.so.2
COPY --from=source /lib/x86_64-linux-gnu/librt.so.1 /lib/x86_64-linux-gnu/librt.so.1
COPY --from=source /lib/x86_64-linux-gnu/libpthread.so.0 /lib/x86_64-linux-gnu/libpthread.so.0
COPY --from=source /lib/x86_64-linux-gnu/libgcc_s.so.1 /lib/x86_64-linux-gnu/libgcc_s.so.1
COPY --from=source /lib/x86_64-linux-gnu/libc.so.6 /lib/x86_64-linux-gnu/libc.so.6
COPY --from=source /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2
COPY --from=source /lib/x86_64-linux-gnu/libm.so.6 /lib/x86_64-linux-gnu/libm.so.6
COPY --from=striper /bin/sh /bin/sh

WORKDIR /data
ENV STEGOS_DATA_DIR /data

EXPOSE 3145 9090
ENTRYPOINT [ "/usr/local/bin/stegosd" ]
