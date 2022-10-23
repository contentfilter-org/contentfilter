# Detect Content
Used to detect content through sieves in a filter.

**URL** : `/detect`

**Method** : `POST`

**Auth required** : NO

**Data constraints**
```json
{
    "filter_name": "Filter Name",
    "content": "Content To Be Detected (Text, Image, or Audio)"
}
```

**Data example**
```json
{
    "filter_name": "porn_words",
    "content": "This is an example sentence, which is used to test porn text filter."
}
```

## Success Response
**Code** : `200 OK`

**Content example**

```json
{
    "count": 1,
    "hits": [
        {
            "create_time": 1666492193000,
            "dr_dhash": 18446744073709551615,
            "dr_md5": "89e55d4f580dd044088b9a003110b37a",
            "id": 64237,
            "property_map": "{\"activated_scenes\": \"kids\"}",
            "similarity": 1.0,
            "target": "porn"
        }
    ],
    "status": "success",
    "time": 0.003119765
}
```

## Error Response
**Condition** : If `filter_name` does not exists, or `content` is wrong.

**Code** : `200 OK`

**Content** :

```json
{
    "count": 0,
    "hits": [],
    "status": "filter not found error",
    "time": 0.003119765
}
```
