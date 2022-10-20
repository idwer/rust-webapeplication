#/usr/bin/env bash
ab -n 1000 -c 1000 -m POST -T 'application/json' -p post_data.txt http://127.0.0.1:8080/ape
