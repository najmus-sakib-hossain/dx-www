import http.server
import socketserver

PORT = 8000

Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map['.wasm'] = 'application/wasm'

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print(f"✨ dx-www Demo Server Running")
    print(f"🌐 http://localhost:{PORT}/demo.html")
    print(f"⚡ HTIP Engine Ready")
    print(f"\nPress Ctrl+C to stop")
    httpd.serve_forever()
