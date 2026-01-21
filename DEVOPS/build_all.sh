#!/bin/bash
set -xe

# You should run deploy.sh after this

ENV=$1
if [ "$ENV" != "beta" ] && [ "$ENV" != "prod" ]; then
    echo "Invalid environment. Please specify either beta or prod"
    exit 1
fi

# Build All images (we only have one binary right now)
ssh-add ~/.ssh/id_ed25519 # <-- UPDATE ME add your ssh keys here
./DEVOPS/build_app.sh $ENV app
