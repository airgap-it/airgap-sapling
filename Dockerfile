FROM bitriseio/android-ndk:latest

RUN mkdir /build
WORKDIR /build

# copy source
COPY . /build

# clean project
RUN /build/packages/sapling-android/gradlew --project-dir /build/packages/sapling-android clean

# build apk, exclude prod flavored unit tests
RUN /build/packages/sapling-android/gradlew --project-dir /build/packages/sapling-android assemble

# copy release aar
RUN cp /build/packages/sapling-android/build/outputs/aar/release.aar android-release-unsigned.aar

FROM node:15-slim

RUN apt-get update && apt-get install -yq git python build-essential curl

# install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

# install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# create build directory
RUN mkdir /build
WORKDIR /build

# copy sources
COPY ../.. /build

# install dependencies
RUN npm ci

RUN chmod +x ./scripts/npm/publish.sh
RUN chmod +x ./scripts/npm/publish-beta-only.sh

RUN git update-index --assume-unchanged ./scripts/npm/publish.sh
RUN git update-index --assume-unchanged ./scripts/npm/publish-beta-only.sh

# set to production
RUN export NODE_ENV=production

# build
RUN npm run build:prod
