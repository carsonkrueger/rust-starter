#!/bin/bash
set -xe

# ARGS: {environment} {binary_name}
# Example: ./DEVOPS/build_one.sh prod app

# BASIC example of a build.
# Run from the root directory of the project
# You should run deploy.sh after this.

USER=name # FIX ME
IP=1.2.3.4 # FIX ME

ENV=$1
BIN=$2

if [ "$ENV" != "beta" ] && [ "$ENV" != "prod" ]; then
    echo "Invalid environment. Please specify either beta or prod"
    exit 1
fi

cd ./DEVOPS/"$ENV"

# Build
docker compose --env-file .env build $BIN
docker save "${ENV}_${BIN}:latest" -o "${ENV}_${BIN}.tar"

# Copy to remote
ssh-add ~/.ssh/id_ed25519 # <-- UPDATE ME add your ssh keys here
ssh "$USER@$IP" "mkdir -p ~/$ENV"
scp "${ENV}_${BIN}.tar" "$USER@$IP:~/$ENV"
scp .env "$USER@$IP:~/$ENV" # Using .env is not recommended. Consider adding environments to machine.
ssh "$USER@$IP" "docker load -i ~/${ENV}/${ENV}_${BIN}.tar"

# Cleanup
rm "${ENV}_${BIN}.tar"
cd -
