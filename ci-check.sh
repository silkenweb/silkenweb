#!/usr/bin/env bash

set -euo pipefail

time docker build . -t silkenweb-github-actions
time act -P ubuntu-latest=silkenweb-github-actions:latest "$@"
