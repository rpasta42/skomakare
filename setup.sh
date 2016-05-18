#!/bin/bash

if [ -d "assets" ]; then
   cd assets; git pull origin master; pwd;
else
   git clone git@github.com:KostyaKow/skomakare-examples-data.git
   mv skomakare-examples-data assets
fi
