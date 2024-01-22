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

## Interoperability/Communication

Another very important component of Cosmos is its interoperability with itself, c, and other programming languages. Otherwise known as it's ability to communicate... There are multiple ways that Cosmos acheives this. There are bindings for programming languages that allow access to the Cosmos api using the native protocol.Through the use of modules, Cosmos expands its ability to exchange information so it can send and gather arbritrary data, spread and recieve commands to modules etc.

Automated Development (Code Generation)

There needs to be a module developed as soon as possible that will assist in development of Cosmos by extrapolating standards and protocols into cross platform code. The focus of Cosmos development should be creating logic and behavior and leave as much of the code to the generator, therefore ensuring more accurate code creation, mantenence and repeatability. Employing Cosmos's ablility to create connections between pieces of data, a developer is able to mix generated code with human written (and Cosmos validated) code.

### Portability

The structural qualities of Cosmos should be developed in a standard manner, on varying computational platforms. An example of this is the ability of Cosmos to employ different modular addressing protocols. Cosmos must be written to accomodate different computer architectures that have different qualities and capabilities. Think about the difference between a 4-bit microcontroller and a 64-bit cpu. If our addressing scheme was fixed on a single specification, our application would be much more brittle. If a 64-bit host address had to be handled by a 4-bit IC (integrated circuit), it could concievably do it, however, all operations involving that number would take sixteen times longer than they would if they were a 4-bit counterpart.


Cosmos needs the ability to operate in a connectionless, and connected manner, and have the abilty to send data in a serial, parrallel, syncronous, asyncronous, stream and chunked methods.

### Code Editor

Add tags to parts of code that then show up in your line navigation and can be accessed easily to navigate to certain sections of code. May be difficult to impliment as you'll need to devise a way of creating the tag and having it show up in the code as a comment so that it can be tracked accross editors etc. Either that or it will rely on identifying sections of code, but that could get messy as code is moved arround etc.

### UI Editor

### Media Editor


## Development Questions
  - How do we handle modules?
  - How do we create a common protocol for modules to be standardized (so they can be implemented in other languages)?
  - How do we afford for sub modules and including other modules?
  - How do we deal with module dependencies?

## cosmos verbage
The language of cosmos is important. It both describes what modules and their actions do, and also elicits emotion.

### Common commands:
  - **create**, **crt**: Creates a resource
  - **get**: Gets a resource
  - **update**, **upt**: Updates a resource
  - **delete**, **dlt**: Deletes a resource
  - **duplicate**: **dup**: Duplicates a resource (uses get and create)
  - **connect**, **con**: Connects a resource to another resource
  - **disconnect**, **dsc**: Disconnects a resource from another resource

### Unique commands:
  - **send**: **snd**: Sends a resource
  - **receive**: **rcv**: Recieves a resource

### Opposed action words:
	start - stop
	intitialize - terminate
	add - remove
	create - destroy
	setup - takedown
	load - unload
	set - unset
	read - write

### Symetrical action words:
	read - get
	read - request
	send - push
	send - post

### Common action verbs:
	create
	read
	update
	delete
	connect
	disconnect



##Common data:
  - **id**: Unique ID for the resource
  - **hash**: Hash of the resource. Uses hash module. Can handle established and custom hash algorythms.
  - **group**: A superset of ID and hash, groups have an optional `name` that can reference the group. Groups are used to group data together without explicit relationships.


## Inspiration

- Codrops (tympus.net) has great button / progress bar etc. inspiration available
- Clarise project file...: https://www.youtube.com/watch?v=iiMvUTKjnGk


## DEVELOPMENT RESOURCES

### DESIGN TOOLS
https://trianglify.io/
http://www.0to255.com/

### HINTS AND SHORTCUTS
https://devhints.io/

### DEVELOPMENT METHODOLOGY
CSS
http://www.didoo.net/to-bem-or-not-to-bem/
http://www.didoo.net/to-bem-or-not-to-bem/04__interview-with--micah__godbolt.html
https://github.com/micahgodbolt/beacss
https://www.amazon.com/Frontend-Architecture-Design-Systems-Sustainable-ebook/dp/B01B6WS868
http://www.didoo.net/to-bem-or-not-to-bem/05__interview-with--varya__stepanova.html
CSS MODULES
https://pusher.com/sessions/meetup/front-end-london/css-in-javascript
https://medium.com/yplan-eng/atomic-css-modules-cb44d5993b27
https://www.styled-components.com/docs

### JS
https://blog.garstasio.com/you-dont-need-jquery/dom-manipulation/

### WEBPACK
https://www.smashingmagazine.com/2017/02/a-detailed-introduction-to-webpack/
https://www.contentful.com/blog/2017/04/04/es6-modules-support-lands-in-browsers-is-it-time-to-rethink-bundling/
https://medium.freecodecamp.org/you-might-not-need-to-transpile-your-javascript-4d5e0a438ca
https://www.puncsky.com/blog/2015/01/13/understanding-reactor-pattern-for-highly-scalable-i-o-bound-web-server/


## DESIGN LEARNING

### LOGOS
https://99designs.com/blog/tips/types-of-logos/

### FORMS
https://uxplanet.org/float-label-pattern-in-ux-form-design-7ab5e33010ab
https://uxdesign.cc/design-better-forms-96fadca0f49c
https://www.nngroup.com/articles/form-design-placeholders/
http://govuk-elements.herokuapp.com/form-elements/#form-labels
https://www.smashingmagazine.com/2018/10/form-design-patterns-release/

### BUTTONS
https://uxplanet.org/7-rules-for-mobile-ui-button-design-e9cf2ea54556

### SPACING
https://www.justinmind.com/blog/minimalism-in-interactive-design-dreaming-of-a-white-space-xmas/

### DESIGN TOOL LEARNING
https://bezier.method.ac/

## PROGRAMMING LANGUAGES AND TECHNOLOGIES

### RUST
https://doc.rust-lang.org/book/2018-edition/index.
https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html

### TYPESCRIPT

### WASM
https://webassembly.org/

### FLUTTER
https://hackernoon.com/why-flutter-uses-dart-dd635a054ebf

### BROWSERS
https://ldnwebperf.org/events/progressive-web-apps-with-jake-archibald/


## PROGRAMMING TECHNIQUES
MVC
https://www.infoq.com/articles/no-more-mvc-frameworks 




## DIFFERENT CLI FORMATTING STYLES:

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
```

CHARACTER LINE ORIENTATION
```
Top:
"""
'''
``
***
^^^
```

```
Bottom:
___
...
,,,
```
```
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
