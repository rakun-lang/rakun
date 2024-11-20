FROM scratch

ARG TARGETARCH
COPY rakun-${TARGETARCH} /bin/rakun

CMD ["rakun"]
