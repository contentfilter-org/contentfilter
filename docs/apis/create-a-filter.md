# Create A Filter
Used to create a filter for storing sieves and detecting content.

**URL** : `/filter/create`

**Method** : `POST`

**Auth required** : NO

**Data constraints**
```json
{
    "filter_type": "Filter Type Name",
    "filter_name": "Filter Name",
    "labels": ["Label1", "Label2", "..."]
}
```

**Data example**
```json
{
    "filter_type": "TextWordMatch",
    "filter_name": "porn_words",
    "labels": ["english", "porn"]
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
**Condition** : If `filter_type` is wrong or `filter_name` exists, or labels is not a string array.

**Code** : `200 OK`

**Content** :

```json
{
    "status": "filter exists error",
    "time": 0.000841354
}
```