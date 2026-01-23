#!/bin/bash
set -xe

# ARGS: {environment}
# Example: ./DEVOPS/deploy.sh prod

# BASIC example of a deploy script.
# Run from the root directory of the project
# You should run build_all.sh before this.

USER=name # FIX ME
IP=1.2.3.4 # FIX ME

ENV=$1
if [ "$ENV" != "beta" ] && [ "$ENV" != "prod" ]; then
    echo "Invalid environment. Please specify either beta or prod"
    exit 1
fi

cd ./DEVOPS/"$ENV"

ssh-add ~/.ssh/id_ed25519 # <-- UPDATE ME add your ssh keys here for remote machine

# Deploy
scp docker-compose.yml "$USER@$IP:~/$ENV"
ssh carson@142.93.81.211 "docker compose -f ~/$ENV/docker-compose.yml --env-file ~/$ENV/.env up -d"
