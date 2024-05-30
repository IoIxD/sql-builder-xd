cd native
cargo build 
cp target/debug/libnative.a .
cd ..
go run .
