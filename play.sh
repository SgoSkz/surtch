#!/usr/bin/env bash
# NOTE: requires rofi to be installs to run
everything=$(rofi -dmenu)
output=`./target/release/surtch -s "${everything}" | ./script.sh `
mpv `echo "${output}" | sed "s/^.*\tVidID: \(.*\)\t.*$/https:\/\/youtube.com\/watch?v=\1/"`
