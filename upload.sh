#!/bin/sh

set -ev

if test -e ~/box/Learning\ Progressions\ for\ Partial\ Derivatives/Learning\ Progression\ Database.xlsx; then
    # copy latest version of the learning progression from box
    rsync -a ~/box/Learning\ Progressions\ for\ Partial\ Derivatives/Learning\ Progression\ Database.xlsx progression.xlsx
    rsync -a ~/box/Learning\ Progressions\ for\ Partial\ Derivatives/Figures/* figs/
    git add figs
fi

# wget -O libraries/MathJax.js https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.1/MathJax.js?config=TeX-AMS_HTML
# wget -O libraries/jquery.js https://code.jquery.com/jquery-3.3.1.min.js

fac

SITEDIR=public_html/progression

ssh science.oregonstate.edu "rm -rf $SITEDIR"
ssh science.oregonstate.edu "mkdir -p $SITEDIR"

scp -r output/* science.oregonstate.edu:$SITEDIR/

# rsync -v *.svg *.png *.py *.html *.css $SITE/
# rsync -v figs/*.svg $SITE/figs/

# rm -f hw/solution*
# rsync -v -r hw/*.pdf $SITE/hw/

