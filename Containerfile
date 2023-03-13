FROM busybox:latest

# --build-arg PACKAGE_NAME=${package_name}
ARG PACKAGE_NAME="q-api-auth"

COPY ./target/x86_64-unknown-linux-musl/release/$PACKAGE_NAME /bin/$PACKAGE_NAME
COPY ./Rocket.toml /root

WORKDIR /root

CMD [ "q-api-auth" ]
