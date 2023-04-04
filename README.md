# Product Information Extractor

This Rust program extracts product information from any website using a web scraper and the ChatGPT API. It fetches the HTML content from a given URL and extracts relevant texts and images. The program then generates a prompt to call the ChatGPT API, which helps in extracting the required product information. The extracted information is returned as a JSON object with a specific schema.




## Prerequisite


1. Install Rust programming language: https://www.rust-lang.org/tools/install

2. Ensure yout system has build extras
    ```bash
    sudo apt install build-essential
    ```

3. install libssl-dev
    ```bash
    sudo apt-get -y install libssl-dev
    ```

4. Install the newest version of protobuff
    ```bash
    ARCH="linux-x86_64" && \
    VERSION="22.2" && \
    curl -OL "https://github.com/protocolbuffers/protobuf/releases/download/v$VERSION/protoc-$VERSION-$ARCH.zip" && \
    sudo unzip -o "protoc-$VERSION-$ARCH.zip" bin/protoc "include/*" -d /usr/local && \
    rm -f "protoc-$VERSION-$ARCH.zip"
    ```


## Installation Guide
1. Clone the repository
```bash
git clone https://github.com/hansaskov/extract-product
```
```bash
cd extract-product
```
2. Set up the environment variable for OpenAI API key in Secrets.toml

## API Usage
### Request
Make a GET request to the /extract endpoint with the following query parameter:
- `url` (required): The URL of the website from which the product information is to be extracted.
## Example
### Request
```sh
curl "http://localhost:3000/extract?url=https://www.example.com/product"
```
### Response
```json
{
  "name": "Example Product",
  "price": "$29.99",
  "description": "This is an example product description.",
  "image_url": "https://www.example.com/images/product.jpg"
}

```


