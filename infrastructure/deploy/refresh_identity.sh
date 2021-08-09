#!/bin/bash

# openssl pkcs12 -export -out identity.pfx -inkey /etc/letsencrypt/archive/generic-sauce.de/privkey2.pem -in /etc/letsencrypt/archive/generic-sauce.de/cert2.pem -certfile /etc/letsencrypt/archive/generic-sauce.de/chain2.pem
openssl pkcs12 -export -out identity.pfx -inkey /etc/letsencrypt/live/generic-sauce.de/privkey.pem -in /etc/letsencrypt/live/generic-sauce.de/cert.pem -certfile /etc/letsencrypt/live/generic-sauce.de/chain.pem
