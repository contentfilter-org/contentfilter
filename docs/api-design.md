# 接口设计
## 1 创建数据
- 创建过滤器

    过滤器（`filter`）是一个用于内容过滤的库，比如敏感词库、敏感图库等。

    - 请求
        > POST /filter/create
        ```json
        {
            "filter_type": "TEXT_WORD",
            "filter_name": "chinese_porn_words",
            "labels": ["色情", "中文"]
        }
        ```
    - 响应
        ```json
        {
            "success": true,
            "error": ""
        }
        ```

- 添加筛子
    筛子（`sieve`）是过滤器中的一个个基本元素，主要用于内容的过滤，比如敏感词库中的某一个敏感词、或者敏感图库中的某一个敏感图片。一个过滤器是由多个筛子组成的。

    - 请求
        > POST /sieve/add
        ```json
        {
            "filter_name": "chinese_porn_words",
            "target": "大眾色情成人網",
            "props": {
                "附加属性1": "附加属性1的值",
                "附加属性2": "附加属性2的值"
            }

        }
        ```

    - 响应
        ```json
        {
            "success": true,
            "error": ""
        }
        ```

## 2 内容过滤
当过滤器创建完成之后，我们就可以利用过滤器来过滤我们想要查询的内容（`content`）。

- 请求

    > POST /content/detect
    ```json
    {
        "filter_name": "chinese_porn_words",
        "content": "这是一条很长的待检测的文本或者图片编码！"
    }
    ```

- 响应
    ```json
    {
        "filter_name": "chinese_porn_words",
        "content": "这是一条很长的待检测的文本或者图片编码！",
        "success": true,
        "error": "",
        "hits": [
            {
                "target": "图片编码", 
                "socre": 1.0,
                "_props": {
                    "_create_time": 189829272922,
                    "_md5": "688a8473855d671059a970771d90f478",
                    "props": {
                        "附加属性1": "附加属性1的值",
                    }
                }
            }
        ]
    }
    ```
