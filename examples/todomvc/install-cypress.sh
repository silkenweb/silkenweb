#!/usr/bin/env bash

set -euo pipefail

(
    cd cypress-tests
    npm ci
)
