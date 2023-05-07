# buildifier: disable=no-effect
rust_binary(
    name = "main",
    srcs = glob(
        ["src/**/*.rs"],
    ),
    crate_root = "src/main.rs",
    deps = [
        "//third-party:serde",
        "//third-party:serde_json"
    ]
)
