# Status
This project is in active development, and I'm not organized enough to have a bunch of issues and such yet. 

## Todo:
Rebuild/refactor around the entity ID model
 - Use world object to store entities
 - use enitity id's in hash for quick lookups
 - use entity id's as source of truth, no longer considering vec indices
 - refactor game manager, spawn manager, and events to use entity id's
 - this will lead to next todo of simplifying the input system