output: step.0
	@echo "step0_repl"

step.0: src/step0_repl.rs
	cargo build --release
	cp target/release/step0_repl .
#		cargo build

clean:
	rm -rf target/release/step*
	rm step*