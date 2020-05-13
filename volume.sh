#!/bin/sh

pm2 start "cargo run --release --bin vol_per_1" --name "vol" --no-autorestart
