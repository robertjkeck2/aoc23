.PHONY: r j

r:
	@cd rust && cargo run $(day) $(part)

j:
	@cd java && ./gradlew run --args="$(day) $(part)"