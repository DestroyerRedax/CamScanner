# CamScanner (Rust Core + Android App)

A document scanner built with a high-performance **Rust native core** and **Android CameraX frontend**, featuring automated APK builds via **GitHub Actions**.

## Architecture & Modules

| Path | Description |
| :--- | :--- |
| **`rust/Cargo.toml`** | Rust project configuration (`cdylib` and dependencies `image`, `lopdf`). |
| **`rust/src/lib.rs`** | C FFI boundary functions exposing scanner operations to Android. |
| **`rust/src/detector.rs`** | Document edge detection & corner quadrangle contour estimation. |
| **`rust/src/filters.rs`** | Image filters: Magic Color, B&W threshold, Grayscale, Shadow removal. |
| **`rust/src/ocr.rs`** | Text extraction and local full-text search indexing. |
| **`rust/src/editor.rs`** | Watermark overlays and signature drawing stamps. |
| **`rust/src/pdf_exporter.rs`** | Multi-page PDF generation and stream compression using `lopdf`. |
| **`app/src/main/java/com/scanner/app/MainActivity.kt`** | Android UI with CameraX integration. |
| **`.github/workflows/build-apk.yml`** | Automated GitHub Actions CI workflow to build release APKs on push. |

## Automated Cloud Build via GitHub Actions

1. When you push to the `main` branch, the GitHub Actions workflow triggers automatically.
2. It compiles the Rust core for `arm64-v8a`, `armeabi-v7a`, and `x86_64` using `cargo-ndk`.
3. Gradle builds the unsigned release APK.
4. Download your built APK under the **Artifacts** section of the GitHub Actions run!