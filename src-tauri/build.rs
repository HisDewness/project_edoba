// Copyright 2025 Project Edoba
// SPDX-License-Identifier: Apache-2.0

// Hooks into the build process and allows the Tauri CLI to detect your plugin setup and app config
fn main() {
    tauri_build::build();
}