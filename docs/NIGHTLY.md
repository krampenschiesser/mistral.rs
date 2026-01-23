# Nightly build

For the bleeding edge release there is a nightly job that does the following:

1. Checks if a small model can run inference
2. Builds the runnables for the current environment
3. publishes them as nightly github release overwriting the old one
    1. Artifacts are kept for a longer time, so you can still download older builds


## Nightly Build Targets

| Target Name             | Architecture  | OS             | CUDA Version       | CPU Flags                                                                 |
|-------------------------|---------------|----------------|--------------------|---------------------------------------------------------------------------|
| macos                   | ARM64         | macOS latest   | -                  | (none)                                                                    |
| windows-x86_64          | x86_64        | Windows latest | -                  | +avx2,+avx,+sse4.1,+f16c                                                  |
| windows-x86_64-cuda     | x86_64        | Windows latest | 13.1.0             | +avx2,+avx,+sse4.1,+f16c                                                  |
| linux-x86_64            | x86_64        | Ubuntu 24.04   | -                  | +avx2,+avx,+sse4.1,+f16c                                                  |
| linux-x86_64-avxvnni    | x86_64        | Ubuntu 24.04   | -                  | +avx2,+avx,+avxvnni,+f16c,+sse4.2                                         |
| linux-x86_64-avx512     | x86_64        | Ubuntu 24.04   | -                  | +avx512f,+avx512bf16,+avx512fp16,+avx512ifma,+avx512vbmi,+avx512vbmi2,+avx512vl,+avx512vnni,+f16c,+sse4.2 |
| linux-x86_64-cuda       | x86_64        | Ubuntu 24.04   | 13.1.0             | +avx2,+avx,+sse4.1,+f16c                                                  |
| linux-x86_64-cuda-nccl  | x86_64        | Ubuntu 24.04   | 13.1.0             | +avx2,+avx,+sse4.1,+f16c                                                  |
| linux-aarch64           | ARM64         | Ubuntu 24.04   | -                  | +fp16,+neon                                                               |
| linux-aarch64-cuda      | ARM64         | Ubuntu 24.04   | 13.1.0             | +fp16,+neon                                                               |

## Which build to choose

| Target Name             | Use when                   |
|-------------------------|----------------------------|
| macos                   | Using any mac              |
| windows-x86_64          | Windows                    |
| windows-x86_64-cuda     | Windows + Cuda             |
| linux-x86_64            | Normal linux               |
| linux-x86_64-avxvnni    | Alderlake, Arrowlake, Zen5 |
| linux-x86_64-avx512     | Zen4/5, XEON,              |
| linux-x86_64-cuda       | Cuda                       |
| linux-x86_64-cuda-nccl  | Cuda with NCCL             |
| linux-aarch64           | Using arm64, this is for low end models like jetson, raspberry pi. Not neoverse! |
| linux-aarch64-cuda      | Cuda on arm64              |

## Glibc

Sadly candle doesn't compile on linux-musl so it is dependent on the glibc version.
These builds use the latest glibc in Ubuntu 24.04, so you will have to go through the pain and update that on your local linux.

## CPU Flags Description

The CPU flags used in this nightly build configuration control specific instruction set extensions that enable hardware acceleration for various operations:

### Common Flags:
- **`+avx2`**: Advanced Vector Extensions 2 - provides 256-bit vector instructions for faster floating-point, integer, and logical operations
- **`+avx`**: Basic AVX support (128/256-bit vectors)
- **`+sse4.1`** / **`+sse4.2`**: Streaming SIMD Extensions 4.x - additional multimedia instruction sets
- **`+f16c`**: Half-Precision Floating Point Conversion hardware acceleration

### Specialized Flags:
- **`+avxvnni`**: AVX-VNNI (Vector Neural Network Instructions) for optimized AI/ML operations on Intel CPUs with VNNI support
- **AVX512 Family** (`linux-x86_64-avx512` target):
  - `+avx512f`: Foundation instructions (512-bit vectors)
  - `+avx512bf16`: BFloat16 conversion for better AI performance
  - `+avx512fp16`: Half-Precision Floating Point support
  - `+avx512ifma`: Integer Fused Multiply-Add operations
  - `+avx512vbmi`, **`+avx512vbmi2`**: Bit Manipulation Instructions for advanced data processing
  - `+avx512vl`: Vector Length Extensions (allows mixing of vector sizes)
  - `+avx512vnni`: AVX-VNNI instructions optimized for neural network operations

### ARM64 Flags:
- **`+fp16`**: Half-Precision Floating Point support on ARM processors
- **`+neon`**: NEON SIMD extensions (128-bit vectors) for mobile and embedded devices
