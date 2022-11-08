#!/bin/bash

WASMEDGE_CMD=wasmedge

if [ -n "$WASMEDGE_BUILD_DIR" ]; then
    WASMEDGE_CMD=$WASMEDGE_BUILD_DIR/tools/wasmedge/wasmedge
fi

check_wasmedge() {
    if command -v $WASMEDGE_CMD > /dev/null; then
        local wasmedge_output=$($WASMEDGE_CMD)
        if echo "$wasmedge_output" | grep -q 'nn-preload'; then
            return 0
        else
            echo "Wasmedge is installed but WASI NN plugin is not found."
            echo "Please download WASI NN plugin."
            echo "If you have already downloaded it, please set WASMEDGE_PLUGIN_PATH"
            return 1
        fi
    else
        e