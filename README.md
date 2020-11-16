# Very SmartApe - Backend

## Prerequisites

- Rust nightly
    - Install rustup (see [here](https://www.rust-lang.org/learn/get-started) for installation instructions)
    - Download the nightly toolchain (`rustup toolchain install nightly`)
    - Select the nightly toolchain
        - either globally... (`rustup default nightly`)
        - ...or locally in the project folder (`rustup override set nightly`)
- SQLite 3 (you possibly also need some sqlite3-dev package)

## Installation and Execution

- Clone this repository.
- Copy `.env.sample` to `.env` and adjust the containing values accordingly.
    - For some reason you have to recompile in order to apply changes in .env to the application.
- Execute `cargo run` or `cargo run --release` (for production).
    - Alternative option: Use a systemd-service (see below).
- The server runs on localhost:4224 (dev) or 0.0.0.0:4224 (production).
    - You can change this in Rocket.toml (see [here](https://rocket.rs/v0.4/guide/configuration/#rockettoml) for more information)
- Not neccessary, but strongly recommended for use in production: Place the server behind a reverse proxy and enable TLS.
    
### systemd-service

- Copy the following to `/etc/systemd/system/verysmartape.service`. Replace \<username> by the user you want to run the application and <path_to_repo> by the path to this repository. Change the path to cargo if you have installed it somewhere else.

```
[Unit]
Description=Very SmartApe Backend
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
Restart=on-failure
RestartSec=1
User=<username>
WorkingDirectory=<path_to_repo>
ExecStartPre=/home/<username>/.cargo/bin/cargo build --release
ExecStart=<path_to_repo>/target/release/backend

[Install]
WantedBy=multi-user.target
```

- Run `sudo systemctl daemon-reload`.
- You might want to compile manually before starting the systemd-unit (otherwise it might time out due to the long compilation).
- Now you can use `sudo systemctl (start|stop|enable|disable) verysmartape.service` to start/stop the application and enable/disable autorun.
    
## Backup and Update

- All relevant data is stored in db.sqlite and .env (and Rocket.toml if you modified it). It should be sufficient to backup these files. If you want to be extra sure, you can of course backup the whole folder.
- To update to a newer version, just pull the changes (or download the new version and copy db.sqlite, .env[, Rocket.toml])

## API Documentation

- see [here](API_doc.md)
