< **[Back to documentation main](../documentation.md)**
___

# *Network*

The network module transmits data over a variety of software and hardware protocols. Software wise, it handles both high bandwidth protocols like `https` and `tcp` as well as low bandwidth protocols lke `i2c` and `serial`. Hardware wise, cosmos has the capability to transmit data over typical means like `wifi`, `lan`, and `wan` as well as less typical means like `light`

// Initialize a Cosmos connection
function connect (node, transport) {
	
}

// Destroy a Cosmos connection
function disconnect () {
	
}

// Sends data to Cosmos over the connection provided
funtion send (connection, payload) {
	
}

// Recieves data from Cosmos over the connection provided
function recieve (connection) {
	
}

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
