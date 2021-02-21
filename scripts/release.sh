#!/bin/bash

cargo build

cp ../.env ../release/

cp ../target/debug/enfaria-we{b,b.exe} ../release/web/
cp -r ../enfaria-web/{static,templates,release} ../release/web/

cp ../target/debug/enfaria-serve{r,r.exe} ../release/server/
cp -r ../enfaria-server/{data,templates} ../release/server/

cp ../target/debug/enfaria-gam{e,e.exe} ../release/
cp ../enfaria-game/assets/ ../release/
