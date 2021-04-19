#!/bin/bash

# deploy cloudformation template

# build docker image
docker build -f Dockerfile.aws -t docker-test .

# push docker image to ecr