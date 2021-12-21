# Chat client
Attempt at making a chat client in rust

# Usage 
run with 
```
cargo run
```
This will launch a client that attempts to connect to a chat server at localhost:8888. 
Users will interact with the client with the command line. The client will first asks the user to input their username for chatting, then every message inputted will be broadcasted to all other users.
Messages from other users will be printed in real-time, along with their names.

# Known Issue
If another user sends a message while the client is typing in STDIN, the message will be printed directly into the terminal, messing up the view. This will not affect the message being sent, only the view.
