#!/bin/bash

# Requires scripts in 1p22geo/dotfiles added to path

genkey root
gencsr app
signkey app root

cat app.key >>key.pem
cat app.crt root.crt >>certs.pem
