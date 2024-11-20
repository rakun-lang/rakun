FROM node:latest

ARG TARGETARCH
COPY rakun-${TARGETARCH} /bin/rakun

CMD ["rakun"]
