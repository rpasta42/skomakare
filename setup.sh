#!/bin/bash

if [ -d "assets" ]; then
   cd assets; git pull origin master; pwd;
else
   git clone https://github.com/KostyaKow/skomakare-examples-data
   mv skomakare-examples-data assets
fi
