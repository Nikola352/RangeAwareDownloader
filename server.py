import hashlib
import random
import time
import traceback
from http.server import BaseHTTPRequestHandler, HTTPServer


class BuggyHandler(BaseHTTPRequestHandler):
    data = random.randbytes(random.randint(512 * 1024, 1024 * 1024))

    def do_GET(self):
        data = self.data
        if self.headers.get("Range"):
            try:
                start, end = self.headers["Range"].split("=")[1].split("-")
                start = int(start) if start else 0
                end = int(end) if end else len(self.data)
                data = data[start:end]
            except Exception as err:
                response = traceback.format_exc()

                self.send_error(400, f"Invalid range: {err}")
                self.send_header("Content-Type", "text/plain")
                self.send_header("Content-Length", str(len(response)))
                self.send_header("Connection", "close")
                self.end_headers()
                self.wfile.write(response.encode())
                return

        self.send_response(200 if len(data) == len(self.data) else 206)
        self.send_header("Content-Type", "application/octet-stream")
        self.send_header("Content-Length", str(len(data)))
        self.send_header("Connection", "close")
        self.end_headers()

        # Making the life harder.
        time.sleep(random.randint(1, 100) * 0.01)
        if len(data) > 64 * 1024:
            data = data[:random.randint(64 * 1024, len(data))]

        self.wfile.write(data)


def run_server():
    host = "127.0.0.1"
    port = 8080

    print("Length of data:", len(BuggyHandler.data))
    print("SHA-256 hash of the data:", hashlib.sha256(BuggyHandler.data).hexdigest())

    httpd = HTTPServer((host, port), BuggyHandler, bind_and_activate=True)
    httpd.allow_reuse_address = True
    print(f"Starting HTTP server on port {host}:{port}")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nStopping server...")
        httpd.server_close()


if __name__ == "__main__":
    run_server()