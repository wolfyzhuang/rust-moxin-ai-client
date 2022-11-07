#!/bin/bash

WASMEDGE_CMD=wasmedge

if [ -n "$WASMEDGE_BUILD_DIR" ]; then
    WASMEDGE_CMD=$WASMEDGE_BUILD_DIR/tools/wasmedge/wasmedge
fi

check_wasmedge() {
    if command -v $WASMEDGE_CMD > /dev/null; then
        local wasmedge_ou