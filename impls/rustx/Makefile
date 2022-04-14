output: step.0
	@echo "step0"

step.0: src/step0.rs
	cargo build --release
	cp target/release/step0 .
#		cargo build

clean:
	rm -rf target/release/step*
	rm step*