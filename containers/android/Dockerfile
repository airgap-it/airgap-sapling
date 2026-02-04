FROM debian:bookworm-slim

### Prepare environment ###

# Install essential tools
RUN apt-get update -qq && \
    apt-get install -y -qq \
        wget \
        unzip \
        openjdk-17-jdk \
        python3 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

### Install Android SDK ###

ENV ANDROID_HOME=/usr/lib/android-sdk

ENV ANDROID_COMPILE_SDK=36
ENV ANDROID_CMDLINE_TOOLS=13114758

# Download command line tools
RUN wget -q https://dl.google.com/android/repository/commandlinetools-linux-${ANDROID_CMDLINE_TOOLS}_latest.zip -O android-commandlinetools.zip && \
    unzip -q android-commandlinetools.zip -d ${ANDROID_HOME} && \
    mv ${ANDROID_HOME}/cmdline-tools ${ANDROID_HOME}/latest && \
    mkdir ${ANDROID_HOME}/cmdline-tools && \
    mv ${ANDROID_HOME}/latest ${ANDROID_HOME}/cmdline-tools && \
    rm android-commandlinetools.zip

ENV PATH="$PATH:${ANDROID_HOME}/emulator:${ANDROID_HOME}/cmdline-tools/latest/bin"

# Accept licenses
RUN yes | sdkmanager --sdk_root="${ANDROID_HOME}" --licenses

### Prepare working directory ###

RUN mkdir /build
WORKDIR /build

# copy source
COPY . /build

# clean project
RUN ./packages/sapling-android/gradlew --project-dir ./packages/sapling-android clean

# build apk, exclude prod flavored unit tests
RUN ./packages/sapling-android/gradlew --project-dir ./packages/sapling-android assemble

# copy release aar
RUN cp /build/packages/sapling-android/app/build/outputs/aar/app-release.aar android-release-unsigned.aar
