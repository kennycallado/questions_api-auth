#!/bin/bash

# This script compiles the binary using the rust-musl-builder docker image,
# and build the docker images for multiples platforms.
#
# In order to make the images to work, you need to install
# qemu-user-static in the host machine, because the binary
# is compiled for a different architecture.
#
# Introduce the option to build the stable version. To do so, run:
# ./build.sh stable
set -e

platforms=("linux/amd64" "linux/arm64" "linux/arm/v7" "linux/arm/v6")

# Get the package name and version from Cargo.toml
package_name=$(cat Cargo.toml | grep 'name' | awk '{print $3}' | tr -d '"')
version=$(cat Cargo.toml | grep 'version' | head -1 | awk '{print $3}' | tr -d '"')

# Remove Cargo.lock
rm -f Cargo.lock

# Permissions for target folder
mkdir -p target
chmod -R o+w target

# Build the binary
docker run --rm -it -v "$(pwd)":/home/rust/src nasqueron/rust-musl-builder cargo build --release

for platform in ${platforms[@]}; do
  echo "Building docker image for: $platform."

  # get the tag
  tag=$(echo "${platform//\//_}" | tr -d 'linux_' | xargs -I {} echo {})

  # build the image
  docker build --no-cache --pull \
    --platform ${platform} \
    -t kennycallado/${package_name}:${version}-${tag} \
    --build-arg PACKAGE_NAME=${package_name} \
    -f ./Containerfile .
done

# push the images
docker push -a kennycallado/${package_name}

# create the manifest
docker manifest create kennycallado/${package_name}:${version} \
  --amend kennycallado/${package_name}:${version}-amd64 \
  --amend kennycallado/${package_name}:${version}-arm64 \
  --amend kennycallado/${package_name}:${version}-armv7 \
  --amend kennycallado/${package_name}:${version}-armv6

# manifest for latest version
docker manifest create kennycallado/${package_name}:latest \
  --amend kennycallado/${package_name}:${version}-amd64 \
  --amend kennycallado/${package_name}:${version}-arm64 \
  --amend kennycallado/${package_name}:${version}-armv7 \
  --amend kennycallado/${package_name}:${version}-armv6

# manifest for stable version
if [ "$1" == "stable" ]; then
docker manifest create kennycallado/${package_name}:stable \
  --amend kennycallado/${package_name}:${version}-amd64 \
  --amend kennycallado/${package_name}:${version}-arm64 \
  --amend kennycallado/${package_name}:${version}-armv7 \
  --amend kennycallado/${package_name}:${version}-armv6
fi

# push the manifests
docker manifest push --purge kennycallado/${package_name}:${version}
docker manifest push --purge kennycallado/${package_name}:latest

# tag the latest version # I think it has no sense
# docker tag kennycallado/${package_name}:${version} kennycallado/${package_name}:latest
# docker push kennycallado/${package_name}:latest

# remove the images
docker rmi kennycallado/${package_name}:${version}-amd64
docker rmi kennycallado/${package_name}:${version}-arm64
docker rmi kennycallado/${package_name}:${version}-armv7
docker rmi kennycallado/${package_name}:${version}-armv6

# remove the manifest
docker system prune -f
