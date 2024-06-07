import socket

def client():
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    host = socket.gethostname()
    port = 12345

    client_socket.connect((host, port))

    message = client_socket.recv(1024)
    print("Received from server:", data.encode())

    client_socket.close()

if __name__ == "__main__":
    client()
