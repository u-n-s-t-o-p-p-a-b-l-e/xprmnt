import socket

def server():
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    host = socket.gethostname()
    port = 12345

    server_socket.bind((host, port))

    server_socket.listen(5)

    print("Server listening on {}:{}".format(host, port))

    while True:
        client_socket, addr = server_socket.accept()
        print('Got connection from', addr)

        data = client_socket.recv(1024)
        print("Received from client:", data.decode())

        client_socket.sendall(b"Server received your message. Thanks!")

        client_socket.close()

if __name__ == "__main__":
    server()
