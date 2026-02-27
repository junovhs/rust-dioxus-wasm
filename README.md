# Rust + Dioxus + WASM (Golden Master)

Production-ready template for building WebAssembly apps with Rust.
Pre-wired for **Cloudflare Pages** and **GitHub Actions**.

## 🚀 Usage (Clone & Detach)

Use this workflow to start a fresh project without linking back to this template.

### 1. Setup
```bash
# 1. Clone into your new project folder
git clone https://github.com/junovhs/rust-dioxus-wasm my-new-app

# 2. Enter folder
cd my-new-app

# 3. Detach (Remove git history)
rm -rf .git
# Windows PowerShell: Remove-Item -Recurse -Force .git

# 4. Start fresh
git init
git add .
git commit -m "Initial commit"
```

### 2. Rename Project
Update the name in these two files:
1.  **`Cargo.toml`**: Change `name = "rust-dioxus-wasm"` to your project name.
2.  **`Dioxus.toml`**: Change `name` and `title`.

### 3. Run
```bash
dx serve
```
Server runs at [http://localhost:8080](http://localhost:8080).

---

## 🏗️ Architecture

*   **Logic:** `src/` (Rust/Dioxus).
*   **JS Interop:** `assets/js/bridge.js` handles browser APIs (Audio, Maps, etc).
*   **Routing:** `_redirects` file ensures SPA routing works on Cloudflare/Netlify.
*   **CI/CD:** `.github/workflows/deploy.yml` builds release and deploys on push to main.

## 🛠️ Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install)
*   [Dioxus CLI](https://dioxuslabs.com/learn/0.5/getting_started): `cargo install dioxus-cli`
*   WASM Target: `rustup target add wasm32-unknown-unknown`
