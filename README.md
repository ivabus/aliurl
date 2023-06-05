# aliurl

> ALIaser for URLs

Small http service to create aliases for URLs.

## Installation

```shell
git clone https://github.com/ivabus/aliurl
cd aliurl
cargo b -r
```

### Configuration

Add your access_keys (separating by newline) to `./access_keys` or don't add any, if you don't want to use authorization.

Edit `Rocket.toml` to set port and ip.

### Running

```shell
cargo run -r
```

## API

### Create new alias

#### Request

```http request
POST /api/create_alias HTTP/1.1
```

#### Request body

```json
{
    "url": "<URL_TO_BE_ALIASED>",
    "alias": "<ALIAS_URI>",      // If not provided, UUID will be generated
    "access_key": "<ACCESS_KEY>" // May not be provided, if no ./access_keys file
    "redirect_with_ad": "<BOOL>" //May not be provided, if provided will use ./redirect.html
}
```

### Get all aliases

#### Request

```http request
POST /api/create_alias HTTP/1.1
```

#### Request body

```json
{
    "access_key": "<ACCESS_KEY>" // May not be provided, if no ./access_keys file
}
```

#### Response body

```json
[
    {
        "alias": "alias_without_ad",
        "url": "https://example.com"
    },
    {
        "alias": "alias_with_ad",
        "redirect_with_ad": true,
        "url": "https://example.com"
    }
]
```

### Remove alias

Removes all alias with provided name.

#### Request

```http request
POST /api/remove_alias HTTP/1.1
```

#### Request body

```json
{
    "alias": "<ALIAS>",
    "access_key": "<ACCESS_KEY>" // May not be provided, if no ./access_keys file
}
```

#### Response

##### Alias(es) removed

```json
[
    {
        "alias": "alias",
        "url": "https://example.com"
    },
    {
        "alias": "alias",
        "redirect_with_ad": true,
        "url": "https://another.com"
    }
]
```

##### No aliases found

```json
[]
```

### Use alias

#### Request

```http request
GET /<ALIAS> HTTP/1.1
```
#### Response

```http request
HTTP/1.1 303 See Other
location: <URL>
```

### Alias for `/`

Aliases for root is declared in `src/main.rs` file in `INDEX_REDIRECT` const.

## Redirect with "ad"

See `./redirect.html.example` to understand what's going on.

## License

The project is licensed under the terms of the [MIT license](./LICENSE).