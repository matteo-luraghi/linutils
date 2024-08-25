# 🔧 Linutils [WORK IN PROGRESS]

Welcome to Linutils, a Rust-based application that streamlines the setup of a new Linux installation. Whether you're running Ubuntu or Fedora, this tool helps you quickly and effortlessly configure your system by installing your favorite software and setting up your environment just the way you like it.
## 🚀 Features

- Multi-Distro Support: Works seamlessly with both Ubuntu and Fedora.
- Automated Software Installation:
  - Essential tools like Docker, Go, Python, Java.
  - Productivity apps like Discord.
  - Development tools like Neovim and Alacritty.
- Environment Setup:
  - Configure your workspace with Hyprland and Alacritty.
  - Custom environment variables and aliases.
- Easy Customization: Modify the configuration file to add or remove packages and settings as needed.
- Blazingly fast: Built with Rust for performance and reliability.

## ⚙️ Installation

1. Clone the Repository:
```bash
git clone https://github.com/matteo-luraghi/linutils.git
cd linux-configurator
```
2. Build the Application (ensure you have rust installed):
```bash
cargo build --release
```
3. Run the Application:
```bash
./target/release/linutils
```
4. Follow the On-Screen Prompts:
The application will guide you through selecting your distro and configuring your environment.

## 🛠 Customization

You can customize the setup process by editing the configuration file located at config.toml in the project directory. Add or remove software packages, environment variables, or any other setup instructions as needed.

## 📂 Project Structure

    linuitls/
    ├── src/
    │   ├── main.rs           # Entry point of the application
    │   ├── config.rs         # Configuration file handling
    │   ├── installer.rs      # Installation logic for each package
    │   ├── environment.rs    # Environment setup logic
    │   └── ...               # Additional modules
    ├── config.toml           # Default configuration file
    ├── README.md             # This README file
    └── Cargo.toml            # Rust project file

## 🧑‍💻 Contributing

Feel free to fork this repository and submit pull requests if you have any ideas for new features, supported distros, or improvements!

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Happy coding, and enjoy your perfectly configured Linux system! 🎉

### Future reference

How to exec a specific file

```bash
curl -fsSL https://raw.githubusercontent.com/matteo-luraghi/linutils/master/ubuntu/apps/discord.sh | sudo sh
```
