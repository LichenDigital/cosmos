< **[Back to documentation main](../documentation.md)**
___

# *Store*

Store manages stores of data in a variety of file systems, databases, and memory, accross devices. It eases access to data and makes working with it simpler.

Files, databases, and memory are all handled in segmented approaches, and support built in and custom patterns for storage **[structures](#structures)**. Having a common way of handling this data will support conversion between different styles. [Resources](#resources) are the way of encapsulating and referring to that data.

## Resources
A resource is a way of encapsulating and referring to data. It references a `location` and `range` of data, and can be broken into `chunks`. Chunks are critical in the handling large volumes data. It allows for the breaking of large data `structures` into smaller and more managable pieces. This helps systems of any capability, and current usage the ability to efficiently work with data.

## Structures
Structures are organizational methods of data storage. Structures have `patterns` associated with them that facilitate access and management of the data.

  - graph
  - key
  - object
  - table
  - tree
  - map