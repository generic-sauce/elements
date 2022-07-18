#!/bin/bash

grep --color=always -nI "$1" $(find -name "*.rs" -a -not -wholename "*/target/*")
