FROM elixir:slim

ARG TARGETARCH
COPY rakun-${TARGETARCH} /bin/rakun

CMD ["rakun"]
