#!/bin/sh
cd $(dirname "$0")/../ && 
    cargo build && \
    cargo build --release
