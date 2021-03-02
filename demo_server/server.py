# -*- coding: utf-8 -*-
import http.server
import socketserver
import socket
import os
FILES_LOCATION='web_app'
PORT = 8080
web_dir = os.path.join(os.path.dirname(__file__), FILES_LOCATION)
os.chdir(web_dir)

class HttpRequestHandler(http.server.SimpleHTTPRequestHandler):
    extensions_map = {
        '': 'application/octet-stream',
        '.manifest': 'text/cache-manifest',
        '.html': 'text/html',
        '.png': 'image/png',
        '.jpg': 'image/jpg',
        '.svg':	'image/svg+xml',
        '.css':	'text/css',
        '.js':'application/x-javascript',
        '.wasm': 'application/wasm',
        '.json': 'application/json',
        '.xml': 'application/xml',
    }

with socketserver.TCPServer(("localhost", PORT), HttpRequestHandler) as httpd:
    try:
        print(f"serving {str(web_dir)} at http://localhost:{PORT}")
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("[!] Keyboard Interrupted!")
        httpd.server_close()
        httpd.shutdown()

