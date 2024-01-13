# Time

Time advances in 100ms increments. The clock is a shared mutable resource.

Here's how the event loop works:

- Player issues a command
- command is optionally broken down into sub-commands, eg
  - "attack goblin with bow" becomes: draw arrow -> nock -> draw bow -> aim -> release 
  - "move to click" becomes: pathfind -> (sequence of move actions)
- subcommands are enqueued 
- first command starts, then
- each tick until the command(s) complete or are interrupted:
  - get all newly begun commands
    - concurrently update any state applied during the command
  - for each command just completed
    - update state / issue events as required
    - use decision tree to build new command if none are enqueued
    - otherwise, pop next command in queue to current command
  - if the player has no current command or it is interrupted, await player input
    - in the case of interrupt
      - update UI / print interrupt log message 
      - prompt to either confirm / proceed, or abort
      - if aborting
        - update any state as required
        - clear current and any queued commands
        - wait at least 1 tick for reaction
    - if there is no current command
        - await new command
  - determine tick processing order
  - loop through agents
    - for each other agent, check if there is at least one current command  
      - if a command is interrupted, use behaviour tree to decide whether to continue
      - if there is no current command, use behaviour tree to determine next command
  - loop through time-based effects & usage (torches, disease, etc)
    - send events / update state if necessary
