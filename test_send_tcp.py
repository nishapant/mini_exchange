import socket

HOST = "192.168.50.106"  # The server's hostname or IP address
PORT = 8082  # The port used by the server\

print("hello")

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    print("we are connected")
    s.sendall(b"Hello, world")
    data = s.recv(1024)

print(f"Received {data!r}")