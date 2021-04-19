#!/bin/bash

# build docker image
docker build -f Dockerfile.aws -t docker-test .

# push docker image to ecr

# deploy cloudformation template