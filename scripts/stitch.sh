#!/bin/bash

# Check for at least one argument
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 /path/to/images"
    exit 1
fi

# Image name
IMAGE_NAME="${1##*/}"

# Input directory of image tiles
IMAGE_PATH=./$1/*.png

# Output file
OUTPUT_PATH=./$1/../$IMAGE_NAME.png

# Create montage
montage -tile 8x12 -geometry +0+0 $IMAGE_PATH $OUTPUT_PATH &&
echo "Montage created at $OUTPUT_PATH"
