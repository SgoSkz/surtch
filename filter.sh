#!/usr/bin/env bash
# Used for filtering text
echo ${@} | sed "s/^.*Thmb: //" | sed "s/\x0f//g"
