#!/usr/bin/env bash

set -euo pipefail

trunk serve --ignore=cypress-tests &
trap 'kill %%' EXIT

(
    cd cypress-tests

    if [[ $# = 1 && "$1" = "gui" ]]; then
        npx cypress open
    else
        npx cypress run
    fi
)
