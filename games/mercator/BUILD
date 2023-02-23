load("@rules_rust//rust:defs.bzl", "rust_library", "rust_binary", "rust_test", "rust_doc")

rust_binary(
    name = "mercator",
    srcs = [ "main.rs" ],
    deps = [ ":mercator_lib" ]
)

rust_library(
    name = "mercator_lib",
    srcs = [
        "game.rs",
        "lib.rs",
        "rand.rs",
    ],
)

rust_test(
    name = "mercator_test",
    crate = ":mercator_lib",
)

# TODO Figure out how to spin up a doc server? This documentation target is not
# super useful.
rust_doc(
    name = "mercator_doc",
    crate = ":mercator_lib",
    visibility = ["//:__pkg__"],
)
