#!/bin/bash

#Run tally_votes code, first tests and then main app
if cargo test; then
    echo "Tests passed. Running the application..."
    cargo run
else
    echo "Tests failed. Not running the application."
    exit 1
fi