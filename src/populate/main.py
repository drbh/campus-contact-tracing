import requests
import urllib.request
import random

word_url = (
    "http://svnweb.freebsd.org/csrg/share/dict/words?view=co&content-type=text/plain"
)
response = urllib.request.urlopen(word_url)
long_txt = response.read().decode()
words = long_txt.splitlines()

upper_words = [word for word in words if word[0].isupper()]
name_words = [word for word in upper_words if not word.isupper()]
one_name = " ".join([name_words[random.randint(0, len(name_words))] for i in range(2)])
headers = {}


def rand_name():
    name = " ".join([name_words[random.randint(0, len(name_words))] for i in range(2)])
    return name


def run_query(
    query,
):  # A simple function to use requests.post to make the API call. Note the json= section.
    request = requests.post(
        "http://127.0.0.1:8080/graphql", json={"query": query}, headers=headers
    )
    if request.status_code == 200:
        return request.json()
    else:
        raise Exception(
            "Query failed to run by returning code of {}. {}".format(
                request.status_code, query
            )
        )


def create_a_human(name, identifier):
    m = """
    mutation {{
        createHuman(newHuman: {{ name: "{name}", identifier: "{identifier}" }}) {{
            id
            name
            identifier
        }}
    }}
    """.format(
        name=name, identifier=identifier
    )
    return m


def create_a_resource(name):
    f = """
    mutation {{
        createResource(
            newResource: {{
                name: "{name}"
                location: "-43.00001, 43.4514300000002"
            }}
        ) {{
            id
            name
            location
            interactions {{
                id
            }}
        }}
    }}
    """.format(
        name=name,
    )
    return f


def create_an_interaction(resourceId, humanId):
    m = """
    mutation {{
        recordInteraction(
            interaction: {{ resourceId: {resourceId}, humanId: {humanId}, timestamp: 1588625542 }}
        ) {{
            id
            humanId
            timestamp
        }}
    }}
    """.format(
        resourceId=resourceId, humanId=humanId
    )
    return m


resouces = 5
peeps = 100


# create alot of people
for i in range(0, peeps):
    request = create_a_human(rand_name(), str(random.randint(1000000, 1010000000)))
    run_query(request)

# create a couple shared resources
for i in range(0, resouces):
    new_name = name_words[random.randint(0, len(name_words))]
    request = create_a_resource(new_name)
    # request = create_a_resource(rand_name(), str(random.randint(1000000, 1010000000)))
    run_query(request)

# create way more interactions
for i in range(0, 10_000):
    rand_resource_id = random.randint(0, resouces)
    rand_human_id = random.randint(0, peeps)
    request = create_an_interaction(rand_resource_id, rand_human_id)
    run_query(request)
