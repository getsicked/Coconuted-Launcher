# Coconuted Launcher

Coconuted Launcher is a Windows launcher and client fork of [Horizon](https://github.com/horizon-rust/horizon). It supports local names and Ely.by accounts. The matching client in this repository handles Ely.by server authentication.

## Build on Windows

Install the stable Rust toolchain and the Windows C++ build tools. Build both programs from the same commit:

```powershell
cargo build --release --manifest-path launcher/Cargo.toml
cargo build --profile fast -p horizon-client --bin horizon-client
```

The output files are `launcher/target/release/horizon-launcher.exe` and `target/fast/horizon-client.exe`. Put them in the same folder, naming the client `coconuted-client.exe`. The launcher can then be named `Coconuted Launcher.exe`.

## Accounts

In Settings → Account, sign in to Ely.by or choose a local name. Ely.by sign-in stores session tokens in the operating system keychain. A local name does not authenticate on online-mode servers. Microsoft online-mode servers require a Microsoft account with Minecraft: Java Edition.

## Source and licensing

This fork is based on Horizon and retains its MIT license. See [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). It is unofficial and is not associated with Mojang, Microsoft, or Ely.by. Minecraft game files and assets are not included; the launcher obtains or prepares them on the user's machine.
