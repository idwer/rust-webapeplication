# run the app:
cargo run

# test the endpoint, height and wingspan are in centimeter:
curl http://127.0.0.1:8000/api/ape -X POST -d '{"height": 200, "wingspan": 200}'