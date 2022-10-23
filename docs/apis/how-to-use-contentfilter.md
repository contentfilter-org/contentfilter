# How to Use Contentfilter
These are a list of concepts or interfaces for helping to use Contentfilter.
## 1 Filter Type
<table>
  <tr>
    <th>Media Type</th>
    <th>Filter Type</th>
    <th>Description</th>
    <th>Status</th>
  </tr>
  <tr>
    <td rowspan="2">Text</td>
    <td>TextWordMatch</td>
    <td></td>
    <td>✅</td>
  </tr>
  <tr>
    <td>TextSimilarityMatch</td>
    <td></td>
    <td>🚧</td>
  </tr>
  <tr>
    <td rowspan="3">Image</td>
    <td>ImageDhashMatch</td>
    <td></td>
    <td>✅</td>
  </tr>
  <tr>
    <td>ImageSimilarityMatch</td>
    <td></td>
    <td>🚧</td>
  </tr>
  <tr>
    <td>ImageFaceSimilarityMatch</td>
    <td></td>
    <td>🚧</td>
  </tr>
</table>

## 2 REST APIs
### 2.1 Meta APIs
* [<font color="gray">Get Service Status</font>](#) : `GET /status`

### 2.2 Filter APIs
* [Create A Filter](./create-a-filter.md) : `POST /filter/create`
* [<font color="gray">Delete A Filter</font>](#) : `POST /filter/delete`
* [<font color="gray">Update A Filter</font>](#) : `POST /filter/update`
* [<font color="gray">Describe A Filter</font>](#) : `POST /filter/describe`
* [<font color="gray">List All Filters</font>](#) : `POST /filter/list`
* [<font color="gray">Dump A Filter</font>](#) : `POST /filter/dump`

### 2.1 Sieve APIs
* [Add A Sieve](./add-a-sieve.md) : `POST /sieve/add`
* [<font color="gray">Delete A Sieve</font>](#) : `POST /sieve/delete`
* [<font color="gray">Update A Sieve</font>](#) : `POST /sieve/update`
* [<font color="gray">Describe A Sieve</font>](#) : `POST /sieve/describe`

### 2.4 Detection APIs
* [Detect Content](./detect-content.md) : `POST /detect`
