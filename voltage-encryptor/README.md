# Voltage Encryptor

WIP

Aiming to evaluate the use of:
- `async`/`await` backed by `tokio::Runtime`, sending async computed results across `CxxQtThread`.
- `JsRuntime` from `deno_core` to perform voltage encryption via pure JS scripts

## Notes

If the application is built by C++ linking to Rust lib, the Rust crate-type needs to be `staticlib`.
There are several implications:
- Rust side must not have a `main.rs` entrypoint. If a `main.rs` is defined with attempts to load `QQmlApplicationEngine` and run `QGuiApplication`, the Qml qrc loading step will fail due to error finding qrc resource. This is likely a result of static linking.
- By extension, the application is either built from the C++ (`main.cpp`) or Rust (`main.rs`) side, but not both.
