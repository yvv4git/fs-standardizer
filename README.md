# fs-standardizer

## About
Standardizes file names



## Run
```bash
cargo run -- .
cargo run -- -c config.toml
cargo run -- -c config.toml -r ~/Downloads/media
cargo run -- -c config.toml -v -r ~/Downloads/media/tmp
cargo run -- -c config.toml -f -v -r ~/Downloads/media/tmp
```
Where:
- `-c` - path to configuration file
- `-r` - scan recursively
- `-v` - show old and new file names
- `-f` - preview mode (without actual renaming)