#!/usr/bin/env python3

# This is just a basic feasibility verification tool, do
# not use it in a production environment.

import re
import httpx
import flask
from flask import request

FontsApi = 'https://cdn.dnomd343.top/fonts/'


def get_css_content(params: str, ua: str) -> str:
    def url_replace(raw) -> str:
        return f'url({raw[1].replace('https://fonts.gstatic.com/', FontsApi)})'

    url = f'https://fonts.googleapis.com/css?{params}'
    print(f'css upstream url: {url}')
    transport = httpx.HTTPTransport(retries=3, http2=True)
    client = httpx.Client(follow_redirects=True, transport=transport)
    req = client.get(url, timeout=15, headers={'User-Agent': ua})
    if req.status_code not in range(200, 300):
        raise Exception('failed to request css upstream')
    return re.sub(r'url\((\S+)\)', url_replace, req.text)


def get_font_file(url: str, ua: str) -> tuple[str, bytes]:
    url = f'https://fonts.gstatic.com/{url}'
    print(f'font upstream url: {url}')
    client = httpx.Client(follow_redirects=True, http2=True)
    req = client.get(url, timeout=15, headers={'User-Agent': ua})
    if req.status_code not in range(200, 300):
        raise Exception('failed to request fonts upstream')
    return req.headers['content-type'], req.content


app = flask.Flask(__name__)


@app.route('/css', methods=['GET'])
def css_api() -> flask.Response:
    print(f'css origin url: {request.url}')
    items = [f'{x}={y.replace(' ', '+')}' for x, y in request.args.items()]
    raw = get_css_content('&'.join(items), str(request.user_agent))
    resp = flask.make_response(raw)
    resp.mimetype = 'text/css'
    resp.headers.set('Access-Control-Allow-Origin', '*')
    return resp


@app.route('/', defaults={'path': ''}, methods=['GET'])
@app.route('/<path:path>', methods=['GET'])
def font_api(path: str) -> flask.Response:
    print(f'font origin url: {request.url}')
    mime, file = get_font_file(path, str(request.user_agent))  # TODO: args keep here
    resp = flask.make_response(file)
    resp.mimetype = mime
    resp.headers.set('Access-Control-Allow-Origin', '*')
    return resp


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=47114)
