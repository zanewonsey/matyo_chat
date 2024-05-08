# Client
- can run as an application or on the web
- can set a display name
- can connect to a matyo chat server
- can request a room be opened
- can display a list of rooms
- can select and join a room
    - limit to one connection to a room
- can send messages to the chat room that was joined
- will receive messages to the chat room that is connected to
- can leave a chat room

# Server
- listens for new connections
- opens chat rooms at client request
- connects client to requested chat room
- each chat room forwards messages to all clients of that room
- each chat room will announce a client connecting or disconnecting to other connected clients
- will remove client when the client disconnects
    - client may 'soft' disconnect
        - this is when the client leaves the room
    - client may 'hard' disconnect
        - this is when the client closes the application or the connection is otherwise terminated