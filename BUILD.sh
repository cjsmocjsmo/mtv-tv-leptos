#!/bin/bash
cd /home/whitepi/mtv-tv-leptos;

git pull;

trunk clean;

trunk build --release;

arch=$(uname -m);

if [ "$arch" = "aarch64" ]; then
    docker build -t mtvtvlep:arm64 -f ./arm64/Dockerfile .
    docker run -d -p 9093:80 mtvtvlep:arm64
fi

if [ "$arch" = "armv7l" ]; then
    docker build -t mtvtvlep:arm32 -f ./arm32/Dockerfile .
    docker run -d -p 9093:80 mtvtvlep:arm32
fi

if [ "$arch" = "x86_64" ]; then
    docker build -t mtvtvlep:amd64 -f ./amd64/Dockerfile .
    docker run -d -p 9093:80 mtvtvlep:amd64
fi