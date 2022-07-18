#!/bin/bash

grep "$1" $(find -name "*.rs")
