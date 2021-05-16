#!/usr/bin/env bash

set -euo pipefail

docker build . -t silkenweb-github-actions
act -P ubuntu-latest=silkenweb-github-actions:latest "$@"
