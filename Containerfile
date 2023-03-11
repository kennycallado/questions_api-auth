FROM busybox:latest

COPY ./target/x86_64-unknown-linux-musl/release/<package-name> /bin/<package-name>
COPY ./Rocket.toml /root

WORKDIR /root

CMD [ "<package-name>" ]

