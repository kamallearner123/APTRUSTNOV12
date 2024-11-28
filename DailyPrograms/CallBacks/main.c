#include <stdio.h>
#include <stdlib.h>
#include <signal.h>
#include <sys/socket.h>
/*
create a thread to listen and read the data on tcp 
When message "QUIT" is recieved then call exit_callback()
*/
void http_callback(int *ptr) {
    printf("http_callback\n");
    printf("Counter: %d\n", *ptr);
    printf("Exiting...\n");
    exit(0);
}

void ftp_callback(int *ptr) {
    printf("ftp_callback\n");
    printf("Counter: %d\n", *ptr);
    printf("Exiting...\n");
    exit(0);
}

// create_tcp_server_socket
int create_tcp_server_socket() {
    // Create a socket
    int socket = socket(AF_INET, SOCK_STREAM, 0);
    if (socket == -1) {
        perror("Error creating socket");
        return -1;
    }

    // Bind the socket to an address
    struct sockaddr_in server_address;
    server_address.sin_family = AF_INET;
    server_address.sin_port = htons(9002);
    server_address.sin_addr.s_addr = INADDR_ANY;
    int bind_result = bind(socket, (struct sockaddr *)&server_address, sizeof(server_address));
    if (bind_result == -1) {
        perror("Error binding socket");
        return -1;
    }

    // Listen for incoming connections
    int listen_result = listen(socket, 5);
    if (listen_result == -1) {
        perror("Error listening on socket");
        return -1;
    }

    return socket;
}


// Tcp server thread function to read the data and call exit_callback() when "QUIT" is recieved
void create_tcp_server(void *arg) {
    strcut handle *ptr = arg;
    void (*accept_callback)(int) = arg.accept_callback;
    // Create a tcp server
    int socket = create_tcp_server_socket();
    if (socket == -1) {
        printf("Error creating tcp server\n");
        return;
    }

    // Listen for incoming connections
    int client_socket = accept_client_connection(socket);
    if (client_socket == -1) {
        printf("Error accepting client connection\n");
        return;
    }

    // Read data from client in while loop  
    while (1) {
        char buffer[1024];
        int bytes_read = read_data(client_socket, buffer, sizeof(buffer));
        if (bytes_read == -1) {
            printf("Error reading data from client\n");
            break;
        }

        // Check if message is "QUIT"
        if (strcmp(buffer, "HTTP:Request") == 0) {
            http_callback(arg.ptr);
            break;
        } else if (strcmp(buffer, "FTP") == 0) {
            ftp_callback(arg.ptr);
            break;
        }
    }

}

// Initialize the service accept function pointer a argument
int init_service (struct handle *h) {
    // craete thread and run create_tcp_server
    pthread_t tcp_server_thread;
    tcp_server_thread = pthread_create(&tcp_server_thread, NULL, create_tcp_server, h);
    pthread_join(tcp_server_thread, NULL);
}

struct handle {
    void (*http_callback)(int); // address size
    void (*ftp_callback)(int);
    int *ptr;
}


int counter = 0;

int main() {
    // crate thread and run create_tcp_server
    struct handle *h = malloc(sizeof(struct handle));
    h->http_callback = http_callback;
    h->ftp_callback = ftp_callback;
    h->ptr = &counter;
    init_service(h);

    while (1) {
        // Infinite loop to keep the program running
        pause(); // Wait for signals
    }

    return 0;
}
