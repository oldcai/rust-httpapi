FROM alpine:latest

WORKDIR /web/
COPY ./httpapi /web/

CMD ["/web/httpapi", "0.0.0.0:8000"]
