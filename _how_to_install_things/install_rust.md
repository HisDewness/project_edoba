## 🛠️ Local Development Setup (macOS + Homebrew)

### 1. Install Rust
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
`rustup update`

#### If rustup is not recognized, add the shell to your PATH ~/.zshrc (or ~/.bashrc if using bash)
`. "$HOME/.cargo/env"`

### 2. Install Node.js & Tauri CLI
`brew install node`
`npm install -g @tauri-apps/cli`

### 3. Install Xcode Command Line Tools
`xcode-select --install`

### 4. (Optional) Check Tauri Environment
`npx tauri info`

### 5. Run the App Locally
`npm install`
`npm run tauri dev`

