#!/bin/sh

set -ev

if test -e ~/box/Learning\ Progressions\ for\ Partial\ Derivatives/Learning\ Progression\ Database.xlsx; then
    # copy latest version of the learning progression from box
    rsync -a ~/box/Learning\ Progressions\ for\ Partial\ Derivatives/Learning\ Progression\ Database.xlsx progression.xlsx
    rsync -a ~/box/Learning\ Progressions\ for\ Partial\ Derivatives/Figures/* figs/
    git add figs
fi

fac

SITEDIR=public_html/progression

ssh science.oregonstate.edu "rm -rf $SITEDIR"
ssh science.oregonstate.edu "mkdir -p $SITEDIR"

scp output/* science.oregonstate.edu:$SITEDIR/

# rsync -v *.svg *.png *.py *.html *.css $SITE/
# rsync -v figs/*.svg $SITE/figs/

# rm -f hw/solution*
# rsync -v -r hw/*.pdf $SITE/hw/

