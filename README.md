# Description
This is a webservice, to calculate your ape index! Provide height and wingspan. Wingspan is the distance from finger tip to finger tip, while holding your arms horizontally.

https://en.wikipedia.org/wiki/Ape_index

# run the app:
cargo run

# Using the endpoint, height and wingspan are in centimeter:
curl --header 'Content-Type: application/json' http://127.0.0.1:8080/ape -X POST -d '{"height": 200, "wingspan": 200}'

# JSON output:
{"height":200,"wingspan":200,"ape_index":1.0}
