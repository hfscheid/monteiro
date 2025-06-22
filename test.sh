#!usr/bin/env bash
curl -X POST -H "Content-Type: application/octet-stream" \
  --data-binary @montefile.toml \
  localhost:8080
# curl -X POST -F "file=@montefile.toml" localhost:8080
