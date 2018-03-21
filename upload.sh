#!/bin/sh

set -ev

# copy latest version of the learning progression from box
cp ~/box/Learning\ Progressions\ for\ Partial\ Derivatives/Learning\ Progression\ Database.xlsx progression.xlsx

fac

SITEDIR=public_html/progression

ssh science.oregonstate.edu "rm -rf $SITEDIR"
ssh science.oregonstate.edu "mkdir -p $SITEDIR"

scp output/* science.oregonstate.edu:$SITEDIR/

# rsync -v *.svg *.png *.py *.html *.css $SITE/
# rsync -v figs/*.svg $SITE/figs/

# rm -f hw/solution*
# rsync -v -r hw/*.pdf $SITE/hw/

