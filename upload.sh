#!/bin/sh

set -ev

# if test -e ~/box/Learning\ Progressions\ for\ Partial\ Derivatives/Learning\ Progression\ Database.xlsx; then
#     # copy latest version of the learning progression from box
#     rsync -a ~/box/Learning\ Progressions\ for\ Partial\ Derivatives/Learning\ Progression\ Database.xlsx progression.xlsx
#     rsync -a ~/box/Learning\ Progressions\ for\ Partial\ Derivatives/Figures/* figs/
#     git add figs
# fi

wget -O libraries/MathJax.js https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.1/MathJax.js?config=TeX-AMS_HTML
wget -O libraries/jquery.js https://code.jquery.com/jquery-3.3.1.min.js

fac

# cargo run --release -- --base-url 'http://physics.oregonstate.edu/~roundyd/progression'
cargo run --release -- --base-url 'https://paradigms.oregonstate.edu/progressions/derivatives'

SITEDIR=progressions/derivatives

ssh paradigms.oregonstate.edu "rm -rf $SITEDIR"
ssh paradigms.oregonstate.edu "mkdir -p $SITEDIR"

scp -r mirror/* paradigms.oregonstate.edu:$SITEDIR/

scp style.css paradigms.oregonstate.edu:$SITEDIR/

scp -r libraries paradigms.oregonstate.edu:$SITEDIR/

scp -r figs paradigms.oregonstate.edu:$SITEDIR/
