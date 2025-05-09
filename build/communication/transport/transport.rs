/*** Data types ***/

/* Defines the structure of the protocol used for node communication.

Protocol attributes set using this datatype ensure compatability between nodes.
A robust and simple protocol is critical to ensure that nodes running different
versions of Cosmos can contentedly talk to eachother.
*/

/* Defines the structure of a session 

Sessions, generated psudorandomly, are used to uniquely identify connections
between nodes and provide a mechanism to create parallel communication and
connection failover.
  
  A real world example where this would be useful: A node sends a piece
  of data to another node, experiences a network disconnect, and is unable to
  recieve further communication over that line. If it were to restablish a
  connection to the same node, but recieved a different IP address, it would
  seem to be a different node, if one were to only look at the IP address and port.
  However, if the node supplies its session when it connects, the responding node
  will know that the connecting node is the one it previously recieved data from
  and to deliver its queued response to the connected and identified node.
  
Sessions are created in the initialization phase, and destroyed in the termination
phase.
*/
typedef struct session {};

/* Defines the structure of a transport

Transports are used to carry data from one node to another over a physical transmission medium.
Setting up a transport requires the presence of the specified transport, population of required
transport specific configuration data, if required defaults are not present, and the call of the
connect routine (function).
*/
typedef struct transport {
  intptr_t * connection_type; /* input, output, bidirectional */
};

typedef struct connection {
  intmax_t id;
  session session;
};


/*** Methods, Routines, Functions ***/

/*
Initializes a Cosmos node
  Returns a session
*/
session initiallize (transport transport) {}

/*
Connects to a Cosmos node
  Creates and returns a connection
*/
connection connect (session session, transport transport) {}

/*
Sends data to a Cosmos node over the connection provided
*/
void send (connection connection, intptr_t *data, intmax_t data_size) {}

/*
Recieves data from Cosmos over the connection provided
*/
char *data receive (connection connection) {}

/*
Disconnects from a Cosmos node
  Destroys the connection provided
*/
void disconnect (connection connection) {}

/*Terminates the Cosmos process

  * Cleans up the environment
  * Destroys the session
*/
void terminate (session session) {}