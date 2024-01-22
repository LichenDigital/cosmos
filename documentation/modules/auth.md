< **[Back to documentation main](../documentation.md)**
___

# *Auth*
Auth handles both authorization and authentication.

## Authorization

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

User manages users and user groups. It has a built in permission system that is built on top of cosmos's [event](#event) system. User structures and permission systems can be built to fit custom organizational structures. Each user and group are assigned a unique id using cosmos's id module, and can have roles associated with them that are used in the permission system for a specific (and familiar) way of managing permissions. Permissions can also be granted or revoked base on other conditions through the cosmos `event` system.

## Authentication