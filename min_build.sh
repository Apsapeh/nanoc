#RUSTFLAGS="-C opt-level=z -C lto -C codegen-units=1 -C strip=symbols -C panic=abort -C debuginfo=none -C debug-assertions=off -C overflow-checks=false -C incremental=false -Z location-detail=none" cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort\
    #--target aarch64-apple-darwin --release

RUSTFLAGS="-Z location-detail=none" cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort\
     --target aarch64-apple-darwin --release --all --verbose

ls -lh target/aarch64-apple-darwin/release