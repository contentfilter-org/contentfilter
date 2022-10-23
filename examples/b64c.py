import base64
import json

if __name__ == "__main__":
    with open("./tests/resources/dhash/grape1.jpeg", 'rb') as fp:
    # with open("../../Downloads/1.jpg", 'rb') as fp:
        image_read = fp.read() 
        image_64_encode = base64.standard_b64encode(image_read)
    with open("./data/b64.json", "w") as fp:
        request = {
            "filter_name": "grape1",
            "target": str(image_64_encode, "utf8"),
            "property_map": {}
        }
        fp.write(json.dumps(request, ensure_ascii=False, indent=4))
