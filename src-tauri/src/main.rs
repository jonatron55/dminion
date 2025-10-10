// Copyright (c) 2025 Jonathon B. Cobb
// Licensed under the MIT License

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    dminion_lib::run()
}
