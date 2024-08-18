default:
    @just --list

server:
    cargo run --package idp-server

migrate:
    @cargo run --package idp-migration