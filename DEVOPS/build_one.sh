#!/bin/bash
set -xe

# BASIC example of a build and deploy script. You should run deploy.sh after this.
# Run from the root directory of the project

USER=john_doe # FIX ME
IP=192.168.1.1 # FIX ME

ENV=$1
if [ "$ENV" != "beta" ] && [ "$ENV" != "prod" ]; then
    echo "Invalid environment. Please specify either beta or prod"
    exit 1
fi

BIN=$2

cd ./DEVOPS/"$ENV"

# Build
docker compose --env-file .env build $BIN
docker save "${ENV}_${BIN}:latest" -o "${ENV}_${BIN}.tar"

# Copy to remote
ssh-add ~/.ssh/id_ed25519 # <-- UPDATE ME add your ssh keys here
ssh "$USER@$IP" "mkdir -p ~/$ENV"
scp "${ENV}_${BIN}.tar" "$USER@$IP:~/$ENV"
scp .env "$USER@$IP:~/$ENV" # Using .env is not recommended. Consider adding environments to machine.

# Deploy
ssh "$USER@$IP" "docker load -i ~/${ENV}/${ENV}_${BIN}.tar"

# Cleanup
rm "${ENV}_${BIN}.tar"
