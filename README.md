# m2
Requirements:
- [Rust](https://rust-lang.org/)
- [Docker](https://www.docker.com/)
- [GNU Make](https://www.gnu.org/software/make/)

## Quick start
To run the server (development mode):
```shell
cargo run -- ./config.yml
```

To run the server in Docker (release mode):
```shell
docker compose up -d --build
```
> [!IMPORTANT]  
> If the server fails to start because it is unable to open the database file,
> create m2.sqlite3 at the project's root and run the above command again.

- REST API: http://localhost:55432
- Swagger UI: http://localhost:55432/swagger-ui

