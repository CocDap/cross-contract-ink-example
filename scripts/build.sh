#!/bin/bash

# Build flipper contract  
cargo contract build --manifest-path flipper/Cargo.toml


# Build other_flipper contract  
cargo contract build --manifest-path other_flipper/Cargo.toml