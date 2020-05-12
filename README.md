# RESOURCE BASED CONTRACT TRACING

Contact tracing is traditionally focused on tracking the individual, thier movements and proximity to other indiviudals.

As these interactions are recorded they are stored in such a way that the graph of relationships can be traversed and you can find all of the potiental indiviudals who may have varying degrees of contact.

This project is a alternative method for tracting contact. This method is unothordox since we focus on contact via shared resouces and not the individual themselves.

# Shared resouces

A shared resource is any item/building that more than one person interacts with. A shared resouce needs to be registered and named. We then watch this resouce for contact interactions or probably contact interactions.

Lets say we have a shared resouce of a bathroom door knob. This bathroom is in a large office building and serves a single floor. Imagaine there is a camera is in the hallway nearby the entrence to this bathroom. We can use that camera to record the interactions with our doorknob resource.

Everytime a person interacts with the doorknob we'd log that contact interaction as WHO TOUCHED, WHICH RESOURCE, WHEN.

An added feature of tracking resouces - is that it is privacy perserving and actionable. If we know that a shared resoruce is possible dangerous or infected. We can respond accordingly; clean the surface or stop future interactions.

It is privacy preserving because we can alert people about an infected item - not an individual.

#

Register a human

```graphql
mutation {
	createHuman(newHuman: { name: "drbh", identifier: "123456789" }) {
		id
		name
		identifier
	}
}
```

Register a resource

```graphql
mutation {
	createResource(
		newResource: {
			name: "Floor 1 - Bathroom A"
			location: "-43.00001, 43.4514300000002"
		}
	) {
		id
		name
		location
		interactions {
			id
		}
	}
}
```

Record an interaction

```graphql
mutation {
	recordInteraction(
		interaction: { resourceId: 1, humanId: 1, timestamp: 1588625542 }
	) {
		id
		humanId
		timestamp
	}
}
```

Record an infection

```graphql
mutation {
	recordInfection(infection: { humanId: 1, timestamp: 1588625542 }) {
		id
		humanId
		timestamp
	}
}
```

Subscribe to alerts [ Webhook ]

Subscribe to alerts [ Websocket ]
