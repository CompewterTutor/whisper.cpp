.PHONY: fmt lint test check smoke-real verify-release dev web-install web-build web-dev

fmt:
	cargo fmt --check

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

check: fmt lint test

smoke-real:
	@if [ -z "$$WHISPER_CLI_PATH" ] || [ -z "$$WHISPER_MODEL_PATH" ] || [ -z "$$WHISPER_AUDIO_PATH" ]; then \
		echo "Skipping smoke-real: set WHISPER_CLI_PATH, WHISPER_MODEL_PATH, and WHISPER_AUDIO_PATH"; \
		exit 0; \
	fi
	cargo test --features real-whisper-smoke --lib real_whisper_cli_smoke_test -- --ignored

verify-release: check smoke-real

# Frontend (SvelteKit) commands
web-install:
	npm install

web-build:
	npm run build

web-dev:
	npm run dev

# Tauri development (starts both frontend dev server and Tauri)
dev:
	cargo tauri dev
