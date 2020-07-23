cargo build --release
mkdir -p build
cp target/release/clock build
cd build
tar cf clock.tar.gz ./clock
rm clock
