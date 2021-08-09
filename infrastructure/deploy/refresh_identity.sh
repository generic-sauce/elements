#!/bin/bash

openssl pkcs12 -export -out identity.pfx -inkey /etc/letsencrypt/live/generic-sauce.de/privkey.pem -in /etc/letsencrypt/live/generic-sauce.de/cert.pem -certfile /etc/letsencrypt/live/generic-sauce.de/chain.pem

cp identity.pfx ./game_server/identity.pfx
cp identity.pfx ./master_server/identity.pfx
