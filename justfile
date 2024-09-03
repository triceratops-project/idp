default:
    @just --list

start:
    just frontend
    just server

proxy:
    caddy run -c Caddyfile

frontend:
    bun install
    bun run --filter=@idp/frontend build

server:
    cargo run --package idp-server

migrate:
    @cargo run --package idp-migration