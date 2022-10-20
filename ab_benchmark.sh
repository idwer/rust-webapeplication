#/usr/bin/env bash
ab -n 100000 -c 1000 -T 'application/json' -p post_data.txt http://127.0.0.1:8000/ape
