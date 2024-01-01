#!/bin/bash

# Check for a single argument
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 /path/to/image"
    exit 1
fi

# Image directory
IMAGE_DIR="${1%/*}"

# Image name
IMAGE_NAME="${1##*/}"

HALF_IMAGE_NAME="${IMAGE_NAME%.*}-half.${IMAGE_NAME##*.}"

QUARTER_IMAGE_NAME="${IMAGE_NAME%.*}-quarter.${IMAGE_NAME##*.}"

EIGHTH_IMAGE_NAME="${IMAGE_NAME%.*}-eighth.${IMAGE_NAME##*.}"

SIXTEENTH_IMAGE_NAME="${IMAGE_NAME%.*}-sixteenth.${IMAGE_NAME##*.}"


# Shrink image to 50%
convert $1 -resize 50% $IMAGE_DIR/$HALF_IMAGE_NAME

# Shrink image to 25%
convert $1 -resize 25% $IMAGE_DIR/$QUARTER_IMAGE_NAME

# Shrink image to 12.5%
convert $1 -resize 12.5% $IMAGE_DIR/$EIGHTH_IMAGE_NAME

# Shrink image to 6.25%
convert $1 -resize 6.25% $IMAGE_DIR/$SIXTEENTH_IMAGE_NAME