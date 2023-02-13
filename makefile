build: 
	cargo build --release
install:
	chmod +x target/release/norg-to-markdown
	cp target/release/norg-to-markdown /usr/bin
clean:
	rm -rf target
