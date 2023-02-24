load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_doc", "rust_library", "rust_test")

rust_binary(
    name = "mercator",
    srcs = ["main.rs"],
    deps = [":mercator_lib"],
)

rust_library(
    name = "mercator_lib",
    srcs = [
        "cards.rs",
        "game.rs",
        "lib.rs",
        "rand.rs",
    ],
)

rust_test(
    name = "mercator_test",
    crate = ":mercator_lib",
)

rust_doc(
    name = "mercator_doc",
    crate = ":mercator_lib",
    visibility = ["//:__pkg__"],
)
