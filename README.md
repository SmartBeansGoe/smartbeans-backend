# SmartBeans - Backend

## Installation instructions

### Prerequisites

- Rust (see [rust-lang.org](https://www.rust-lang.org/learn/get-started) for installation instructions)
- MySQL/MariaDB (on Ubuntu 'mysql-server'/'mariadb-server')
  - You'll need a database and a user who can read and write it

### Installation

- Clone or download this repository
- Copy `SettingsDefault.toml` to `Settings.toml` and adjust the values if necessary.
  - See the [wiki](https://github.com/SmartBeansGoe/smartbeans-backend/wiki/Settings) for more information.
- Compile: `cargo build` or `cargo build --release` (for production)
- Start the backend by executing `cargo run [--release]`.
  - Alternative option: Use a systemd-service (see below).

The server should now run on http://localhost:8000 (you can change this in `Settings.toml`).

#### systemd service (optional)

- Copy the following to `/etc/systemd/system/smartbeans-backend.service`. Replace \<username> by the user you want to run the application and <path_to_repo> by the path to this repository.

```
[Unit]
Description=SmartBeans Backend
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
Restart=on-failure
RestartSec=1
User=<username>
WorkingDirectory=<path_to_repo>
ExecStart=<path_to_repo>/target/release/backend

[Install]
WantedBy=multi-user.target
```

- Run `sudo systemctl daemon-reload`.
- Now you can use `sudo systemctl (start|stop|enable|disable) smartbeans.service` to start/stop the application and enable/disable autostart.

### Backup and Update

- All relevant data is stored in the database and Settings.toml. It should be sufficient to backup these files. If you want to be extra sure, you can of course save the entire folder.
- To update to a newer version, just pull the changes and recompile (ans restart the systemd service, if you use it).

## Documentation

- [Routes](https://github.com/SmartBeansGoe/smartbeans-backend/wiki/Routes)
- [Database](https://github.com/SmartBeansGoe/smartbeans-backend/wiki/Database)

## License

TODO