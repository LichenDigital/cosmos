#if defined (__linux__) || defined (__linux) || defined (__gnu_linux)

#include "../src/linux/ipc.h"
#include "stdint.h"

#elif defined (_unix)  || defined (__unix__)

#elif defined (__APPLE__)  || defined (__MACH__)

#elif defined (_WIN32) || defined (__WIN32) || defined (__WIN32__)

#elif defined (_WIN64) || defined (__WIN64) || defined (__WIN64__)

#elif defined (__AVR_ATtiny85__)

#elif defined (__AVR_ATmega168__)

#elif defined (__AVR_ATmedga328P__)

#elif defined (__AVR_ATmega1280__)

#elif defined (__AVR_ATmega2560__)

#elif defined (__AVR_ATmega32U4)

#elif defined (__SAM3x8e__)

#elif defined (__CYGWWIN__)

#elif defined (__CYGWIN32__)

#elif defined (__sun) || defined (__SVR4) || defined (__sun__)  || defined (__SUNOS) || defined (__svr4__)

#elif defined (__hpux)

#elif defined (_AIX)

#elif defined (i386) || (__i386) || (__i386__)

#elif defined (__amd64__) || (__amd64) || (__x86_64__) || (__x86_64)

#elif defined (__arm)

#elif defined (__aarch64__)

#elif defined 

#else

#endif


/*** Data types ***/

/* Defines the structure of the protocol used for node communication.

Protocol attributes set using this datatype ensure compatability between nodes.
A robust and simple protocol is critical to ensure that nodes running different
versions of Cosmos can contentedly talk to eachother.
*/
typedef struct protocol {};

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