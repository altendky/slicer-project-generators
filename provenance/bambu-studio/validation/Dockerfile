FROM ubuntu:24.04@sha256:4fbb8e6a8395de5a7550b33509421a2bafbc0aab6c06ba2cef9ebffbc7092d90

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install --yes --no-install-recommends \
        dbus-x11 \
        libfontconfig1 \
        libgl1 \
        libgstreamer-plugins-base1.0-0 \
        libgstreamer1.0-0 \
        libgtk-3-0t64 \
        libwebkit2gtk-4.1-0 \
        unzip \
        xauth \
        xvfb \
    && rm -rf /var/lib/apt/lists/*
