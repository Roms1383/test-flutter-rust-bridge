example:
	cd rust && cargo run --example kira
bindings:
	RUST_BACKTRACE=full flutter_rust_bridge_codegen --rust-input rust/src/api.rs --dart-output dart/lib/bridge_generated.dart
