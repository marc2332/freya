import errno
import functools
import http.server
import os
import sys

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
