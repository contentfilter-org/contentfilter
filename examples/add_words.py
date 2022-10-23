import json
import time

import requests
from tqdm import tqdm


def create_filter():
    url = "http://127.0.0.1/filter/create"
    data = json.dumps(
        {
            "filter_type": "TextWordMatch",
            "filter_name": "sensitive_words",
            "labels": ["色情", "中文", "政治"]
        }, 
        ensure_ascii=False
    ).encode("utf-8")
    rsp = requests.post(url=url, data=data)
    print(rsp.text)


def add():
    with open("./data/words.txt", "r") as fp:
        lines = fp.readlines()
    words = list(map(lambda s: s.strip(), lines))
    url = "http://127.0.0.1/sieve/add"
    for word in tqdm(words, total=len(words)):
        word = word.strip()
        if word == "":
            continue
        data = json.dumps(
            {
                "filter_name": "sensitive_words",
                "target": word,
                "property_map": {}

            },
            ensure_ascii=False
        ).encode("utf-8")
        rsp = requests.post(url=url, data=data)
        # print(rsp.text)


if __name__ == "__main__":
    create_filter()
    add()
