#!/bin/sh
set -x
while true; do
    sed -i 's/value = 43/value = 42/' mconfig.conf
    sleep 1
    sed -i 's/value = 42/value = 43/' mconfig.conf
    sleep 1
done
