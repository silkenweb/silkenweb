#!/usr/bin/env bash

set -euo pipefail

(
    cd cypress-tests
    npm install
    npm install cypress
)
