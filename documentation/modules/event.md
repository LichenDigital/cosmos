< **[Back to documentation main](../documentation.md)**
___

# *Event*
The cosmos event system is designed to handle building complex reactive systems. Integrated systems for monitoring the event system are included in the [monitor](#monitor) module. Events are comprised of a few simple elements: [triggers](#triggers) and [actions](#actions) and the ubiqutous [id](#id) and [groups](#group).

## Triggers
Trigges are conditions that cosmos watch then initiate coresponding actions. An event may have multiple triggers, and muliple corresponding actions. Triggers are themselves an entity that cosmos tracks, and are grouped together, therefore making matching associated events easier.

For example, if two events are registered with cosmos, with the same trigger, they are associated through their triggers. So one trigger is stored, and is associated with muliple events. In this way, not only as mentioned above, does it make cosmos have to work less since it can watch one trigger instead of two of them, but it also makes it possible to edit the same trigger on multiple events. If the trigger of an event is edited, and it matches other triggers, you can update a selection, or all of them. Triggers themselves are grouped using cosmos groups. Also using this strategy, if an event is registered that matches another event, then the event can still be registered, but is only linked to the other event for the details of the trigger and actions. There could also be a setting in the event system that could discard events that match other events, instead of registering them and pointing them at the other same event. The difference between the two events would be the id, groups, and time that it was registered, and potentially the history too.

Triggers can be any condition that cosmos can evaluate and match. So they can also match actions. This allows for chaining events together. So one event can trigger an action, which then triggers another action, etc. They can also watch other events based on their id or group. This means that there can be event chaining that can watch for activity of a particular event or group of events.

## Actions
Actions are the commands that cosmos runs on the triggering of an event. These commands are cosmos running modules, so an action could be used not only in a familiar way through to trigger something happening visually in a user interface, but they can be used to run any command, so their useage is broad and deep. This means that it can be used for everything from storage changes, to user management, and transportation of data. 

## Event Managment
Events are stored like other data in the cosmos storage module. Here they can be created, read, updated, deleted like anything else. This includes both singular edits as well as bulk edits. They can like other pieces of data have other events associated with them like an expiration (delete this id or group when X time has elapsed etc.), transformation (the event changes its action based on the time of day that the event is triggered) etc. So chainging allows for very complex structures to be able to be built simply

## Event Visualization
Event visualization is handled through the [visualize](#visualize) module. This module leverages the [UI](#ui) module to create a way to visualize events, their connection, and realtime flow. This helps to see what is going on and debug event flow.