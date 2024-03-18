#!/usr/bin/env bash
#
#

busctl --user call \
    com.thetechforest.WallsD \
    /com/thetechforest/WallsD \
    com.thetechforest.WallsD1 \
    SayHello \
    s \
    "wallsd"

