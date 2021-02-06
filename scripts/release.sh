#!/bin/bash

cargo build

cp ../.env ../release/

cp ../target/debug/enfaria-we{b,b.exe} ../release/web/
cp -r ../enfaria-web/{static,templates,release} ../release/web/

cp ../target/debug/enfaria-serve{r,r.exe} ../release/server/
cp -r ../enfaria-server/{data,templates} ../release/server/

cp ../target/debug/enfaria-update{r,r.exe} ../release/

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    godot --path ../enfaria-game --export "Linux/X11" enfaria-game
else
    godot --path ../enfaria-game --export "Windows Desktop" ../release/enfaria-game.exe
fi
