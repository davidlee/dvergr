# approach

GOAP seems right. Some extensions, e.g. from BT literature feel appropriate too, eg
- sequences / subtasks
- instant vs monitor conditions

some of the advice I've come across:
- limit depth of plans, and breadth of state (a few enums > many booleans)
- extract complexity to other systems, eg. a dedicated investigation system
- virtual agents for e.g. "investigation" and "squad tactics" can better assign roles
  - just have the main planner wait while they take control / issue commands
- separate action consequences from what they promise for bonus fun

# command grammar

# terminology

AwaitState
- enum resource:
  - PlayerInput
  - PlayerAgent
  - OtherAgent
  - Animate(e)


Sensor: a system that updates agentstate
AgentState: state of an agent, hash of key/value pairs 
- keys: ASKey
- values: ASVal of bool / u8 / enum(ASState)

Condition:
  Precondition: must be satisfied before starting a Step
  Monitor: must remain satisfied each tick or a Step is aborted
  - has a key/value pair
  
Motive: hash of typed keys -> values(bool / u8 / enum); used to weight / select goals
Goal: 
 - consists of 
   - a desired change to agentstate (k,v)
   - weight / multiplier / formula
   - effect(s)
Planner: creates and selects Plans
Plan: series of steps
Step: 
  - part of a plan (high level, eg GoToPosition, Get, UseSmartObject, Attack, Carry(item: Entity, position), ..
  - has
    - (pre)conditions
    - promises (what effect the planner thinks it has)
    - effects (what actually happens to agentstate)
Resolver:
  - turns a Step into one or (usually) a sequence of Actions, and enqueues them on an Agent
  - may eg. select targets, do pathfinding, or engage other systems 
  - "takes over" from GOAP until it fails or completes

Runner
 - a system which runs each tick
  - for each Agent
    - if missing a Plan
     - run the Planner
       - build weighted list of goals
       - for each goal, build weighted list of valid Steps whose preconditions can be met
        - steps may have preconditions which create additional Steps
       - select best path to Goal attainment as Plan
        - or Idle as fallback
       - store Plan and make first Step current
    - with current Step
      - validate conditions
          - abort (clear Actions and Plan) on failure 
      - init Resolver for Step
        - if Step is not yet resolved
          - enqueue Actions
        - otherwise, if there is a current Action
          - with the Resolver 
            - validate Action
              - abort (clear Actions and Plan) on failure 
            - execute Action
        - otherwise, if ActionQueue is not empty
          - pop current Action from ActionQueue
          - on success
            - if any steps remain
              - proceed to next Step in Plan and release control
        - otherwise, Step is complete
          - apply any effects
          - if any Step remains
            - proceed to next step
          - otherwise, Plan is complete
            - apply any Effects
            - if Goal is reached, apply any Effects
            - re-run Planner

Action: 
 - a low-level command 
 - contains all the state required to actually run the system, eg reference to a Locus, Direction, Creature ...

Conditions
  precondition
  monitor



Inventory(Verb, item: Entity, target: Option<Entity>)
UseItem(item: Entity, target: Option<Entity>)
Move(Verb, Direction)
Attack(Kind, Verb, target: Entity,)

# actions

enum MotiveKey
hashmap Motive

vvv

Sensor -> Blackboard

hashmap BlackBoard 
* propName, propValue

enum propName impl Display

enum propValue {
  boolean
  atLeast u8
  atMost u8
  status propStatusEnum
}

enum propStatus {
  Idle,
  Thinking,
  Peaceful,
  Alerted,
  Combat,
  Fleeing,
}


Condition(key, )

enum Condition {
  boolean(Bool),
  atLeast(u8),
  atMost(u8),i
}

Goal {
  desiredState: Condition
}

Task

Action - an executable command


ActionPlugin






to think about
- how to model needs / motives to ensure coherence with game state: can the planner access a 
  component without borrow checker issues? 
- should actions in GOAP be super lean and be fleshed out into more detail at execution time? 
  eg a move action might want a reference to origin and destination Locus (with facing, etc)
  these feel like different types, which need unique names & not to be confused.

implementation order: 
- should we start with player commands, subtasks & action queue?
- pathfinding: if we use A* for both planner and movement pathfinding, do this early
https://github.com/TheAlgorithms/Rust/blob/master/src/graph/astar.rs
https://github.com/rschifflin/astar-rust/blob/master/astar-synchronous.rs
https://blog.logrocket.com/pathfinding-rust-tutorial-examples/

# behaviour trees

https://www.youtube.com/@petterogren7535
https://github.com/Sollimann/bonsai
https://github.com/hyranno/bevior_tree/tree/master
https://www.gamedeveloper.com/programming/behavior-trees-for-ai-how-they-work
https://www.gameaipro.com/GameAIPro/GameAIPro_Chapter06_The_Behavior_Tree_Starter_Kit.pdf

# goap
https://www.louissimons.com/action-planning-part-2/
http://www.gameaipro.com/GameAIPro2/GameAIPro2_Chapter13_Optimizing_Practical_Planning_for_Game_AI.pdf
https://web.archive.org/web/20141217015739/http://alumni.media.mit.edu/~jorkin/gdc2006_orkin_jeff_fear.pdf


BehaviourTreeBuilder

BehaviourStatus::
  None
  Running
  Suspended
  Success
  Failure
  
Condition::
  Instant
  Monitor // each tick

Behaviour
  Status
  init
  tick -> status
  terminate(status)
  
Decorator

Composites
  Sequences // and; 
  Selectors // or;
    Filter
    Precondition
  Parallels

--- GOAP ---

State / blackboard
  k/v
