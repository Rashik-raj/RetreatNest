# My Retreat Nest
An app, where users can find the hotels, resorts, farmhous of their choice with ease.

# Requirements
- install rust and cargo
```bash
curl https://sh.rustup.rs -sSf | sh
```
- Install postgres database, follow this [link](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-postgresql-on-ubuntu-20-04).
- Restore `.env` file taking help from `example.env` file.
- Apply migration, take help from below.
- Run the server using one of the below command.

## Production

### Running a server
```bash
cargo run --release
```

## Development

### Installing additional depenndencies
```bash
cargo install sea-orm-cli 
cargo install cargo-watch
```

### Running a server with auto-reload
```bash
cargo watch -x run
```
### Running a server without auto-reload
```bash
cargo run
```

### Create a migration
```bash
sea-orm-cli migrate generate NAME_OF_MIGRATION [-d src/migrations]
```

### Applying a migrations
```bash
sea-orm-cli migrate up
```

### Generate entity from migration being applied
```bash
sea-orm-cli generate entity -o src/entities  
```