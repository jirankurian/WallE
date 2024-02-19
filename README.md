# Wall E

## Installing Rust and Cargo

Rust is a modern programming language that offers performance, reliability, and productivity. Cargo is Rust's package manager and build system. This guide covers the installation of both Rust and Cargo on macOS, Windows, and Linux.

### macOS and Linux

1. Open the Terminal.
2. Install Rust and Cargo using rustup by running the following command:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Follow the on-screen instructions to complete the installation.

### Windows

1. Download the rustup-init.exe from **[rustup](https://rustup.rs/)**.
2. Run the downloaded rustup-init.exe file.
3. Follow the on-screen instructions to complete the installation.

## Verifying Rust and Cargo Installation

Verify that Rust and Cargo are correctly installed by running the following commands in your terminal or command prompt:

```
rustc --version
cargo --version
```

## Adding the `armv7a-none-eabi` Target

To compile code for ARM Cortex-A processors, you need to add the **`armv7a-none-eabi`** target to your Rust installation.

1. Open a Terminal (macOS and Linux) or Command Prompt (Windows).
2. Run the following command:

```
rustup target add armv7a-none-eabi
```

### Verifying the Target Installation

Verify that the `armv7a-none-eabi` target has been added by listing the installed targets:

```
rustup target list --installed
```

Ensure `rmv7a-none-eabi` appears in the output.

## Troubleshooting

- If you encounter any issues, ensure that Rust and Cargo are correctly installed and up to date by running **`rustup update`**.
- For issues related to the **`armv7a-none-eabi`** target, consult the Rust documentation or seek assistance from Rust community forums.