# 🐧 Linutils

Welcome to Linutils, your go-to Rust-based tool for making Linux installation a breeze. 

Everything’s pre-configured to my taste, but you can easily customise it to match your own preferences!

![linutils](https://github.com/user-attachments/assets/9711d70b-7215-42b9-9c81-9863514e38ce)

## 🚀 Features

- Multi-Distro Support: Works with both Ubuntu and Fedora.
- Automated Software Installation:
  - Development tools like neovim, lazygit and docker.
  - Productivity apps like discord.
- Environment Setup:
  - Configure your terminal with zsh, alacritty and kitty.
  - Set up your workflow with i3 and tmux.
  - Custom environment variables and aliases.
- Customisation: modify the configuration file and the bash scripts to add or remove packages and settings as needed.
- Blazingly fast: built with Rust for performance and reliability.

## ⚙️ Installation

1. Clone the Repository:
```bash
git clone https://github.com/matteo-luraghi/linutils.git
cd linutils
```
2. Make sure to have [rust installed](https://www.rust-lang.org/tools/install)

3. Build the binary and run the app
```bash
cargo build --release
./target/release/linutils
```
4. Follow the On-Screen Prompts:
The application will guide you through selecting your distro and packages to install and set up.

## 🛠 Customization

1. Update the src/config.toml file: Add any new packages or distros that you want to include in your setup.

2. Create a folder for new distros: if you've added a new distro, create a corresponding directory under src/commands with the name of the distro.
   
3. Create bash scripts for each package: inside the directory for each distro (under src/commands), create bash scripts for the packages you’ve added. These scripts should handle the installation or configuration of those packages on the respective distro.

4. If you need to use configuration files in your bash scripts, you can easily save them in the src/utils directory.

## 📂 Project Structure

    linuitls/
    ├── src/
    |   ├── commands/         # All the bash scripts needed to install packages
    |   |  ├── _tests/        # Bash scripts for testing
    |   |  ├── fedora/        # Bash scripts to set up and install packages in Fedora
    |   |  └── ubuntu/        # Bash scripts to set up and install packages in Ubuntu
    │   ├── utils/            # Config files, wallpaper and fonts
    │   ├── config.rs         # Configuration file handling
    │   ├── main.rs           # Entry point of the application
    │   ├── processing.rs     # Processes logic
    |   ├── tui.rs            # TUI logic
    │   └── config.toml       # Configuration file for packages and distros
    ├── README.md             # This README file
    └── Cargo.toml            # Rust project file

## 🧑‍💻 Contributing

Feel free to fork this repository and submit pull requests if you'd like to add packages and/or distributions. 

See [Customization](#-customization) for more info.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
