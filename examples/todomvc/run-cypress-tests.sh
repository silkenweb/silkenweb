#!/usr/bin/env bash

set -euo pipefail

# We build first to try and mitigate cypress timing out waiting for our server.
trunk build
trunk serve --release --no-autoreload --ignore=cypress-tests &
trap 'kill %%' EXIT

(
    cd cypress-tests

    if [[ $# = 1 && "$1" = "gui" ]]; then
        npx cypress open
    else
        npx cypress run
    fi
)
