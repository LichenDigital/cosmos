< **[Back to documentation main](../documentation.md)**
___

# *Protocol*

All information sent to Cosmos flows through interpreter workers. The interpreter workers inspect recieved data to match it a Cosmos protocol supported by that version of Cosmos.

By treating all data to the Cosmos application as a command, associated with a connection, it is simplistic to pass any type of data you have in the data portion of the command over any type of connection to another Cosmos application to execute the command. Messages will alow information to be communicated simply and efficiently. And, usage of a standaridized protocol will allow for the interoperability of many different entities. More complicated modules may be developed to ensure command delivery reliability.

The command protocol will exist in two forms. A text based protocol, and a binary protocol. These protocols can be easily translated allowing for flexibility. They will be two representations of the same thing. Or I guess one could think of the text based version, being a higher level version of the binary one. Maybe Cosmos can be configured to automatically translate text based commands to binary commands when communicating with other Cosmos processes, and then translate them back to text based commands when they return. Should there be a return protocol argument in the connection setup that specifies what type of protocol to use in return.