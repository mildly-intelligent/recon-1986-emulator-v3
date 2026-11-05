#!/bin/bash

image="$(magick "$1" -modulate 100,100,100 -gravity center -crop 4:3 /dev/stdout | chafa -f symbols --symbols=solid --dither=noise -s 80x60 -c 256)"

# image="$(echo "$image" | tail -c +11 | head -c -11 | sed 's/\x1b\[38;5;//g')"

if [[ "$2" = "" ]]; then
    printf "$image"
else
    printf "$image" | tee "$2"
fi