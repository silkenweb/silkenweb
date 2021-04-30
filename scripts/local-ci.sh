#!/usr/bin/env bash

docker build . -t silkenweb-github-actions
act -P ubuntu-latest=silkenweb-github-actions:latest "$@"
