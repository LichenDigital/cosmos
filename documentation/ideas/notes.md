# Cosmos Development Notes
```
  o---o     o---o      o--o    o     o     o---o      o--o  
 /         /     \    |        | \ / |    /     \    |     
o         o       o    o-o     |  O  |   o       o    o-o  
 \         \     /        |    |     |    \     /        | 
  o---o     o---o     o--o     o     o     o---o     o--o
```



## Structure

I have determined that the eventual structure of the Cosmos application must not use any libraries. We're talking about writing our own libraries for everything so we can create a modular operating system. It must be this way to allow Cosmos to be run independantly of other code, directly on the processor. Writing the system to be functionally modular and cross platform will provide it the ability to run on simple processors like 8-bit microcontrollers (an effort should be made to eventually write code to support even simpler hardware such as 4, 2, or even 1 bit processors), and still retain all functionality that that chip can accomodate. Even though Cosmos will have the ability to run by itself, it will also have the ability to be run ontop of other operating systems, or run foreign operating systems upon itself. Cosmos processes will be able to communicate how much of a load they are placing on the system, and be able to auto-negotiate hardware usage. There will be modules written that would provide this functionality. Conceptually, those modules would monitor Cosmos processing usage statistics and start, stop, create, or destroy worker processors. 

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

## Interoperability/Communication

Another very important component of Cosmos is its interoperability with itself, c, and other programming languages. Otherwise known as it's ability to communicate... There are multiple ways that Cosmos acheives this. There are bindings for programming languages that allow access to the Cosmos api using the native protocol.Through the use of modules, Cosmos expands its ability to exchange information so it can send and gather arbritrary data, spread and recieve commands to modules etc.


## Command Protocol

All information sent to Cosmos flows through interpreter workers. The interpreter workers inspect recieved data to match it a Cosmos protocol supported by that version of Cosmos.

By treating all data to the Cosmos application as a command, associated with a connection, it is simplistic to pass any type of data you have in the data portion of the command over any type of connection to another Cosmos application to execute the command. Messages will alow information to be communicated simply and efficiently. And, usage of a standaridized protocol will allow for the interoperability of many different entities. More complicated modules may be developed to ensure command delivery reliability.

The command protocol will exist in two forms. A text based protocol, and a binary protocol. These protocols can be easily translated allowing for flexibility. They will be two representations of the same thing. Or I guess one could think of the text based version, being a higher level version of the binary one. Maybe Cosmos can be configured to automatically translate text based commands to binary commands when communicating with other Cosmos processes, and then translate them back to text based commands when they return. Should there be a return protocol argument in the connection setup that specifies what type of protocol to use in return.

## Authorization

Cosmos authorization is optional and permissions based. Users are implemented for tracking who is doing what and what they can do.

By default users can:
- Set permissions of those of similar or lower permissions
  
### Permission structure

Permissions structure can be customized as desired. We'll need to develop a definition for what the building blocks of this system look and how they can work together, so that people can use them to create their own permissions stuctures. This will also allow for visualization of these structures. For example:

- You could set it up so that users who have lower permissions can control users with higher permissions.
- Only once a certain number of people allow permission for a certain user then that permissino is granted.
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

## Authentication

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

