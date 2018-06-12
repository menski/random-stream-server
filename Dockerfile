FROM alpine:3.7

EXPOSE 9001
ENTRYPOINT [ "/bin/random-stream-server" ]
CMD [ "0.0.0.0:9001" ]

COPY target/x86_64-unknown-linux-musl/release/random-stream-server /bin/random-stream-server
