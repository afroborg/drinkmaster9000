#!/bin/bash -e

# check that the first argument is either frontend or pi, or that it is empty
if [[ $1 == "frontend" ]] || [[ -z $1 ]]; then
    sh ./scripts/deploy-frontend.sh
fi

if [[ $1 == "pi" ]] || [[ -z $1 ]]; then
    sh ./scripts/deploy-pi.sh
fi