FROM busybox:latest

# --build-arg PACKAGE_NAME=${package_name}
ARG PACKAGE_NAME

COPY ./target/x86_64-unknown-linux-musl/release/$PACKAGE_NAME /bin/$PACKAGE_NAME
COPY ./Rocket.toml /root

WORKDIR /root

CMD [ "$PACKAGE_NAME" ]

