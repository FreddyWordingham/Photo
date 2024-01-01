#!/bin/bash

# Check for at least one argument
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 /path/to/images"
    exit 1
fi

# Image name
IMAGE_NAME="${1##*/}"

# Input directory of image tiles
IMAGE_PATH=./$1/*-colour.png

# Output file
OUTPUT_PATH=./$1/../$IMAGE_NAME-colour.png

# Create montage
montage -tile 8x8 -geometry +0+0 -background none $IMAGE_PATH $OUTPUT_PATH &&
echo "Montage created at $OUTPUT_PATH"

# Input directory of image tiles
IMAGE_PATH=./$1/*-time.png

# Output file
OUTPUT_PATH=./$1/../$IMAGE_NAME-time.png

# Create montage
montage -tile 8x8 -geometry +0+0 -background none $IMAGE_PATH $OUTPUT_PATH &&
echo "Montage created at $OUTPUT_PATH"
