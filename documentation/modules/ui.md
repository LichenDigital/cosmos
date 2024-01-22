< **[Back to documentation main](../documentation.md)**
___

# *UI*

The UI (User Interfaces) module exposes the power of cosmos visually. It is used to create custom user interfaces accross multiple technologies, from something as basic as a character LCD, to augmented projections. It is both used internally for cosmos's own UI, for managing modules and data, and also for custom UIs that can be built for the web, native apps, and 2D and 3D game engine interfaces.

The web UI contains building blocks for building pages and is broken down into sub-types. When developing the SCSS and CSS for the UI, the focus will be to make it as modular and efficient as possible. Structural and visual code will be separated as much as possible. Extensive use of variables, mixins, functions and the like will also be used to make maintenance and use of the code as simple and intuitive as possible.

UI protocol will have the ability to collect, send, and execute or replicate the following information:

  - Interaction  (can have multiple people working on the same UI)
    - Location
    - Movement of interaction (scroll etc.)
    - Type (pressure (can be negative) etc.)
    - Value

Have user interface elements be able to affect other elements with relationships.

* One use case for this is being able to have other elements be able to flow around other elements. For instance there could be something like a flow factor which would give an item a presence when moving through a space and be able to affect elements in certain directions. Picture placing an interface item like an image somewhere and being able to have it force other user interface elements to flow around it. Similar to how float works, but not just affecting things in a top to bottom left to right or right to left way.
* Have elements be able to have constraints. 


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
The command line interface is a way of interacting with cosmos on the command line of linux, Mac OS, and Windows. It directly interfaces with the cosmos protocol interpreter in order to run modules and pass them data from the command line.

**Command Line Implementation Ideas**
Loading and progress spinners and bars will be inspired by https://www.npmjs.com/package/cli-spinners and other


## CLI Name Representations

```
 __   __    __  _  _   __    __
/    /  \  (_   |\/|  /  \  (_  
\__  \__/  __)  |  |  \__/  __)

 
   o---o     o---o      o--o    o     o     o---o      o--o   
  /         /     \    |        | \ / |    /     \    |       
 o         o       o    o-o     |  O  |   o       o    o-o    
  \         \     /        |    |     |    \     /        |   
   o---o     o---o     o--o     o     o     o---o     o--o    


  ____    ___    ____    __  __    ___    ____  
 / ___|  / _ \  / ___|  |  \/  |  / _ \  / ___| 
| |     | | | | \___ \  | |\/| | | | | | \___ \ 
| |___  | |_| |  ___) | | |  | | | |_| |  ___) |
 \____|  \___/  |____/  |_|  |_|  \___/  |____/


 ______     ______     ______     __    __     ______     ______    
/\  ___\   /\  __ \   /\  ___\   /\ "-./  \   /\  __ \   /\  ___\   
\ \ \____  \ \ \/\ \  \ \___  \  \ \ \-./\ \  \ \ \/\ \  \ \___  \  
 \ \_____\  \ \_____\  \/\_____\  \ \_\ \ \_\  \ \_____\  \/\_____\ 
  \/_____/   \/_____/   \/_____/   \/_/  \/_/   \/_____/   \/_____/ 
                                                                    


   _____      ______       ____     __   __      ______      ____
  / ____|    / ____ \     / ___|   |  \_/  |    / ____ \    / ___|
 / /        / /    \ \   | (___    | |\_/| |   / /    \ \  | (___ 
| |        | |      | |   \___ \   | |   | |  | |      | |  \___ \ 
 \ \____    \ \____/ /     ___) |  | |   | |   \ \____/ /    ___) |
  \_____|    \______/     |____/   |_|   |_|    \______/    |____/  
       
```

#