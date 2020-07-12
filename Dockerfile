FROM alpine:latest

WORKDIR /web/
COPY ./httpapi /web/

CMD ["./httpapi", "0.0.0.0:8000"]
