#!/bin/bash
set -xe

# ARGS: {environment}
# Example: ./DEVOPS/build_all.sh prod

# BASIC example of a build.
# Run from the root directory of the project
# You should run deploy.sh after this.

ENV=$1
if [ "$ENV" != "beta" ] && [ "$ENV" != "prod" ]; then
    echo "Invalid environment. Please specify either beta or prod"
    exit 1
fi

# Build All images (we only have one binary right now)
./DEVOPS/build_one.sh $ENV app
