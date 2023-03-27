evince thesis.pdf &
while inotifywait -qq -e modify thesis.tex; do { make; }; done 
