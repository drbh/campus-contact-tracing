```graphql
mutation {
	createHuman(newHuman: { name: "David Holtz", identifier: "9999" }) {
		id
		name
		identifier
	}
}
```

```graphql
mutation {
	createHuman(newHuman: { name: "drbh", identifier: "123456789" }) {
		id
		name
		identifier
	}
}
```

```graphql
mutation {
	createHuman(newHuman: { name: "ness", identifier: "123456789" }) {
		id
		name
		identifier
	}
}
```

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

```graphql
mutation {
	createResource(
		newResource: {
			name: "Doorknob Z"
			location: "-44.00001, 43.4514300000002"
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

```graphql
mutation {
	recordInteraction(
		interaction: { resourceId: 1, humanId: 2, timestamp: 1588628542 }
	) {
		id
		humanId
		timestamp
	}
}
```

```graphql
mutation {
	recordInteraction(
		interaction: { resourceId: 2, humanId: 2, timestamp: 1589628542 }
	) {
		id
		humanId
		timestamp
	}
}
```
