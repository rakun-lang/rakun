FROM node:alpine

ARG TARGETARCH
COPY rakun-${TARGETARCH} /bin/rakun

CMD ["rakun"]
