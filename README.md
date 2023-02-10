# sh-to-telegram

![build workflow](https://github.com/itsrobli/sh-to-telegram/actions/workflows/release.yml/badge.svg)

This CLI app sends pre-canned messages to Telegram.

## Motivations

I wanted to learn Rust. And I wanted to know when long-running downloads started and finished.

## TODO

- [ ] Sign the app for macOS.
- [ ] Add build for Windows. 
- [ ] Remove the prints from `init_check()` when things are fine.
- [ ] End-to-end tests of the CLI commands.
- [ ] Feels like too much `.unwrap()` usage.

# Installation

1. [Download](/releases) and unzip the binary for your system. Since Rust is awesome, feel free to just `cargo build`.
2. The binary should be in `~/bin/` because this is where the CLI app's config and logs will go.
3. Add this location to your PATH e.g. `export PATH="$HOME/bin:$PATH"`.
4. First run will panic but create the config file in `~/bin/sh-to-telegram.toml`.
    - If you're on macOS you need to go to settings and allow this app to run. I need to sign it.
5. Put your Telegram details in there. This app uses [Telegram's Bot API](https://core.telegram.org/bots#how-do-i-create-a-bot)
6. use the `--help` flag to see further instructions.

# Deployment steps

1. Update `./CHANGELOG.md` with new version.
2. Tag the commit in the format like `v0.3.1`.
3. GitHub actions will run and create all binaries and a GitHub Release. See `./.github/workflows/release.yml`
