evince thesis.pdf &
while inotifywait -qq -e modify thesis.tex; do { make > /dev/null ; }; done & 
