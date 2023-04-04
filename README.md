# Installation

Ensure yout system has build extras
```bash
sudo apt install build-essential
```

install libssl-dev
```bash
sudo apt-get -y install libssl-dev
```


Install the newest version of protobuff
```bash
ARCH="linux-x86_64" && \
VERSION="22.2" && \
curl -OL "https://github.com/protocolbuffers/protobuf/releases/download/v$VERSION/protoc-$VERSION-$ARCH.zip" && \
sudo unzip -o "protoc-$VERSION-$ARCH.zip" bin/protoc "include/*" -d /usr/local && \
rm -f "protoc-$VERSION-$ARCH.zip"
```


