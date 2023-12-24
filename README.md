# Hangitbot

A boring bot for hanging your boss up.

```usage
hangitbot 0.1.0
A boring bot for hanging your boss up.

USAGE:
    hangitbot [OPTIONS] --tgbot-token <TGBOT_TOKEN>

OPTIONS:
    -d, --database-uri <DATABASE_URI>
            Database URI [env: DATABASE_URI=] [default:
            sqlite:///hangitbot.db]

    -D, --debug
            Enable debug mode

    -h, --help
            Print help information

    -t, --tgbot-token <TGBOT_TOKEN>
            Telegram bot token [env: TGBOT_TOKEN=]

    -V, --version
            Print version information
```

## build

You should use `nightly` build kit.

```bash
rustup default nightly
cargo build
```

Or simply use docker.

```bash
docker build -t bot .
```
