```
   o---o     o---o      o--o    o     o     o---o      o--o   
  /         /     \    |        | \ / |    /     \    |       
 o         o       o    o-o     |  O  |   o       o    o-o    
  \         \     /        |    |     |    \     /        |   
   o---o     o---o     o--o     o     o     o---o     o--o 
```   

# cosmos = ∞

## What is cosmos

cosmos is an open source event driven, asyncronous, modular, distributed, cross platform application framework and eventual operating system. It is also a software and hardware platform built with and to support the framework. It's primary focus is enhancing access and connectivity of systems through simplification, abstraction, and standardization. The cosmos community is open, diverse, and inclusive. As is the source code, protocol, and hardware, which is available in a variety of languages and supports people of all ability.

comsos = ∞

## Core Modules:
**[transport](#transport)**
Transports data between cosmos nodes and other end points, over a variety of software protocols and hardware protocols.

**[store](#store)**
Manages stores of data in a variety of file systems, databases, and memory, accross devices.

**[event](#event)**
An event system designed to handle building complex reactive systems. Integrated systems for monitoring the event system are included in the admin interface for cosmos.

**[ui](#ui)**
Used for building beautiful, cross platform, reactive and connected user interfaces, as well as simple static media. UI has modules that handle displaying components on everything from high DPI screens to 8-bit lcd displays. It can also be used for design and print media.

**[user](#user)**
User manages users and user groups. It has a built in permission system that is built on top of the cosmos [event](#event) system.

**[id](#id)**

**[math](#math)**

**[protocol](#protocol)**

**[encrypt](#encrypt)**
Encrypt makes data secret. It supports encryption on streams and static sources of data.

**[message](#message)**
Message supports sending data (be it text or other forms of data) through different forms of messaging methods. It supports SMS, email, and other message protocols. We'll have to figure out a module for voice communication, and how to do things like text over voice, and voice over text etc.

**[translate](#translate)**
Translate text, audio, and imagery from one form to another. This covers both spoken and programming / scripting languages.

**[convert](#convert)**

**[media](#media)**
Supports management of `audio`, `images`, and `video` resources.

**[history](#history)**
Tracks the history of every resource that cosmos interacts with. It is conditionally optional, but very powerful if enabled.

## Development Questions
  - How do we handle modules?
  - How do we create a common protocol for modules to be standardized (so they can be implemented in other languages)?
  - How do we afford for sub modules and including other modules?
  - How do we deal with module dependencies?

## Common commands:
  - **create**, **crt**: Creates a resource
  - **get**: Gets a resource
  - **update**, **upt**: Updates a resource
  - **delete**, **dlt**: Deletes a resource
  - **duplicate**: **dup**: Duplicates a resource (uses get and create)
  - **connect**, **con**: Connects a resource to another resource
  - **disconnect**, **dsc**: Disconnects a resource from another resource

##Unique commands:
  - **send**: **snd**: Sends a resource
  - **receive**: **rcv**: Recieves a resource


##Common data:
  - **id**: Unique ID for the resource
  - **hash**: Hash of the resource. Uses hash module. Can handle established and custom hash algorythms.

##Sections:
  modules
  keywords
  protocol
  common-data


## Module Descriptions

### -- *Transport* --
The transport module transmits data over a variety of software and hardware protocols. Software wise, it handles both high bandwidth protocols like `https` and `tcp` as well as low bandwidth protocols lke `i2c` and `serial`. Hardware wise, cosmos has the capability to transmit data over typical means like `wifi`, `lan`, and `wan` as well as less typical means like `light` (think )

### -- *Store* --
Store manages stores of data in a variety of file systems, databases, and memory, accross devices. It eases access to data and makes working with it simpler.

Files, databases, and memory are all handled in segmented approaches, and support built in and custom patterns for storage **[structures](#structures)**. Having a common way of handling this data will support conversion between different styles. [Resources](#resources) are the way of encapsulating and referring to that data.

#### Resources
A resource is a way of encapsulating and referring to data. It references a `location` and `range` of data, and can be broken into `chunks`. Chunks are critical in the handling data. It allows for the breaking of large data `structures` into smaller and more managable pieces. This helps systems of any capability, and current usage the ability to efficiently work with data.

#### Structures
Structures are organizational methods of data storage. Structures have `patterns` associated with them that facilitate access and management of the data.

  - graph
  - key
  - object
  - table
  - tree
  - map

### -- *User* --
User manages users and user groups. It has a built in permission system that is built on top of cosmos's [event](#event) system. User structures and permission systems can be built to fit custom organizational structures. Each user and group are assigned a unique id using cosmos's id module, and can have roles associated with them that are used in the permission system for a specific (and familiar) way of managing permissions. Permissions can also be granted or revoked base on other factors, basically anything that the event system can handle.

### -- *Event* --
The cosmos event system is designed to handle building complex reactive systems. Integrated systems for monitoring the event system are included in the admin interface. Events are comprised of a few simple elements: [triggers](#triggers) and [actions](#actions) and of course the ubiqutous [id](#id) and [groups](#group).

#### Triggers
Trigges are conditions that cosmos watch then initiate coresponding actions. An event may have multiple triggers, and muliple corresponding actions. Triggers are themselves an entity that cosmos tracks, and are grouped together, therefore making matching associated events easier.

For example, if two events are registered with cosmos, with the same trigger, they are associated through their triggers. So one trigger is stored, and is associated with muliple events. In this way, not only as mentioned above, does it make cosmos have to work less since it can watch one trigger instead of two events, but it also makes it possible to edit the same trigger on multiple events. If the trigger of an event is edited, and it matches other triggers, you can update a selection, or all of them. Triggers themselves are grouped using cosmos groups. Also using this strategy, if an event is registered that matches another event, then the event can still be registered, but is only linked to the other event for the details of the trigger and actions. There could also be a setting in the event system that could discard events that match other events, instead of registering them and pointing them at the other same event. The difference between the two events would be the id, groups, and time that it was registered, and potentially the history too.

Triggers can be any condition that cosmos can evaluate and match. So they can also match actions. This allows for chaining events together. So one event can trigger an action, which then triggers another action, etc. They can also watch other events based on their id or group. This means that there can be event chaining that can watch for activity of a particular event or group of events.

#### Actions
Actions are the commands that cosmos runs on the triggering of an event. These commands are cosmos running modules, so an action could be used not only in a familiar way through to trigger something happening visually in a user interface, but they can be used to run any command, so their useage is broad and deep. This means that it can be used for everything from storage changes, to user management, and transportation of data. 

#### Event Management


### -- *History* --
Tracks the history of every resource that cosmos interacts with. It is conditionally optional, but very powerful if enabled. It can be a history of commands run, events triggered, files interacted with etc. This means that cosmos can reconstruct the history of data and actions. It also stores the history of a resource every time it interacts with it if it is different from previously; so that it can track the history of data that itself does not change or manipulate, or it could be another cosmos instance that is set to not store the history of its actions.





## Bare modules:
  - memory
    - stdint
    - stdfloat
  - process



## General Feature Requests -
  - Ability to use SCSS variable in javascript and visaversa



The web UI contains building blocks for building pages and is broken down into sub-types. When developing the SCSS and CSS for the UI, the focus will be to make it as modular and efficient as possible. Structural and visual code will be separated as much as possible. Extensive use of variables, mixins, functions and the like will also be used to make maintenance and use of the code as simple and intuitive as possible.

UI protocol will have the ability to collect, send, and execute or replicate the following information:

  - Interaction  (can have multiple people working on the same UI)
    - Location
    - Movement of interaction (scroll etc.)
    - Type (pressure (can be negative) etc.)
    - Value

