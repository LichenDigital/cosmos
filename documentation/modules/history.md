< **[Back to documentation main](../documentation.md)**
___

# *History*
Tracks the history of every resource that cosmos interacts with. It is conditionally optional, but very powerful if enabled. At its most basic, the history module is important to facilitate undo and redo operations. It can be a history of commands run, events triggered, files interacted with etc. This means that cosmos can reconstruct the history of data and actions. It also stores the history of a resource every time it interacts with it if it is different from previously; so that it can track the history of data that itself does not change or manipulate, or it could be another cosmos instance that is set to not store the history of its actions.

The history module is hooked up to the protocol module so that it can selectively track the data it needs.