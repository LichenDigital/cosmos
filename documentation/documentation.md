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

**[auth](#auth)**
Handles authentication and authorization. This will include various strategies like OAuth, JWT, etc.

**[config](#config)**
Manages configuration settings in a centralized manner, allowing different parts of cosmos to access these settings.

**[cache](#cache)**
Provides caching mechanisms to improve performance.

**[validation](#validation)**
Tools for validating data, such as form inputs or API request payloads.

**[testing](#testing)**
Includes utilities for unit testing, integration testing, and end-to-end testing

**[error](#error)**
Handles error reporting and exception handling in a standardized way.

**[globalization](#globalization)**
Supports internationalization and localization for multi-language applications.

**[network](#network)**
Transports data between cosmos nodes and other end points, over a variety of software protocols and hardware protocols.

**[store](#store)**
Manages stores of data in a variety of file systems, databases, and memory, accross devices.

**[event](#event)**
An event system designed to handle building complex reactive systems. Integrated systems for monitoring the event system are included in the admin interface for cosmos.

**[ui](#ui)**
Used for building beautiful, cross platform, reactive and connected user interfaces, as well as simple static media. UI has modules that handle displaying components on everything from high DPI screens to 8-bit lcd displays. It can also be used for design and print media.

**[draw](#draw)**
Draws and renders 2D and 3D elements. Used for graphic design, 2D and 3D CAD and VFX.

**[user](#user)**
User manages users and user groups. It has a built in permission system that is built on top of the cosmos [event](#event) system.

**[id](#id)**
Identification of data through fingerprinting and other means.

**[math](#math)**
Provides standardized tools to handle math operations.

**[protocol](#protocol)**
Handles the protocol interpretation and compatability for all cosmos modules.

**[encrypt](#encrypt)**
Encrypt makes data secret and secure. It supports encryption on streams and static sources of data.

**[message](#message)**
Message supports sending data (be it text or other forms of data) through different forms of messaging methods. It supports SMS, email, and other message protocols. We'll have to figure out a module for voice communication, and how to do things like text over voice, and voice over text etc.

**[translate](#translate)**
Translate text, audio, and imagery from one form to another. This covers both spoken and programming / scripting languages.

**[convert](#convert)**

**[media](#media)**
Supports management of `audio`, `images`, and `video` resources.

**[history](#history)**
Handles the history and versioning of every resource that cosmos interacts with. It is conditionally optional, but very powerful if enabled.

**[system](#system)**
Manage the system that cosmos is running on. This includes both the software (like processes running, memory usage), and hardware (storage, monitor temperatures etc.).


## Module Descriptions

### *Protocol*
All information sent to Cosmos flows through interpreter workers. The interpreter workers inspect recieved data to match it a Cosmos protocol supported by that version of Cosmos.

By treating all data to the Cosmos application as a command, associated with a connection, it is simplistic to pass any type of data you have in the data portion of the command over any type of connection to another Cosmos application to execute the command. Messages will alow information to be communicated simply and efficiently. And, usage of a standaridized protocol will allow for the interoperability of many different entities. More complicated modules may be developed to ensure command delivery reliability.

The command protocol will exist in two forms. A text based protocol, and a binary protocol. These protocols can be easily translated allowing for flexibility. They will be two representations of the same thing. Or I guess one could think of the text based version, being a higher level version of the binary one. Maybe Cosmos can be configured to automatically translate text based commands to binary commands when communicating with other Cosmos processes, and then translate them back to text based commands when they return. Should there be a return protocol argument in the connection setup that specifies what type of protocol to use in return.


### *Auth*
#### Authorization

Cosmos authorization is optional and permissions based. Users are implemented for tracking who is doing what and what they can do.

By default users can:
- Set permissions of those of similar or lower permissions
  
### Permission structure

Permissions structure can be customized as desired. We'll need to develop a definition for what the building blocks of this system look and how they can work together, so that people can use them to create their own permissions stuctures. This will also allow for visualization of these structures. For example:

- Only once a certain number of people allow permission for a certain user then that permission is granted.
- All users have the same permissions and there is no heirarchy.

Potential set up of elements of this system could be:

- user
  - permissions
    - permission
      - action (what the user can and cannot do)
      - authorization
        - who (what user grants this permission)
        - condition (what other factors are needed to implement this permission)

It is important to note that by design, actions (calling what the user can do actions for now) can be granted to both allow or disallow activties. Let's think about this, but generally permission systems are set up to dissallow everything unless a user has been granted permission to do something. In this model this is useful in that you could have a user belong to a group that is at the group level granted permissions to do something. But, have that same permission disallowed for a specific user. This model I think could be more powerful than only a one sided permission structure just based on allowing permsissions.

#### Authentication

### *UI*
The UI (User Interfaces) module exposes the power of cosmos visually. It is used to create custom user interfaces accross multiple technologies, from something as basic as a character LCD, to augmented projections. It is both used internally for cosmos's own UI, for managing modules and data, and also for custom UIs that can be built for the web, native apps, and 2D and 3D game engine interfaces.

---

### *Draw*
Draw is the base level module that handles visual rendering of data. Want to draw a triangle? Done. Want to render a complex 3D scenes? It can do that too. And it can do it in both the web and natively. This module like, all other cosmos modules supports realtime collaboration, as well as disconnected workflows.
Inspiration: figma / blender / unreal / clarisse / inventor

---

### *Network*
The network module transmits data over a variety of software and hardware protocols. Software wise, it handles both high bandwidth protocols like `https` and `tcp` as well as low bandwidth protocols lke `i2c` and `serial`. Hardware wise, cosmos has the capability to transmit data over typical means like `wifi`, `lan`, and `wan` as well as less typical means like `light`

---

### *Store*
Store manages stores of data in a variety of file systems, databases, and memory, accross devices. It eases access to data and makes working with it simpler.

Files, databases, and memory are all handled in segmented approaches, and support built in and custom patterns for storage **[structures](#structures)**. Having a common way of handling this data will support conversion between different styles. [Resources](#resources) are the way of encapsulating and referring to that data.

#### Resources
A resource is a way of encapsulating and referring to data. It references a `location` and `range` of data, and can be broken into `chunks`. Chunks are critical in the handling large volumes data. It allows for the breaking of large data `structures` into smaller and more managable pieces. This helps systems of any capability, and current usage the ability to efficiently work with data.

#### Structures
Structures are organizational methods of data storage. Structures have `patterns` associated with them that facilitate access and management of the data.

  - graph
  - key
  - object
  - table
  - tree
  - map

---

### *User*
User manages users and user groups. It has a built in permission system that is built on top of cosmos's [event](#event) system. User structures and permission systems can be built to fit custom organizational structures. Each user and group are assigned a unique id using cosmos's id module, and can have roles associated with them that are used in the permission system for a specific (and familiar) way of managing permissions. Permissions can also be granted or revoked base on other conditions through the cosmos `event` system.

---

### *Event*
The cosmos event system is designed to handle building complex reactive systems. Integrated systems for monitoring the event system are included in the [monitor](#monitor) module. Events are comprised of a few simple elements: [triggers](#triggers) and [actions](#actions) and the ubiqutous [id](#id) and [groups](#group).

#### Triggers
Trigges are conditions that cosmos watch then initiate coresponding actions. An event may have multiple triggers, and muliple corresponding actions. Triggers are themselves an entity that cosmos tracks, and are grouped together, therefore making matching associated events easier.

For example, if two events are registered with cosmos, with the same trigger, they are associated through their triggers. So one trigger is stored, and is associated with muliple events. In this way, not only as mentioned above, does it make cosmos have to work less since it can watch one trigger instead of two of them, but it also makes it possible to edit the same trigger on multiple events. If the trigger of an event is edited, and it matches other triggers, you can update a selection, or all of them. Triggers themselves are grouped using cosmos groups. Also using this strategy, if an event is registered that matches another event, then the event can still be registered, but is only linked to the other event for the details of the trigger and actions. There could also be a setting in the event system that could discard events that match other events, instead of registering them and pointing them at the other same event. The difference between the two events would be the id, groups, and time that it was registered, and potentially the history too.

Triggers can be any condition that cosmos can evaluate and match. So they can also match actions. This allows for chaining events together. So one event can trigger an action, which then triggers another action, etc. They can also watch other events based on their id or group. This means that there can be event chaining that can watch for activity of a particular event or group of events.

#### Actions
Actions are the commands that cosmos runs on the triggering of an event. These commands are cosmos running modules, so an action could be used not only in a familiar way through to trigger something happening visually in a user interface, but they can be used to run any command, so their useage is broad and deep. This means that it can be used for everything from storage changes, to user management, and transportation of data. 

#### Event Managment
Events are stored like other data in the cosmos storage module. Here they can be created, read, updated, deleted like anything else. This includes both singular edits as well as bulk edits. They can like other pieces of data have other events associated with them like an expiration (delete this id or group when X time has elapsed etc.), transformation (the event changes its action based on the time of day that the event is triggered) etc. So chainging allows for very complex structures to be able to be built simply

#### Event Visualization
Event visualization is handled through the [visualize](#visualize) module. This module leverages the [UI](#ui) module to create a way to visualize events, their connection, and realtime flow. This helps to see what is going on and debug event flow.

---

### *History*
Tracks the history of every resource that cosmos interacts with. It is conditionally optional, but very powerful if enabled. At its most basic, the history module is important to facilitate undo and redo operations. It can be a history of commands run, events triggered, files interacted with etc. This means that cosmos can reconstruct the history of data and actions. It also stores the history of a resource every time it interacts with it if it is different from previously; so that it can track the history of data that itself does not change or manipulate, or it could be another cosmos instance that is set to not store the history of its actions.

The history module is hooked up to the protocol module so that it can selectively track the data it needs.


## Development Priorities and Goals

### Structure

The core of cosmos will have as few external dependencies as possible, only using them when something very specific, complex, and difficult is being achieved (media transcoding for example). Striving to minimize these dependencies will keep cosmos simpler, and easier to maintain and keep secure. Writing the system to be functionally modular and cross platform will provide it the ability to run on simple processors like 8-bit microcontrollers (an effort should be made to eventually write code to support even simpler hardware such as 4, 2, or even 1 bit processors), and retain as much functionality as that that chip can accomodate. Even though Cosmos will have the ability to run by itself on bare metal, it will also have the ability to be run on top of other operating systems, or run foreign operating systems upon itself (through virtualization / containerization). Cosmos processes will be able to communicate how much of a load they are placing on the system, and be able to auto-negotiate hardware usage. There will be modules written that would provide this functionality. Conceptually, those modules would monitor Cosmos processing usage statistics and start, stop, create, or destroy worker processors. 

Cosmos must be lighting fast. And therefore inherently simplistic. The modules that use Cosmos will be free to do very complex operations, but the program as a whole remains lean, fast and secure. Highly involved actions like securing data, authorizing access to modules, creating connections and the like will be modularized to ensure that the application can continue to run smoothly despite potential bottlenecks, like waiting on intensive processes, or transmission delay, etc. By maintaining this flexible quality, Cosmos will be able to handle a huge volume of information on every input and output it has, by capturing the data in queues in memory, emptying full queues to disk, or falling back to blocking if those options are not available. Then it can span any number of worker threads and processes to chunk through the data.

## Desired structural quailities:
    - Fast
    - Simple
    - Secure
    - Distributed
    - Fault tollerant
    - Highly modular

Even though I conceptualize how modules would be built and function, and how the Cosmos protocol would function, it is difficult for me to determine how Cosmos processes will actually talk to eachother. I envision Cosmos being able to talk over a wide variety of transmission methods, over different protocols and mediums.

Cosmos will have to be supportive of all operating systems, and processors. It should be able to run on even highly simplistic processors, like microcontrollers. These basic processing units, could be utilized by a module to create massive parrallelization of processing, in many different forms. I wonder if you could hand out more intensive tasks to a huge number of microcontrolles by chunking the large task up into small batches of work, and distributing it to them intelligently and recieve their responses from the work, and provide them with more work when they ask for it.

The more I think about it, the more I realize I should use ZMQ as the basis for communication inbetween processes, threads, and internal communication. This will provide almost all of the functionality I am looking for initially, until I learn enough that I can make that functionality native to Cosmos. Cosmos will also be able to use other protocols to transmitt commands through other internal transport methods like http, ssh (which would be ssh out, and std in on the other end, since ssh is a remote shell), std in, (you could have a module monitoring a file, and treating it as a repository for commands and rules that would tell it to execute any additions or modifications of commands, and it could use the diff module to determine the differences in the file)

For running on microcontrollers zeromq won't be an option. But other transport methods such as serial or I2C will be utilized. Cosmos for microcontrollers will likely have to be written using C++ since as far as I can tell at this time, rust is not well supported on those chipsets. ARM processors seem to be the simplest (there seem to be rather powerful arm chips out there right now) chips that rust can easily be developed for.



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
  - **group**: A superset of ID and hash, groups have an optional `name` that can reference the group. Groups are used to group data together without explicit relationships.

##Sections:
  modules
  keywords
  protocol
  common-data



## General Feature Requests -
  - Ability to use SCSS variable in javascript and visaversa



The web UI contains building blocks for building pages and is broken down into sub-types. When developing the SCSS and CSS for the UI, the focus will be to make it as modular and efficient as possible. Structural and visual code will be separated as much as possible. Extensive use of variables, mixins, functions and the like will also be used to make maintenance and use of the code as simple and intuitive as possible.

UI protocol will have the ability to collect, send, and execute or replicate the following information:

  - Interaction  (can have multiple people working on the same UI)
    - Location
    - Movement of interaction (scroll etc.)
    - Type (pressure (can be negative) etc.)
    - Value



# Inspiration

Codrops (tympus.net) has great button / progress bar etc. inspiration available
Clarise project file...: https://www.youtube.com/watch?v=iiMvUTKjnGk





## Interoperability/Communication

Another very important component of Cosmos is its interoperability with itself, c, and other programming languages. Otherwise known as it's ability to communicate... There are multiple ways that Cosmos acheives this. There are bindings for programming languages that allow access to the Cosmos api using the native protocol.Through the use of modules, Cosmos expands its ability to exchange information so it can send and gather arbritrary data, spread and recieve commands to modules etc.

Automated Development (Code Generation)

There needs to be a module developed as soon as possible that will assist in development of Cosmos by extrapolating standards and protocols into cross platform code. The focus of Cosmos development should be creating logic and behavior and leave as much of the code to the generator, therefore ensuring more accurate code creation, mantenence and repeatability. Employing Cosmos's ablility to create connections between pieces of data, a developer is able to mix generated code with human written (and Cosmos validated) code.

### Portability

The structural qualities of Cosmos should be developed in a standard manner, on varying computational platforms. An example of this is the ability of Cosmos to employ different modular addressing protocols. Cosmos must be written to accomodate different computer architectures that have different qualities and capabilities. Think about the difference between a 4-bit microcontroller and a 64-bit cpu. If our addressing scheme was fixed on a single specification, our application would be much more brittle. If a 64-bit host address had to be handled by a 4-bit IC (integrated circuit), it could concievably do it, however, all operations involving that number would take sixteen times longer than they would if they were a 4-bit counterpart.


Cosmos needs the ability to operate in a connectionless, and connected manner, and have the abilty to send data in a serial, parrallel, syncronous, asyncronous, stream and chunked methods.

Code Editor
    Add tags to parts of code that then show up in your line navigation and can be accessed easily to navigate to certain sections of code. May be difficult to impliment as you'll need to devise a way of creating the tag and having it show up in the code as a comment so that it can be tracked accross editors etc. Either that or it will rely on identifying sections of code, but that could get messy as code is moved arround etc.

## UI

Have user interface elements be able to affect other elements with relationships.

* One use case for this is being able to have other elements be able to flow around other elements. For instance there could be something like a flow factor which would give an item a presence when moving through a space and be able to affect elements in certain directions. Picture placing an interface item like an image somewhere and being able to have it force other user interface elements to flow around it. Similar to how float works, but not just affecting things in a top to bottom left to right or right to left way.
* Have elements be able to have constraints. 


## Image and Video Editing

* Have the ability to define multiple crops of the same image / video
* Crops can be of different position and size
* Crops can be animated over time in position and size
* Open questions
  * Should crops be limited to the physical image / video size?
  * Should they be able to have the support for different shapes other than rectangles? (or is this masking and should be used as masking and then also have support for blending modes)


# File Transcoding

The file module can transcode (convert) different file types.


## Data Conversion

## Data Version History and Control

## Devices

Introducing cosmos hardware. The bulk of the work initially on cosmos will be focused on the software. But, there will be open source cosmos hardware developed to ensure that access to the software is not limited to the people that can afford the hardware to run it. This means that not only will there be plans and tutorials to create cosmos devices, but there will also be funds set up that allow for people to access hardware and data connections for free.

## Graphics

Section for talking about how graphics will be implemented in Cosmos.

### Color

Color may be sononomous with graphics. In either the lack thereof or implementation of it it's involved. Cosmos will support a wide array of color models and will be able to swicth / convert colors between them. Here are color models Cosmos will support by default:

- RGB
  - RGB+A
- HSV
  - HSV+A
- CMYK
  - CMYK+A
- HEX
  - HEX+A

Notice that each color model can have an associated alpha (transparency) value associated with it.

In implementing colors we'll need to create supports around dealing with them. Things that may help with this could be:

- Libraries / Themes
- Sampling
- Mixing

Colors will have a potential structure in them. In that more complex representation and mixing of colors will reference base colors for example:

- Gradients
  - Radial
  - Linear
- Maps
  - Noise
  - Bitmap
  - Vector

Fills or strokes will reference those base colors or more complex representations of colors. Let's definitely look into color libraries and what they can do. For instance SASS has great functions when it comes to color. Looking into what those can do will give us a good foundation for what we can build out for Cosmos and expose powerful tools both in code and visually.


## DEVELOPMENT STRATEGY - 
  The general development strategry I am going to take initially is to work on getting all of the html elements and CSS figured out to get to the end result desired. I'll follow this up by breaking down all of the similaritys and integrating things like mixins, functions, and extends.


## CONTROL - 
A control is a single element or group of elements that make up a control.

**button**
**input**
**textArea**
**datalist**
**select** - select and datalist are pretty much the same thing, both being dropdowns, with the datalist providing the ability to search for data. Perhaps consider replacing all select with datalists
**checkbox**
**radio**
**output**

For instance a control could be a select + input + button + output. A control could also be a singular button, that contains just text, or an icon and text etc.

**FIELD** -
label
control
help

**FORM** -
control
field

## Command Line Interface (CLI)
The initial interface to run and interact with cosmos will be on the command line.

**Command Line Implementation Ideas**
Loading and progress spinners and bars will be inspired by https://www.npmjs.com/package/cli-spinners and other


## DEVELOPMENT RESOURCES

DESIGN TOOLS
https://trianglify.io/
http://www.0to255.com/

HINTS AND SHORTCUTS
https://devhints.io/

DEVELOPMENT METHODOLOGY
CSS
http://www.didoo.net/to-bem-or-not-to-bem/
http://www.didoo.net/to-bem-or-not-to-bem/04__interview-with--micah__godbolt.html
https://github.com/micahgodbolt/beacss
https://www.amazon.com/Frontend-Architecture-Design-Systems-Sustainable-ebook/dp/B01B6WS868
http://www.didoo.net/to-bem-or-not-to-bem/05__interview-with--varya__stepanova.html
OOCSS
BEM
BEACSS
ATOMIC CSS
CSS MODULES
https://pusher.com/sessions/meetup/front-end-london/css-in-javascript
https://medium.com/yplan-eng/atomic-css-modules-cb44d5993b27


https://www.styled-components.com/docs


JS
https://blog.garstasio.com/you-dont-need-jquery/dom-manipulation/

WEBPACK
https://www.smashingmagazine.com/2017/02/a-detailed-introduction-to-webpack/
https://www.contentful.com/blog/2017/04/04/es6-modules-support-lands-in-browsers-is-it-time-to-rethink-bundling/
https://medium.freecodecamp.org/you-might-not-need-to-transpile-your-javascript-4d5e0a438ca


https://www.puncsky.com/blog/2015/01/13/understanding-reactor-pattern-for-highly-scalable-i-o-bound-web-server/



DESIGN LEARNING

LOGOS
https://99designs.com/blog/tips/types-of-logos/

FORMS
https://uxplanet.org/float-label-pattern-in-ux-form-design-7ab5e33010ab
^^^ CONTRADICTION HERE: https://uxdesign.cc/design-better-forms-96fadca0f49c
https://www.nngroup.com/articles/form-design-placeholders/
http://govuk-elements.herokuapp.com/form-elements/#form-labels
https://www.smashingmagazine.com/2018/10/form-design-patterns-release/

BUTTONS
https://uxplanet.org/7-rules-for-mobile-ui-button-design-e9cf2ea54556

SPACING
https://www.justinmind.com/blog/minimalism-in-interactive-design-dreaming-of-a-white-space-xmas/

DESIGN TOOL LEARNING
https://bezier.method.ac/

PROGRAMMING LANGUAGES AND TECHNOLOGIES
JAVASCRIPT
https://medium.com/@marcisbee/how-i-built-super-fast-js-framework-faster-than-react-ea99f0d03150
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Iterators_and_Generators
https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty
https://medium.com/javascript-scene/master-the-javascript-interview-what-is-a-pure-function-d1c076bec976
https://medium.com/@zeolearn/functional-programming-in-javascript-6ef89de6e0ef
https://medium.com/the-renaissance-developer/concepts-of-functional-programming-in-javascript-6bc84220d2aa
https://blog.bitsrc.io/understanding-javascript-mutation-and-pure-functions-7231cc2180d3



Rust
https://doc.rust-lang.org/book/2018-edition/index.
https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html

WASM
https://webassembly.org/

NODE
https://www.smashingmagazine.com/2018/06/nodejs-tools-techniques-performance-servers/

VUE

DART
https://hackernoon.com/why-flutter-uses-dart-dd635a054ebf

FLUTTER
https://hackernoon.com/why-flutter-uses-dart-dd635a054ebf

BROWSERS
https://ldnwebperf.org/events/progressive-web-apps-with-jake-archibald/



PROGRAMMING TECHNIQUES
MVC
https://www.infoq.com/articles/no-more-mvc-frameworks 


Different formatting styles:

```
Underlined
----------
```

```
---------
Overlined
```

```
Possible Characters to Utilize:
------
======
++++++
||||||
\\\\\\
//////
~~~~~~
``
######
^^^^^^
******
______
()
{}
[]
::::::
;;;;;;
......
,,,,,,
''''''
""""""
<<<<<<
>>>>>>

Orientation:
  Top:
    """
    '''
    ``
    ***
    ^^^

  Bottom:
    ___
    ...
    ,,,

  Center:
    +++
    ===
    ---
    []
    {}
    ()
    ;;;
    :::
    <<<
    >>>
    ///
    \\\
    |||
```


Intitially the following modules will be availble for usage:
  
  file
    version
    difference
    send
    recieve
  node
  transport
    process
    thread
    http
    ssh
    zmq
  database
  
  
And eventually these modules will be available
  audio
  display
    image
    video
    vector
    polygon
    graph
  project

  process
  threading
  memory

  application
    start
    stop

  financial
    charge
    send
    deposit
    withdraw
    convert
    tax
  production
  inventory


  time
    calendar
    scheduling
    timetracking
    timer
    stopwatch


  location
    map
    direction
      land
      sea
      air


  math

  science
    climate
    biology
    geological
  
  engineering
    simulation
    CAD
      hybrid
        2D
        3D

  medical
    monitor
      pulse
      bloodpressure
      glucose
      temperature
      bloodoxygen


  react/trigger


  sense
    light



  control
    temperature
    motion
    humidity
    light
    pressure
    flow



  art

  web
    sever
    html
    css
    javascript

  notify

  communication
    text
    email
    audio
    video
    image
    light

  measure
    volume
    mass
    length


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


Opposed action words:
	start - stop
	intitialize - terminate
	add - remove
	create - destroy
	setup - takedown
	load - unload
	set - unset
	read - write

Symetrical action words:
	read - get
	read - request
	send - push
	send - post

Common action verbs:
	create
	read
	update
	delete
	connect
	disconnect



COSMOS MODULE_NAME MODULE_COMMAND MODULE_COMMAND_ARGUMENTS
CO

#Data Protocol

```
time [ time, date, timezone ]
location [ coordinates, reference, position, direction ]
units {
    length : {
        metric: [ mm, cm, m, km ],
        imperial: [ in, ft, yd, mi ]
        conversion: []      
    },
    volume: {
        metric: []
        imperial: []
    }

}

```

 __   __    __         __    __
/    /  \  (_   |\/|  /  \  (_  
\__  \__/  __)  |  |  \__/  __)

 
   o---o     o---o      o--o    o     o     o---o      o--o   
  /         /     \    |        | \ / |    /     \    |       
 o         o       o    o-o     |  O  |   o       o    o-o    
  \         \     /        |    |     |    \     /        |   
   o---o     o---o     o--o     o     o     o---o     o--o    

   o---o  
  /       
 o        
  \       
   o---o  

   o---o    
  /     \   
 o       o  
  \     /   
   o---o    

  o--o   
 |       
  o-o    
     |   
 o--o    

 o     o  
 | \ / |  
 |  O  |  
 |     |  
 o     o  

  ____    ___    ____    __  __    ___    ____  
 / ___|  / _ \  / ___|  |  \/  |  / _ \  / ___| 
| |     | | | | \___ \  | |\/| | | | | | \___ \ 
| |___  | |_| |  ___) | | |  | | | |_| |  ___) |
 \____|  \___/  |____/  |_|  |_|  \___/  |____/


   _____      ______       ____     __   __      ______      ____
  / ____|    / ____ \     / ___|   |  \_/  |    / ____ \    / ___|
 / /        / /    \ \   | (___    | |\_/| |   / /    \ \  | (___ 
| |        | |      | |   \___ \   | |   | |  | |      | |  \___ \ 
 \ \____    \ \____/ /     ___) |  | |   | |   \ \____/ /    ___) |
  \_____|    \______/     |____/   |_|   |_|    \______/    |____/  



   _____
  / ____|
 / /       
| |       
 \ \____  
  \_____|  
   ______ 
  / ____ \
 / /    \ \
| |      | |
 \ \____/ /
  \______/
  ____
 / ___| 
| (___ 
 \___ \ 
  ___) |
 |____/      
  __   __
 |  \_/  | 
 | |\_/| |  
 | |   | |
 | |   | |
 |_|   |_|
       