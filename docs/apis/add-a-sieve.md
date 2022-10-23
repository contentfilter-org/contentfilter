# Add A Sieve
Used to add a sieve into an existing filter for detecting content.

**URL** : `/sieve/add`

**Method** : `POST`

**Auth required** : NO

**Data constraints**
```json
{
    "filter_name": "Filter Name",
    "target": "Sieve Target (words, images, or audio)",
    "property_map": {
        "key 1": "value1",
        "key 2": "value2",
        "...": "..."
    }

}
```

**Data example**
```json
{
    "filter_name": "porn_words",
    "target": "porn",
    "property_map": {
        "activated_scenes": "kids"
    }

}
```

## Success Response
**Code** : `200 OK`

**Content example**

```json
{
    "status": "success",
    "time": 0.000841354
}
```

## Error Response
**Condition** : If `target` is wrong or `filter_name` does not exists, or `property_map` is wrong.

**Code** : `200 OK`

**Content** :

```json
{
    "status": "sieve add error",
    "time": 0.000841354
}
```
