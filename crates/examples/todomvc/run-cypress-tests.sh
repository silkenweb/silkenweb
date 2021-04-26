#!/usr/bin/env bash

set -euo pipefail

trunk build
trunk serve --ignore=cypress-tests &
trap 'kill %%' EXIT

(
    cd cypress-tests
    npx cypress run
)
