export PATH=$(bash -c "echo /tmp/rust/toolchains/*/bin"):"$PATH" # The * will become the most recent version if everything goes to plan.
echo Path is now: "$PATH"
cd rust_app
if $(cat ../run-only.bool) ; then
  f=`bash -c "echo ./target/release/*.d"`
  ${f%.*}
else
  cargo run --release
fi