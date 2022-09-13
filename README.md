# Rustycraft

This is educational project in early prototype phase and full of bad code, hardcode and lack of error handling.
Current goal is understanding workflow.

Project inspired by [TrinityCore](https://github.com/TrinityCore/TrinityCore)

The main goal is to create modern, distributed, scalable World of Warcraft server emulator. 

#### Current supported version of World of Warcraft: *9.2.0 build 43206*

### What's ready for the prototype
- [x] Basic login to the realm
- [x] Switch client from realm server to world server
- [x] Create a character
- [ ] Log into the world

#### Plans to alpha
- [ ] Think about design (Event sourcing? RPC? Main entities to deal with?)
- [ ] Basic world update loop
- [ ] Clients synchronisation
- [ ] Handle user registration
- [ ] Persistence

#### Plans to beta
- [ ] Distributed runtime
- [ ] Plugins for scripting world behaviour (dynamic linking? LUA?)
- [ ] Use suitable tools for database, messaging, e.t.c.
