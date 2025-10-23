FROM python:3.11.7-alpine

WORKDIR /app

RUN apk add vim python3 py3-pip

COPY . /app

RUN rm -rf .env

# install dependencies
RUN apk add --virtual .build-deps gcc musl-dev \
    && pip install --no-cache-dir -r requirements.txt --no-deps \
    && apk del .build-deps && pip install --upgrade --force-reinstall xhtml2pdf[all] 


EXPOSE 8000 