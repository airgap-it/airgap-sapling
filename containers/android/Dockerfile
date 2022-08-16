FROM androidsdk/android-31

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
