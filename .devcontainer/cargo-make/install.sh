#!/bin/sh

set -e

cargo binstall cargo-make
sudo chown -R vscode:rustlang /usr/local/cargo
