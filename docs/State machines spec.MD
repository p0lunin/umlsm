# State machines specification
This document describes specification of the implemented state machine conception.

## Overview
There are two core concepts in the implementation:
1. States.
2. Pseudo-states.
3. Transitions.

Any can be only one of state, pseudo-state or transition.

## States
State is a solid unit that holds invariant and possible some data related to the invariant.

### Simple State
Simple State is a state with one invariant and possible some data related to this invariant.

### Composite state
Composite state is a state with inner state. You can think of it like a nested state machine. Composite state have the same event list as with top-level state machine and can reuse it or create new event types. When event comes, top-level state machine will first handle it, then if from such state aren't exists a transition that handles the event, so the event will be passed to the inner state of the composite state and so on.

## Changing state
There are two ways to change state: initializing and entering. Main difference is that when state is entered, then entry point will be called. When state is initialized, entry point won't be called.

## Transitions
Transition is a function used to exit one state and enter another state. Transition _always_ calls exit points on the old state and entry point on the new state. Transition has one input state and one output state.
