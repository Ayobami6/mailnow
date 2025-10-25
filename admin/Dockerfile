FROM python:3.11.7-alpine

WORKDIR /app

RUN apk add vim python3 py3-pip

COPY . /app
COPY requirements.txt /app/requirements.txt

RUN rm -rf .env

# install dependencies
RUN apk add --virtual .build-deps gcc musl-dev postgresql-dev \
    && pip install --no-cache-dir -r requirements.txt \
    && apk del .build-deps


EXPOSE 8000

# CMD ["python", "manage.py", "runserver", "0.0.0.0:8000"] 