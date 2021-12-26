# Dockerfile for `act` (local github actions runner)
FROM catthehacker/ubuntu:act-latest

RUN apt-get update \
    && apt-get install --no-install-recommends -y \
        curl \
        # For headless browser tests
        firefox-geckodriver chromium-browser \
        # For Cypress
        libgtk2.0-0 libgtk-3-0 libgbm-dev libnotify-dev libgconf-2-4 libnss3 libxss1 libasound2 libxtst6 xauth xvfb \
    && rm -rf /var/lib/apt/lists/*

# Install NPM
RUN curl -sL https://deb.nodesource.com/setup_17.x | bash -
RUN apt-get update \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*
