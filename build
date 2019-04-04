#!/bin/sh

echo "Buidling Hedera-Python SDK..."
docker build . --tag hedera/pyo3-pack
docker run --rm -v $(pwd):/io hedera/pyo3-pack build --release
