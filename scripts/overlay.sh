#!/bin/bash

# Check for at two arguments
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 /path/to/foreground/image /path/to/background/image"
    exit 1
fi



# Overlay the foreground image on the background image
composite -gravity center $1 $2 output/final.png