#!/bin/bash

EDITOR="hx"
if pgrep -x "$EDITOR" > /dev/null
then
    echo "$EDITOR is already running"
else
    alacritty -e helix thesis.tex &
fi

VIEWER="evince"
if pgrep -x "$VIEWER" > /dev/null
then
    echo "$VIEWER is already running"
else
    evince thesis.pdf &
fi

while inotifywait -qq -e modify thesis.tex; do { make; }; done 
