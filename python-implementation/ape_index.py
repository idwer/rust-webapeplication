from flask import Flask, request
import json

from ape_index_lib import ApeIndex

app = Flask(__name__)


@app.route('/ape', methods=['POST'])
def endpoint_post() -> str:
    ape = ApeIndex

    request_json = request.get_json()

    height: int = int(json.dumps(request_json['height']))
    wingspan: int = int(json.dumps(request_json['wingspan']))

    ape_index: float = ape.get_ape_index(ape, height, wingspan)

    # return 7 decimals, like Rust returns
    ape_index = float("{:.7f}".format(ape_index))

    return json.dumps({'height': height, 'wingspan': wingspan, 'ape_index': ape_index})


if __name__ == "__main__":
    app.run()