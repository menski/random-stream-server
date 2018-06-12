FROM alpine:3.7

EXPOSE 9001
ENTRYPOINT [ "/bin/random-stream-server" ]
CMD [ "0.0.0.0:9001" ]

ADD https://github.com/menski/random-stream-server/releases/download/0.1.0/random-stream-server-linux /bin/random-stream-server
RUN chmod +x /bin/random-stream-server
