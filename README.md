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

Add your access_keys to `./access_keys` or don't add any, if you don't want to use authorization.

Edit `Rocket.toml` to set port and ip.

### Running

```shell
cargo run -r
```

## Usage

### Create new alias

#### Request

```http request
POST /post HTTP/1.1
```

#### Request body

```json
{
    "url": "<URL_TO_BE_ALIASED>",
    "alias": "<ALIAS_URI>",      // If not provided, UUID will be generated
    "access_key": "<ACCESS_KEY>" // May not be provided, if no ./access_keys file
}
```

### Use alias

```http request
GET /<ALIAS> HTTP/1.1
```

```http request
HTTP/1.1 303 See Other
location: <URL>
```

### Alias for `/`

Aliases for root is declared in `src/main.rs` file in `INDEX_REDIRECT` const.

## License

The project is licensed under the terms of the [MIT license](./LICENSE).