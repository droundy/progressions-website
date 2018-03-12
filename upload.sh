#!/bin/sh

set -ev

fac -v

SITEDIR=public_html/progression

ssh science.oregonstate.edu "rm -fv $SITEDIR"
ssh science.oregonstate.edu "mkdir -p $SITEDIR"

scp output/* science.oregonstate.edu:$SITEDIR/

# rsync -v *.svg *.png *.py *.html *.css $SITE/
# rsync -v figs/*.svg $SITE/figs/

# rm -f hw/solution*
# rsync -v -r hw/*.pdf $SITE/hw/

