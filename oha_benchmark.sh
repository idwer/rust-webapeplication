#/usr/bin/env bash
oha -n 1000 -c 1000 -m POST -T 'application/json' -D post_data.txt http://127.0.0.1:8080/ape
