use chrono::{offset::Utc, DateTime};
use juniper::FieldResult;
use juniper::RootNode;
use juniper::{GraphQLInputObject, GraphQLObject};
use std::collections::HashSet;
// use petgraph::algo::connected_components;
// use petgraph::prelude::*;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use rustorm::{DbError, FromDao, Pool, ToColumnNames, ToDao, ToTableName};
use std::time::{SystemTime, UNIX_EPOCH};

//
#[derive(GraphQLInputObject, Debug)]
struct NewHuman {
    name: String,
    identifier: String,
}

#[derive(GraphQLInputObject, Debug)]
struct NewResource {
    name: String,
    location: String,
}

#[derive(GraphQLInputObject, Debug)]
struct RecordInteraction {
    resource_id: i32,
    human_id: i32,
    timestamp: i32,
}

#[derive(GraphQLInputObject, Debug)]
struct RecordInfection {
    human_id: i32,
    timestamp: i32,
}

/////

#[derive(GraphQLObject)]
struct Human {
    id: i32,
    name: String,
    identifier: String,
}

#[derive(GraphQLObject)]
struct Resource {
    id: i32,
    name: String,
    location: Vec<f64>,
    interactions: Vec<Interaction>,
}

#[derive(GraphQLObject)]
struct Interaction {
    id: i32,
    resource_id: String,
    human_id: String,
    timestamp: String,
}

#[derive(GraphQLObject)]
struct Infection {
    id: i32,
    human_id: String,
    timestamp: String,
}

#[derive(GraphQLObject)]
struct Generic {
    r#type: String,
    message: String,
}

#[derive(GraphQLObject)]
struct ConnectedComps {
    message: String,
    groups: Vec<Vec<i32>>,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn human(id: String) -> FieldResult<Human> {
        Ok(Human {
            id: 1,
            name: "drbh".to_owned(),
            identifier: "identifier".to_owned(),
        })
    }
    fn graph(point_in_time: String) -> FieldResult<ConnectedComps> {
        let db_url = "sqlite://rbct.db";
        let mut pool = Pool::new();
        let mut em = pool.em(db_url).unwrap();

        let sql = "SELECT * FROM interaction WHERE timestamp > ".to_owned() + &point_in_time;
        println!("{:?}", sql);
        let actors: Result<Vec<for_retrieve::Interaction>, DbError> =
            em.execute_sql_with_return(&sql, &[]);

        let rows = actors.unwrap();
        // println!("{:#?}", actors);

        let mut resource_id = rows
            .clone()
            .iter()
            .map(|data| data.resource_id.clone())
            .collect::<Vec<String>>();

        let mut human_id = rows
            .clone()
            .iter()
            .map(|data| data.human_id.clone())
            .collect::<Vec<String>>();

        let set: HashSet<_> = resource_id.drain(..).collect(); // dedup
        resource_id.extend(set.into_iter());

        let set: HashSet<_> = human_id.drain(..).collect(); // dedup
        human_id.extend(set.into_iter());

        let resource_sql =
            "SELECT * FROM resource WHERE id IN (".to_owned() + &resource_id.join(",") + ")";
        let human_sql = "SELECT * FROM human WHERE id IN (".to_owned() + &human_id.join(",") + ")";

        println!("{:?}", resource_sql);
        println!("{:?}", human_sql);

        let resource_sql_rows: Result<Vec<for_retrieve::Resource>, DbError> =
            em.execute_sql_with_return(&resource_sql, &[]);

        // println!("{:#?}", resource_sql_rows);

        let human_sql_rows: Result<Vec<for_retrieve::Human>, DbError> =
            em.execute_sql_with_return(&human_sql, &[]);

        // println!("{:#?}", human_sql_rows);

        let mut graph: Graph<&str, f32, petgraph::Undirected> = Graph::new_undirected();

        let rrows = resource_sql_rows.unwrap();
        for res in &rrows {
            let a = graph.add_node(&res.name); // node with no weight
        }
        let hrows = human_sql_rows.unwrap();
        for res in &hrows {
            let a = graph.add_node(&res.name); // node with no weight
        }

        // println!("{:#?}", rrows);
        // println!("{:#?}", hrows);

        // NODES INDEXES ARE
        // RESOUCES N... LEN(RESOURCE) + HUMAN N ... LEN(HUMAN)
        // HUMAN X == NODEINDEXS[ LEN(RESOUCES)+X ]
        // RESOUCES X  == NODEINDEXS[ X ]

        for r in &rows {
            println!("{:?}", r);

            let mut human_pos = hrows
                .iter()
                .position(|x| r.human_id.parse::<i32>().unwrap() == x.id)
                .unwrap();

            let resource_pos = rrows
                .iter()
                .position(|x| r.resource_id.parse::<i32>().unwrap() == x.id)
                .unwrap();

            human_pos += rrows.len();

            // println!("{:?} {:?} {:?} ", r, resource_pos, human_pos);

            graph.extend_with_edges(&[(
                NodeIndex::new(human_pos),
                NodeIndex::new(resource_pos),
                0.5,
            )]);
        }

        // let scc = petgraph::algo::tarjan_scc(&graph); // recursive
        let scc1 = petgraph::algo::kosaraju_scc(&graph); // iterative

        // println!("{:#?}", scc);
        println!("{:#?}", scc1);

        let res = scc1
            .iter()
            .map(|x| {
                ///
                return x.iter().map(|y| y.index() as i32).collect::<Vec<i32>>();
            })
            .collect::<Vec<Vec<i32>>>();

        println!("{:?}", res);
        println!("{}", petgraph::dot::Dot::new(&graph));

        Ok(ConnectedComps {
            message: "graph".to_string(),
            groups: res, //petgraph::dot::Dot::new(&graph).to_string(),
        })
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    fn createHuman(new_human: NewHuman) -> FieldResult<Human> {
        println!("{:?}", new_human);

        let db_url = "sqlite://rbct.db";
        let mut pool = Pool::new();
        let mut em = pool.em(db_url).unwrap();

        let human = for_insert::Human {
            name: new_human.name.clone().into(),
            identifier: new_human.identifier.to_string(),
        };

        let actors: Result<Vec<for_retrieve::Human>, DbError> = em.insert(&[&human]);

        println!("{:?}", actors);

        match actors {
            Ok(res) => Ok(Human {
                id: res.first().unwrap().id,
                name: new_human.name.to_owned(),
                identifier: new_human.identifier.to_owned(),
            }),
            Err(_) => Ok(Human {
                id: 0,
                name: "failed".to_owned(),
                identifier: "failed".to_owned(),
            }),
        }
    }
    fn createResource(new_resource: NewResource) -> FieldResult<Resource> {
        println!("{:?}", new_resource);

        let db_url = "sqlite://rbct.db";
        let mut pool = Pool::new();
        let mut em = pool.em(db_url).unwrap();

        // new_resource.identifier.to_string()
        let resource = for_insert::Resource {
            name: new_resource.name.clone().into(),
            location: new_resource.location, //vec![32.32.to_string(), 53.42.to_string()],
        };

        let actors: Result<Vec<for_retrieve::Resource>, DbError> = em.insert(&[&resource]);
        println!("{:#?}", actors);

        match actors {
            Ok(res) => {
                // this mess makes the location string into a vec
                let loc = res
                    .first()
                    .unwrap()
                    .location
                    .split(",")
                    .collect::<Vec<&str>>();

                println!("{:?}", loc);

                let long: f32 = loc.first().unwrap().trim().parse().unwrap();
                let lat: f32 = loc.last().unwrap().trim().parse().unwrap();

                Ok(Resource {
                    id: res.first().unwrap().id,
                    name: res.first().unwrap().name.clone(),
                    location: vec![long.into(), lat.into()],
                    interactions: Vec::new(), //res.first().unwrap().interactions,
                })
            }
            Err(_) => Ok(Resource {
                id: 0,
                name: String::new(),
                location: Vec::new(),
                interactions: Vec::new(),
            }),
        }
    }
    fn recordInteraction(interaction: RecordInteraction) -> FieldResult<Interaction> {
        println!("{:?}", interaction);

        let db_url = "sqlite://rbct.db";
        let mut pool = Pool::new();
        let mut em = pool.em(db_url).unwrap();

        // new_resource.identifier.to_string()
        let resource = for_insert::Interaction {
            resource_id: interaction.resource_id.to_string(),
            human_id: interaction.human_id.to_string(),
            timestamp: interaction.timestamp.to_string(),
        };

        let actors: Result<Vec<for_retrieve::Interaction>, DbError> = em.insert(&[&resource]);
        println!("{:#?}", actors);

        match actors {
            Ok(res) => Ok(Interaction {
                id: res.first().unwrap().id,
                resource_id: res.first().unwrap().resource_id.clone(),
                human_id: res.first().unwrap().human_id.clone(),
                timestamp: res.first().unwrap().timestamp.to_string(),
            }),
            Err(_) => Ok(Interaction {
                id: 1,
                resource_id: "drbh".to_string(),
                human_id: "drbh".to_string(),
                timestamp: 241532.to_string(),
            }),
        }
    }
    fn recordInfection(infection: RecordInfection) -> FieldResult<Infection> {
        println!("{:?}", infection);

        let db_url = "sqlite://rbct.db";
        let mut pool = Pool::new();
        let mut em = pool.em(db_url).unwrap();

        // new_resource.identifier.to_string()
        let resource = for_insert::Infection {
            human_id: infection.human_id.to_string(),
            timestamp: infection.timestamp.to_string(),
        };

        let actors: Result<Vec<for_retrieve::Infection>, DbError> = em.insert(&[&resource]);
        println!("{:#?}", actors);

        match actors {
            Ok(res) => Ok(Infection {
                id: res.first().unwrap().id,
                human_id: res.first().unwrap().human_id.clone(),
                timestamp: res.first().unwrap().timestamp.to_string(),
            }),
            Err(_) => Ok(Infection {
                id: 1,
                human_id: "drbh".to_string(),
                timestamp: 241532.to_string(),
            }),
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

pub mod for_insert {
    use super::*;

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct Actor {
        pub first_name: String,
        pub last_name: String,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct Human {
        pub name: String,
        pub identifier: String,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct Resource {
        pub name: String,
        pub location: String,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct Interaction {
        pub resource_id: String,
        pub human_id: String,
        pub timestamp: String,
    }

    #[derive(Debug, PartialEq, ToDao, ToColumnNames, ToTableName)]
    pub struct Infection {
        pub human_id: String,
        pub timestamp: String,
    }
}

pub mod for_retrieve {
    use super::*;

    #[derive(Debug, FromDao, ToColumnNames, ToTableName)]
    pub struct Actor {
        pub actor_id: i32,
        pub first_name: String,
        pub last_name: String,
        pub last_update: DateTime<Utc>,
    }

    #[derive(Debug, FromDao, ToColumnNames, ToTableName)]
    pub struct Human {
        pub id: i32,
        pub name: String,
        pub identifier: String,
    }

    #[derive(Debug, FromDao, ToColumnNames, ToTableName)]
    pub struct Resource {
        pub id: i32,
        pub name: String,
        pub location: String,
    }

    #[derive(Debug, FromDao, ToColumnNames, ToTableName, Clone)]
    pub struct Interaction {
        pub id: i32,
        pub resource_id: String,
        pub human_id: String,
        pub timestamp: i32,
    }

    #[derive(Debug, FromDao, ToColumnNames, ToTableName)]
    pub struct Infection {
        pub id: i32,
        pub human_id: String,
        pub timestamp: i32,
    }
}

fn get_ms_time() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis() as i64
}
