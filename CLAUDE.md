# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

whisper.cpp is a high-performance C/C++ implementation of OpenAI's Whisper automatic speech recognition (ASR) model. It provides inference-only capabilities with no dependencies, supporting multiple hardware backends (CPU, CUDA, Metal, Vulkan, etc.).

## Build Commands

### Core Library (CMake)

```bash
# Standard build
cmake -B build
cmake --build build --config Release -j

# Build with CUDA support
cmake -B build -DGGML_CUDA=1
cmake --build build --config Release -j

# Build with Vulkan support
cmake -B build -DGGML_VULKAN=1
cmake --build build --config Release -j

# Build with OpenBLAS CPU acceleration
cmake -B build -DGGML_BLAS=1
cmake --build build --config Release -j

# Build with Core ML (Apple Silicon)
cmake -B build -DWHISPER_COREML=1
cmake --build build --config Release -j
```

### Running Tests

```bash
# Run CTest (requires models in models/ directory)
cd build && ctest

# Run specific test label
cd build && ctest -L tiny
```

### Quick Demo with Models

```bash
# Download model and run on samples
make base.en    # Downloads base.en model, builds, and transcribes samples/

# Download additional samples
make samples
```

### Main Executable

```bash
# Transcribe audio file
./build/bin/whisper-cli -m models/ggml-base.en.bin -f samples/jfk.wav

# With language specification
./build/bin/whisper-cli -m models/ggml-base.bin -l fr -f samples/jfk.wav
```

## Architecture

### Core Components

- **`src/whisper.cpp`** + **`include/whisper.h`**: Complete Whisper model implementation. The C-style API in `whisper.h` is the primary interface.
- **`ggml/`**: The GGML machine learning library submodule providing tensor operations and hardware backends:
  - `ggml/src/ggml.c`: Core tensor operations
  - `ggml/src/ggml-cpu/`: CPU backend
  - `ggml/src/ggml-cuda/`: NVIDIA GPU backend
  - `ggml/src/ggml-metal/`: Apple Metal backend
  - `ggml/src/ggml-vulkan/`: Vulkan GPU backend
  - Other backends: OpenCL, SYCL, HIP, CANN, etc.

### Key API Pattern

```c
// Basic usage pattern from whisper.h
whisper_context_params cparams = whisper_context_default_params();
struct whisper_context * ctx = whisper_init_from_file_with_params("/path/to/ggml-base.en.bin", cparams);

if (whisper_full(ctx, wparams, pcmf32.data(), pcmf32.size()) != 0) {
    // error handling
}

const int n_segments = whisper_full_n_segments(ctx);
for (int i = 0; i < n_segments; ++i) {
    const char * text = whisper_full_get_segment_text(ctx, i);
    // process text
}

whisper_free(ctx);
```

### Examples Structure

The `examples/` directory contains various applications demonstrating library usage:

- **`cli/`**: `whisper-cli` - main command-line transcription tool
- **`stream/`**: Real-time microphone transcription (requires SDL2)
- **`server/`**: HTTP server with OpenAI-compatible API
- **`bench/`**: Performance benchmarking tool
- **`command/`**: Voice command recognition
- **`talk-llama/`**: Voice assistant with LLaMA integration
- **`whisper.wasm/`**: WebAssembly build for browsers

Common shared code: `examples/common.cpp`, `examples/common.h`, `examples/common-sdl.cpp`

### Bindings

Language bindings in `bindings/` directory:
- **JavaScript**: `bindings/javascript/` - Emscripten/WebAssembly
- **Go**: `bindings/go/`
- **Java**: `bindings/java/`
- **Ruby**: `bindings/ruby/`

## Models

Models are stored in `models/` directory in custom `ggml` binary format.

```bash
# Download pre-converted model
bash ./models/download-ggml-model.sh base.en

# Available models: tiny, tiny.en, base, base.en, small, small.en,
# medium, medium.en, large-v1, large-v2, large-v3, large-v3-turbo
```

Quantized models (e.g., `large-v3-q5_0`) use less memory with minimal accuracy loss.

## Frontend-Tauri (Rust Desktop App)

The `frontend-tauri/` directory contains a Tauri-based desktop application in bootstrap phase.

### Prerequisites

- Rust toolchain (stable)
- Tauri system prerequisites: https://v2.tauri.app/start/prerequisites/

### Development Commands

From `frontend-tauri/`:

```bash
make fmt          # Format check
make lint         # Clippy with warnings as errors
make test         # Unit tests
make check        # All local quality checks (fmt + lint + test)
make smoke-real   # Optional real whisper smoke test (requires env vars)
make verify-release  # Release verification (check + optional smoke)
make dev          # Run app locally
```

### Frontend Workflow Constraints

- Use Conventional Commits for all commit messages
- Follow Semantic Versioning for releases
- Keep `changelog.md` updated for user-visible changes
- Always ask for approval before committing

### Frontend Architecture

- `src/main.rs`: Tauri app entry point with command registration
- `src/commands.rs`: Backend command shell for system operations
- `src/contracts.rs`: Typed request/response contracts
- `src/execution.rs`: Process execution service (whisper-cli runner)
- `src/tauri_commands.rs`: Tauri command wrappers
- `src/mvp_binding.rs`: WebView binding layer
- `src/ui_state.rs`: Workflow state management
- `src/config.rs`: Configuration persistence
- `src/errors.rs`: Error mapping

## Audio Requirements

- Input audio must be **16-bit WAV format at 16kHz**
- Convert other formats with ffmpeg:
  ```bash
  ffmpeg -i input.mp3 -ar 16000 -ac 1 -c:a pcm_s16le output.wav
  ```

## Voice Activity Detection (VAD)

VAD support filters non-speech segments before transcription:

```bash
# Download VAD model
./models/download-vad-model.sh silero-v6.2.0

# Use with transcription
./build/bin/whisper-cli --vad --vad-model models/ggml-silero-v6.2.0.bin \
    -m models/ggml-base.en.bin -f samples/jfk.wav
```
