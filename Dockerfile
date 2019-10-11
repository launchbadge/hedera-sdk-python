FROM rustlang/rust:nightly

ENV PYTHONUNBUFFERED=1 \
    DEBIAN_FRONTEND=noninteractive \
    PROTOC_ZIP=protoc-3.3.0-linux-x86_64.zip
        
RUN apt-get update \
        && apt-get install -yq --no-install-recommends \
        python-setuptools

# install pyo3-pack
RUN easy_install pip && pip install pyo3-pack

# install protoc required for library
RUN wget -q https://github.com/google/protobuf/releases/download/v3.3.0/$PROTOC_ZIP && \
    unzip -o $PROTOC_ZIP -d /protoc && \
    ln -s /protoc/bin/protoc /usr/local/bin/protoc && \
    rm -f $PROTOC_ZIP

# pyo3-pack requires newer version of rust
RUN rustup toolchain add nightly-2019-02-07 && rustup default nightly-2019-02-07

ADD . /io/

WORKDIR /io

ENTRYPOINT ["pyo3-pack"]
