#!/bin/bash

echo "Pulling postgres:alpine image..."
docker pull postgres:alpine
echo "Renaming postgres:alpine image..."
docker tag postgres:alpine postgres:egline
docker rmi postgres:alpine
echo "Running postgres:egline image..."
docker run -d --name postgres-egline -e POSTGRES_PASSWORD=postgres -e POSTGRES_USER=admin -e POSTGRES_DB=egline -p 127.0.0.1:5432:5432/tcp postgres:egline