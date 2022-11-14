# Description
This is a webservice, to calculate your ape index! Provide height and wingspan, in centimeters. Height and wingspan input must be between 1 and 300.

Wingspan is the distance measured from finger tip to finger tip, while holding your arms horizontally.


https://en.wikipedia.org/wiki/Ape_index

# Run the app:
```cargo run```

# Using the endpoint:
```curl --header 'Content-Type: application/json' http://127.0.0.1:8080/ape -X POST -d '{"height": 200, "wingspan": 200}'```

The endpoint will restrict input to reasonable values. Input, for height and wingspan, less than 1 or above 300 will be rejected.

# JSON output:
```{"height":200,"wingspan":200,"ape_index":1.0}```
