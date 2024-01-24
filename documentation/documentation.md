```
 ______     ______     ______     __    __     ______     ______    
/\  ___\   /\  __ \   /\  ___\   /\ "-./  \   /\  __ \   /\  ___\   
\ \ \____  \ \ \/\ \  \ \___  \  \ \ \-./\ \  \ \ \/\ \  \ \___  \  
 \ \_____\  \ \_____\  \/\_____\  \ \_\ \ \_\  \ \_____\  \/\_____\ 
  \/_____/   \/_____/   \/_____/   \/_/  \/_/   \/_____/   \/_____/ 
```   

# cosmos = ∞

## What is cosmos

cosmos is an open source event driven, asyncronous, modular, distributed, cross platform application framework and eventual operating system. It is also a software and hardware platform built with and to support the framework. It's primary focus is enhancing access and connectivity of systems through simplification, abstraction, and standardization. The cosmos community is open, diverse, and inclusive. As is the source code, protocol, and hardware, which is available in a variety of languages and supports people of all ability.

comsos = ∞

## Core Modules:

**[auth](./modules/auth.md)**
Handles user and user group management, authentication, and authorization. The built in permission system is built on top of the cosmos [event](./modules/event.md) system.

**[cache](./modules/cache.md)**
Provides caching mechanisms to improve performance.

**[config](./modules/config.md)**
Manages configuration settings in a centralized manner, allowing different parts of cosmos to access these settings.

**[data](./modules/data.md)**
Operations on data like hashing, chunking, diffing, and validating.

**[encrypt](./modules/encrypt.md)**
Encrypt makes data secret and secure. It supports encryption on streams and static sources of data.

**[error](./modules/error.md)**
Handles error reporting and exception handling in a standardized way.

**[event](./modules/event.md)**
An event system designed to handle building complex reactive systems. Integrated systems for monitoring the event system are included in the admin interface for cosmos.

**[finance](./modules/finance.md)**

**[graphics](./modules/graphics.md)**
Draws and renders 2D and 3D elements. Used for graphic design, 2D and 3D CAD and VFX.

**[globalization](./modules/globalization.md)**
Supports internationalization and localization for multi-language applications.

**[history](./modules/history.md)**
Handles the history and versioning of every resource that cosmos interacts with. It is conditionally optional, but very powerful if enabled.

**[id](./modules/id.md)**
Generates ids and fingerprints of various types (alpha numeric, UUIDs).

**[math](./modules/math.md)**
Provides standardized tools to handle math operations.

**[message](./modules/message.md)**
Message supports sending data (be it text or other forms of data) through different forms of messaging methods. It supports SMS, email, and other message protocols. We'll have to figure out a module for voice communication, and how to do things like text over voice, and voice over text etc.

**[media](./modules/media.md)**
Supports management of `audio`, `images`, and `video` resources including converting, transpiling, and editing.

**[network](./modules/network.md)**
Transports data between cosmos nodes and other end points, over a variety of software protocols and hardware protocols.

**[protocol](./modules/protocol.md)**
Handles the protocol interpretation and compatability for all cosmos modules.

**[store](./modules/store.md)**
Manages stores of data in a variety of file systems, databases, and memory, accross devices.

**[system](./modules/system.md)**
Manage the system that cosmos is running on. This includes both the software (like processes running, memory usage), and hardware (storage, monitor temperatures etc.).

**[test](./modules/test.md)**
Includes utilities for unit testing, integration testing, and end-to-end testing

**[translate](./modules/translate.md)**
Translate text, audio, and imagery from one form to another. This covers both spoken and programming / scripting languages.

**[ui](./modules/ui.md)**
Used for building beautiful, cross platform, reactive and connected user interfaces, as well as simple static media. UI has modules that handle displaying components on everything from high DPI screens to 8-bit lcd displays. It can also be used for design and print media.

**[validation](./modules/validation.md)**
Tools for validating data, such as form inputs or API request payloads.

## Software Development Methodology
The development of cosmos will take a lot of effort, thought, and careful planning. In order to guide this endeavor we've got a [document](design-development.md) that outlines our design and development philosophy, ideas, and questions.
