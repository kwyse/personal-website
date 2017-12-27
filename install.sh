#!/bin/bash

HUGO_VERSION=0.31.1

if [[ ! -f /data/bin/hugo ]]; then
  cd /tmp
  wget https://github.com/gohugoio/hugo/releases/download/v${HUGO_VERSION}/hugo_${HUGO_VERSION}_Linux-64bit.tar.gz
  tar -xzf hugo_${HUGO_VERSION}_Linux-64bit.tar.gz
  mv hugo /data/bin/hugo
  cd -
  rm -rf /tmp/*
fi
