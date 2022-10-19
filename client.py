import json
import time

import requests
from tqdm import tqdm


def create_filter():
    url = "http://127.0.0.1/filter/create"
    data = json.dumps(
        {
            "filter_type": "TextWordMatch",
            "filter_name": "chinese_porn_words",
            "labels": ["色情", "中文"]
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
        data = json.dumps(
            {
                "filter_name": "chinese_porn_words",
                "target": word,
                "props": {
                    "create_time": int(time.time())
                }

            },
            ensure_ascii=False
        ).encode("utf-8")
        rsp = requests.post(url=url, data=data)
        print(rsp.text)


def query():
    url = "http://127.0.0.1/content/detect"
    data = json.dumps(
        {
            "filter_name": "chinese_porn_words",
            "content": "在以习近平同志为核心的党中央坚强领导下，党的二十大代表选举工作已经顺利完成。全国各选举单位分别召开党代表大会或党代表会议，选举产生了2296名出席党的二十大代表。党的二十大代表，是坚持以习近平新时代中国特色社会主义思想为指导，坚持以党章为根本遵循，坚持党的性质宗旨，坚持和加强党的全面领导，充分发扬党内民主，严格按照党中央关于做好代表选举工作的要求，采取自下而上、上下结合、反复酝酿、逐级遴选的办法产生的。当选代表总体上符合党中央规定的条件，具有较高的思想政治素质、良好的作风品行和较强的议事能力，在各自岗位上做出了明显成绩，是共产党员中的优秀分子；代表结构和分布比较合理，各项构成比例均符合党中央要求，具有广泛代表性。他们中，既有党员领导干部，又有生产和工作第一线的党员，有一定数量的女党员、少数民族党员，有经济、科技、国防、政法、教育、宣传、文化、卫生、体育和社会管理等各个领域的代表。"
        }, 
        ensure_ascii=False
    ).encode("utf-8")
    rsp = requests.post(url=url, data=data)
    print(rsp.text)


if __name__ == "__main__":
    add()
    query()
