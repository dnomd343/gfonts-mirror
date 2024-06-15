FROM python:3.12-alpine3.19
RUN pip3 install httpx httpx[http2] flask
COPY . /gfonts-mirror/
CMD ["python3", "/gfonts-mirror/sync.py"]
