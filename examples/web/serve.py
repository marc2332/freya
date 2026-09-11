import errno
import functools
import http.server
import os
import re
import sys
import urllib.request

INDEX = """<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1, user-scalable=no" />
    <title>Freya</title>
    <style>
      html,
      body {{
        margin: 0;
        height: 100dvh;
        overflow: hidden;
        background: #ffffff;
      }}

      #canvas {{
        display: block;
        width: 100dvw;
        height: 100dvh;
        outline: none;
        user-select: none;
        -webkit-user-select: none;
        -webkit-user-drag: none;
        -webkit-touch-callout: none;
        touch-action: none;
      }}
    </style>
  </head>
  <body>
    <canvas id="canvas" draggable="false"></canvas>
    <script>
      var Module = {{ canvas: document.getElementById("canvas") }};
    </script>
    <script src="{name}"></script>
  </body>
</html>
"""


class Handler(http.server.SimpleHTTPRequestHandler):
    extensions_map = {
        **http.server.SimpleHTTPRequestHandler.extensions_map,
        ".wasm": "application/wasm",
    }

    proxied_origin = None

    def do_GET(self):
        if self.path.startswith("/proxy/"):
            url = re.sub(r"^(https?):/+", r"\1://", self.path[len("/proxy/") :])
            origin = re.match(r"^https?://[^/]+", url)
            if origin:
                Handler.proxied_origin = origin[0]
            self.proxy(url)
        elif Handler.proxied_origin and not os.path.exists(self.translate_path(self.path)):
            # Serve missing files from the last proxied origin.
            self.proxy(Handler.proxied_origin + self.path)
        else:
            super().do_GET()

    def proxy(self, url):
        request = urllib.request.Request(
            url,
            headers={
                "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:140.0) Gecko/20100101 Firefox/140.0",
                "Accept": "*/*",
            },
        )
        try:
            with urllib.request.urlopen(request, timeout=30) as response:
                body = response.read()
                content_type = response.headers.get("Content-Type", "application/octet-stream")
        except Exception as error:
            self.send_error(502, f"Proxy request failed: {error}")
            return
        origin = re.match(r"^https?://[^/]+", url)
        if origin and ("text/html" in content_type or "text/css" in content_type):
            body = rewrite_urls(body, origin[0].encode())
        self.send_response(200)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)


def rewrite_urls(body, origin):
    """Rewrite links and assets to go through the proxy."""
    body = re.sub(rb'(href|src|action)=(["\'])(https?:)//', rb"\1=\2/proxy/\3//", body)
    body = re.sub(rb'(href|src|action)=(["\'])//', rb"\1=\2/proxy/https://", body)
    body = re.sub(rb'(href|src|action)=(["\'])/(?!/|proxy/)', rb"\1=\2/proxy/" + origin + rb"/", body)
    body = re.sub(rb'url\((["\']?)(https?:)//', rb"url(\1/proxy/\2//", body)
    body = re.sub(rb'url\((["\']?)//', rb"url(\1/proxy/https://", body)
    body = re.sub(rb'url\((["\']?)/(?!/|proxy/)', rb"url(\1/proxy/" + origin + rb"/", body)
    return body


def main() -> int:
    if len(sys.argv) < 2:
        print("expected the path of the built artifact", file=sys.stderr)
        return 1

    artifact = sys.argv[1]
    directory = os.path.dirname(artifact)
    port = int(os.environ.get("FREYA_WEB_PORT", "8771"))

    with open(os.path.join(directory, "index.html"), "w") as index:
        index.write(INDEX.format(name=os.path.basename(artifact)))

    handler = functools.partial(Handler, directory=directory)

    try:
        server = http.server.ThreadingHTTPServer(("0.0.0.0", port), handler)
    except OSError as error:
        if error.errno == errno.EADDRINUSE:
            print(
                f"Port {port} is already in use, close the other server or set FREYA_WEB_PORT.",
                file=sys.stderr,
            )
            return 1
        raise

    print(f"Serving http://localhost:{port}", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        return 0
    return 0


if __name__ == "__main__":
    sys.exit(main())
