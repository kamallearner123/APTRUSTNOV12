/*
Create a thread and keep listening to https requests
*/

//mod httpss_handler
//{
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::io::Read;
    use std::io::Write;

    /*
    Function Name: handle_client
    Arguments: TcpStream
    Return Type: ()
    */
    fn handle_client(mut stream: TcpStream) {
        // Read the request from the client in infinite loop
        loop {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 {
                        break;
                    }
                    let request = String::from_utf8_lossy(&buffer[..]);
                    println!("Request: {}", request);
                    let response = "HTTP/1.1 200 OK\r\n\r\n";
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    }


    pub fn get_version() ->String {
        return String::from("0.1");
    }

    /*
    Start a thread and keep listening to https connections and create another thread for each connection
     */
    pub fn start_https_server() {
        let listener = TcpListener::bind("127.0.0.1:3030").expect("####### TCP server is not able to start");
        for stream in listener.incoming() { // Incoming is an iterator
            match stream {
                Ok(stream) => {
                    std::thread::spawn(move || {
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
    
    pub fn create_https_server_thread() -> std::thread::JoinHandle<()> {
        let handle = std::thread::spawn(|| {
            start_https_server();
        });
        return handle;
    }

//}
