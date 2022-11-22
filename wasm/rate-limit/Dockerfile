# TODO: Change tag from master to version locked once they release a version >0.26.0. This tag
# has an issue with certificate authorities due to missing packages https://github.com/tinygo-org/docker/pull/21
# This will still use tinygo 0.26.0 for now (as of 11/21/2022).
FROM tinygo/tinygo:master AS builder

COPY . .
RUN tinygo build -scheduler=none -target=wasm -o plugin.wasm main.go && ls -al

# Dockerfile for building "compat" variant of Wasm Image Specification.
# https://github.com/solo-io/wasm/blob/master/spec/spec-compat.md
FROM scratch

COPY --from=builder /home/tinygo/plugin.wasm ./plugin.wasm