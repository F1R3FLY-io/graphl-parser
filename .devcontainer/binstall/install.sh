#!/bin/sh

set -e

cargo install cargo-binstall
sudo chown -R vscode:rustlang /usr/local/cargo
