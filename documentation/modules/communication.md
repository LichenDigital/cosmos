< **[Back to documentation main](../documentation.md)**
___

# *Communication*

## Networking

The network module transmits data over a variety of software and hardware protocols. Software wise, it handles both high bandwidth protocols like `https`, `udp` and `tcp` as well as low bandwidth protocols lke `i2c` and `serial`. Hardware wise, cosmos has the capability to transmit data over typical means like `wifi`, `bluetooth`, and `celular`. Initially the networking protocol will be built with diffrent adapters like `zmq` or something similar to take care of a lot of the hard low-level work.

## Sub-modules

- **transport** - Handles different transports
- **filter** - Filters network traffic - utilizes the event module
- **log** - Logs network traffice - utilizes the log module
- **address** - Handles the addressing of cosmos nodes and other external nodes

## Commands for a transport

- **create**, **crt**: Creates a resource
- **get**: Gets a resource
- **update**, **upt**: Updates a resource
- **delete**, **dlt**: Deletes a resource
- **duplicate**: **dup**: Duplicates a resource (uses get and create)
- **connect**, **con**: Connects a resource to another resource
- **disconnect**, **dsc**: Disconnects a resource from another resource
- **send**: **snd**: Sends a resource
- **receive**: **rcv**: Recieves a resource



Built In Character Command Protocol Formatting (extensible):
	
	// This protocol formatting is a base for protocol interpretations.
	// This fromatting definition is used by the Cosmos interpreter. 
	// If an input to the interpreter matches a defined protocol formatting
	// definintion, than it is translated into native c function calls to the
	// function that is being executed

// White space doesn't matter in between the keyword separator and the value, and the indicator of the end
// of the value and the begining of the keyword
connection:ssh,payload: -p 2200 cosmos@cosmos.com

// Or there can be as much as you want, because it's ignored
// If you notice here, formatting can be entirely different for each keyword, value pair
module:    ssh,    payload:-p 2200 cosmos@cosmos.com

// Just keep this in mind: if the value you are providing requires white space
// than you must enclose it in a container of some sort. This provides the interpreter
// something to use when it defines the difference between whitespace employed for formatting purposes,
// and space that has significance, like space in the payload or module name
module:ssh, payload: "     -p 2200 cosmos@cosmos.com"  // Notice that in this command the payload is encompassed
	// in a pair of double quotes which tells the compiler to pay attention to the space

// 
"ssh", payload:   "-p 2200 cosmos@cosmos.com  ", 
'ssh', '-p 2200 cosmos@cosmos.com'

module= ssh, payload= -p 2200 cosmos@cosmos.com
module=ssh, payload=-p 2200 cosmos@cosmos.com
module='ssh', payload='-p 2200 cosmos@cosmos.com'
module="ssh", payload="-p 2200 cosmos@cosmos.com"

ssh -p 2200 cosmos@cosmos.com 


## Messaging

Message supports sending data (be it text or other forms of data) through different forms of messaging methods. It supports SMS, email, and other message protocols. We'll have to figure out a module for voice communication, and how to do things like text over voice, and voice over text etc.



## Eventual Cosmos Transport Layer

It is an eventual goal to have cosmos include a built in transport layer. This feature would include the following goals:

**Core Features**
-High Performance: Optimized for low latency and high throughput, capable of handling massive volumes of messages efficiently.
-Scalability: Easily scales horizontally and vertically to accommodate growing data volumes and connection counts without significant performance degradation.
-Reliability: Guarantees message delivery through mechanisms like message acknowledgment, retries, and persistent storage for critical data.
-Fault Tolerance: Implements strategies for automatic recovery from failures, ensuring continuous operation and data integrity.
-Security: Incorporates built-in encryption, authentication, and authorization to secure data in transit and control access.

**Advanced Messaging Patterns**
-Flexible Messaging Patterns: Supports a variety of messaging patterns, including request-reply, publish-subscribe, fan-out, and more, to accommodate different communication needs.
-Dynamic Routing and Load Balancing: Dynamically routes messages based on load, priority, or other criteria to optimize resource utilization and response times.
-Stream Processing: Facilitates real-time processing of streaming data, enabling complex event processing, windowing, and state management within the transport layer.

**Interoperability and Integration**
-Protocol Agnostic: Capable of supporting multiple transport protocols (TCP, UDP, IPC, WebSocket, etc.) and data formats (binary, JSON, Protobuf, etc.) to ensure broad compatibility and integration capabilities.
-Service Discovery: Includes mechanisms for automatic service discovery and registration, simplifying the configuration and scaling of distributed systems.
-Backward Compatibility: Maintains compatibility with existing standards and protocols where possible, easing migration and integration with legacy systems.

**Developer-Friendly**
-Comprehensive APIs: Provides well-documented, easy-to-use APIs for various programming languages, enabling developers to quickly implement and customize transport functionalities.
-Monitoring and Management: Features built-in tools for monitoring performance, diagnosing issues, and managing configurations, aiding in maintenance and optimization.
-Extensibility: Designed with extensibility in mind, allowing for custom plugins, modules, or extensions to add new functionalities or integrate with external systems.

**Deployment and Operation**
-Containerization and Orchestration Support: Ensures compatibility with containerized environments and orchestration tools like Docker and Kubernetes for easy deployment and scaling.
-Cross-Platform: Works across different operating systems and environments, ensuring wide applicability and flexibility in deployment options.