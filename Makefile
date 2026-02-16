all:
	cargo build -j$$(nproc) --release
	cp target/release/snow .
