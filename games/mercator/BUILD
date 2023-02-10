load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
    name = "mercator",
    srcs = [
        "lib.rs",
    ]
)

rust_test(
    name = "mercator_test",
    crate = ":mercator",
)
