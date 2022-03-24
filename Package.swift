// swift-tools-version:5.3
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let excludes: [String] = [
    "../../../sapling",
    "../../../sapling-android",
    "../../../../scripts",
    "../../../../Cargo.toml",
    "../../../../jitpack.yml",
    "../../../../lerna.json",
    "../../../../package.json",
    "../../../../package-lock.json"
]

let package = Package(
    name: "AirGapSapling",
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "Sapling",
            targets: ["Sapling"]),
        .library(
            name: "SaplingFFI",
            targets: ["SaplingFFI"]),
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        // .package(url: /* package url */, from: "1.0.0"),
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "Sapling",
            dependencies: ["SaplingFFI"],
            path: "packages/sapling-ios/Sources/Sapling",
            exclude: excludes),
        .binaryTarget(
            name: "SaplingFFI",
            path: "packages/sapling-ios/SaplingFFI.xcframework"),
        .testTarget(
            name: "SaplingTests",
            dependencies: [],
            path: "packages/sapling-ios/Tests/SaplingTests",
            exclude: excludes)
    ]
)
