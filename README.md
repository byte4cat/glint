# ✨ Glint

A lightweight, elegant note and task widget for your desktop.
**Glint** stays in the background of your workspace, providing quick access to your tasks or notes with a clean, Markdown-supported interface.

Built with **Rust**, **GTK4**, and **Layer Shell**, it is designed to be minimal, fast, and highly customizable.

**Currently, NOT SUPPORT Windows.**

# 🚀 Features

- **Layer Shell Integration**: Works as a desktop widget (stays behind or above windows).
- **Smart Resizing**: Automatically shrinks or grows to fit your content perfectly.
- **Markdown Support**: Uses Pango markup for bolding, colors, and styling.
- **Robust Configuration**: Gracefully handles missing settings and platform-specific paths.

# 🛠️ Installation

## 1. Prerequisites
Ensure you have the following C libraries installed on your system (Linux example):

- `gtk4`
- `gtk4-layer-shell`

## 2. Build from Source
```sh
git clone https://github.com/byte4cat/glint.git
cd glint
make install
# after that, you can run it by typing `glint` in the terminal
```

# ⚙️ Configuration

**Glint** looks for a `config.toml` file in your system's standard configuration directory.


| Platform | Path |
| -------- | ---------- |
| Linux     | `~/.config/glint/config.toml`     |
| MacOS | `~/Library/Application Support/glint/config.toml`       |


## Example `config.toml`

```toml
title = "✨ DAILY TASKS"
tasks = [
    "Finish the Rust project",
    "Go to the gym",
    "Read 20 pages of a book"
]

# Positioning
anchor_bottom = true
anchor_right = true
margin_bottom = 40
margin_right = 40

# Styling
background_color = "#1e1e2e"
text_color = "#cdd6f4"
font_size = 14
border_width = 3
border_color = "#89b4fa"
border_radius = 16
```

# ⌨️ Development
The included `Makefile` simplifies common tasks:

- make build: Compile the project.
- make run: Compile and launch Glint immediately.
- make release: Build an optimized binary for production.
- make fmt: Automatically format the code.
- make clean: Remove build artifacts.

# 📝 License
This project is licensed under the MIT License - see the LICENSE file for details.
