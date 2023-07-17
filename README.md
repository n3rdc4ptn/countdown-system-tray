# Countdown System Tray

This Tauri app displays a countdown timer in the system tray. I use it to show me how long I have left for my master thesis.

It is not as user friendly as I would like it to be, but it works for me. But if you want to contribute feel free to do so.

![Screenshot](docs/systemtray-icon.png)

## Build

```bash
cargo tauri build
```

## Installation

I will not upload binaries for this app. If you want to use it you have to build it yourself. I am not a registered Apple developer therefore I cannot sign binaries for macos.

## Configuration

The configuration file is located at $HOME/mcountdown.toml. You can specify the name of the countdown and the target date. The target date must be in the format YYYY-MM-DD.

```toml
name = "Master thesis"
target_date = "2023-08-12"
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
