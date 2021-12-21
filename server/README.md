# Chat thing
Attempt at making a chat server in rust

# Usage 
run with 
```
cargo run
```
This will start a chat server at localhost:8888. 

# Protocol
The server treats the first message sent to the server as the username. After the initial message, the server will broadcast messages sent to the server to all other connected clients.
