build-docs:  ## Build docs for Github Pages
	rm -rf ./docs
	cargo doc --no-deps
	echo "<meta http-equiv=\"refresh\" content=\"0; url=cipha\">" > target/doc/index.html
	cp -r target/doc ./docs

test:  ## Run unit tests
	cargo test

check:  ## Run cargo check with all features
	cargo check --workspace --all-targets --all-features