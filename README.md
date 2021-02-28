# Lever

Lever is a data platform for logistics. It's designed to be as "generic" as
possible and to serve pretty much any logistics use case in principle. Anything
from global supply chains to to running a small boutique manufacturing startup
to keeping track of your personal stuff.

The concepts and initial implementation (coming soon) that you find here should
be seen as modest first entries in what is hopefully a broader conversation. 

## Concepts

Lever is built to be general enough to handle just about any conceivable
logistics use case. That generality stems from Lever being based on just a few
simple concepts: [things], [events], and [actions]. These concepts correspond
directly to what I take to be the central concepts of logistics understood in
the most fundamental sense.

### Things

In Lever, a **thing** is any...well, *thing* that you want to have a digital
representation of. An item you've manufactured, an item that's been shipped to
you, an employee in your organization. Lever enables you to manage information
about an indefinite number of things from an indefinite number of domains and to
introduce new things into the system.

All things have these properties:

* A unique ID
* A kind, which can be any arbitrary string. Things can changed kinds over time
  (but retain their ID).
* A current status. The initial status of a thing is always `created` but that
  can change over time to be anything that fits the use case for that kind, e.g.
  a status might go from `created` to `in_transit` and later to `delivered`.
* A data payload that can be any arbitrary key-value data. That payload can
  change over time. A thing's data could include the thing's current location,
  a secret code name for the thing, the thing's product line, whatever.

Here's an example thing (represented as JSON):

```json
{
  "id": "59915392-598c-4de9-aa91-07c8ada206ba",
  "kind": "inventory_item",
  "status": "on_shelf",
  "data": {
    "product_line": "tickle_me_elmo",
    "warehouse": "Peoria, IL",
    "condition": "good",
    "serial_no": "zTg8AkCWMQqk7XwNLNqz9pWk"
  }
}
```

Things are never really removed from the system. They might be given a final
status like `sold` or `destroyed` but their status and history remain available
to the system.

### Events

An **event** in Lever is anything that happens to a thing, either a change in
the thing's status or data or both. Events have these properties:

* A unique ID
* A timestamp
* Event metadata, which is key-value data of whichever structure is required
  for the use case at hand.

Here's an example event:

```json
{
  "id": "3aff51af-85be-4e1a-bb86-f71ec0ade133",
  "timestamp": "2021-02-27T23:44:30.066722Z",
  "metadata": {
    "thing_id": "59915392-598c-4de9-aa91-07c8ada206ba",
    "retailer": "Target",
    "location": "Galveston, TX",
    "serial_no": "zTg8AkCWMQqk7XwNLNqz9pWk",
    "price": 32.21,
    "delta": {
      "status": "sold"
    }
  }
}
```

> All events in Lever pertain to one thing. It may be necessary to allow for
> events that pertain to several things or no things. For now I like the
> simplicity of this model but I'm not wedded to it.

[Handlers] define what happens when events occur.

#### History

In essence, things in Lever can be thought of as *event streams*. A thing's
creation in the system is its first event, and then each update is another
event. The full stream of events associated with a thing is called that thing's **history**. This gives Lever a built-in concept of *total auditing* across
all domains.

### Actions

An **action** in Lever is a named job that can be run against any thing or 
things in the system, akin to a background job. Actions are designed to be
triggered through the [Lever API][api], potentially ad hoc or at regular
intervals.

An action could be something like adding a data field to all things
that meet some condition, e.g. setting `defective` to `true` for all things
with the property `product_line = "aerostar"` and `factory = "Lowell, MA"` when
you discover a manufacturing defect. The actual logic called in an action is
defined by that action's [handler][handlers].

Actions have just three properties:

* A kind
* A timestamp
* A set of params that are passed to the handler

Here's an example action object:

```json
{
  "kind": "mark_defective",
  "timestamp": "2021-02-27T23:44:30.066722Z",
  "params": {
    "product_line": "aerostar",
    "factory": "Lowell, MA"
  }
}
```

### Handlers

There are two types of handlers in Lever: [event][events] handlers and
[action][actions] handlers.

Event handlers are custom logic triggered by events. They can act on all events
or just some defined subset. An example would be triggering a purchase against
the [Stripe] API when an event of kind `sale` occurs.

> Storing the event is technically a "handler" and the only one that can't be
> removed from the handler chain lest Lever fail to fulfill certain
> [guarantees].

Action handlers are similar but with some importance differences:

* They can be triggered via the [API], whereas event handlers are fired
  automatically
* They act using supplied parameters

#### Handler implementation

In the initial incarnation of Lever, all handlers are hard-coded into the
server and are mostly trivial examples, such as logging all actions and events
to stdout. Later, however, I'd like to allow for external handlers. With that
kind of setup, the Lever API would handle events and actions by firing off a
webhook, writing to a topic in a messaging system, sending a message over a
socket, or something of that sort.

## Guarantees

Lever is built to provide users of the system with a number of simple yet
powerful contracts:

* Every [thing][things] that has ever entered the system stays there. Its last
  available status and data aren't deleted.
* The full [history] of each thing is available at all times.
* Every [event][events] that happens is recorded and handled by all of the event
  [handlers] that apply to that event (any failures are logged).
* All actions are built to totally succeed or totally fail.

## API

Lever will provide a pretty straightforward HTTP+JSON API at first. Later, I
plan to add a [GraphQL]

## Use cases

Lever is meant to be such a general building block that it can address just
about any conceivable logistics use case.

## Integrations

The sky's the limit in terms of integrations with other systems. You can write
action handlers that talk to [Stripe]

## Federation

There's no reason in principle why different Lever systems couldn't be loosely
federated together. Different organizations could create their own handlers
that other orgs could trigger, coordinate information about specific things,
and more.

## Rationale

Logistics software has always been dominated by huge companies, archaic, opaque
platforms, and proprietary systems. But I don't see any good reason why this
domain shouldn't be brought in line with the contemporary Zeitgeist of
software development. I envision an ecosystem of logistics software built in the
open in a wide variety of languages and running on KuberNomaLambdaDuJour and
fodder for talks at edgy meetups. Because why not?!?

## Project status

Right now, the project is mostly conceptual. The actual platform is very much a
WIP but I'm aiming to have a proof of concept ready by mid-March 2021. It will
start out as a monolithic web server that just stores everything in [PostgreSQL]
but it will grow into much more than that over time.

[actions]: #actions
[api]: #api
[events]: #events
[graphql]: https://graphql.org
[handlers]: #handlers
[history]: #history
[lob]: https://lob.com
[postgresql]: https://postgresql.org
[stripe]: https://stripe.com
[things]: #things