#!/bin/bash
set -xe

# BASIC example of a build and deploy script (not fully tested)

# Setup remote directory
ssh myuser@ip 'mkdir -p ~/prod_app'

# Build
scp docker-compose.yml myuser@ip:~/prod_app
docker build -t prod_app:latest .
docker save prod_app:latest -o prod_app.tar
ssh-add ~/.ssh/id_ed25519_prod_app # <-- UPDATE ME add your ssh keys here for remote machine
scp prod_app.tar myuser@ip:~/prod_app
scp .env myuser@ip:~/prod_app

# Deploy
ssh myuser@ip << 'EOF'
cd ./prod_app
docker compose down app
docker rmi prod_app:latest
docker load -i prod_app.tar
docker compose up -d
exit
EOF

# Cleanup
rm prod_app.tar
