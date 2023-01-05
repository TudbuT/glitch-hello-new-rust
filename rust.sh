export PATH=$(bash -c "echo /tmp/rust/toolchains/*/bin"):"$PATH" # The * will become the most recent version if everything goes to plan.
echo Path is now: "$PATH"
cd rust_app
cargo run --release