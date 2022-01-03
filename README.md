# example using flutter rust bridge and kira

PoC at playing sounds from Dart in Rust

#### usage

binary is just here to show that kira actually correctly play sound:
```sh
cargo run
```

but ffi build is done with this command:
```sh
RUST_BACKTRACE=full flutter_rust_bridge_codegen --rust-input src/api.rs --dart-output src/bridge_generated.dart
```

#### sounds credits

* splash screen: https://audiojungle.net/item/hitech-cybernetic-device/6815040
* home screen: https://audiojungle.net/item/cinematic-whoosh-impact-sound/238165