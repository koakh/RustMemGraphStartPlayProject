use rsmgclient::{ConnectParams, Connection, ConnectionStatus, SSLMode, Value};

mod model;

use model::{Developer, NodeTrait, Technology};

use crate::model::NodeType;

fn main() {
    // Connect to Memgraph
    let connect_params = ConnectParams {
        host: Some(String::from("192.168.1.1")),
        port: 7687,
        sslmode: SSLMode::Disable,
        ..Default::default()
    };
    let mut connection = Connection::connect(&connect_params).unwrap();

    // Check if connection is established.
    let status = connection.status();

    if status != ConnectionStatus::Ready {
        println!("Connection failed with status: {:?}", status);
        return;
    } else {
        println!("Connection established with status: {:?}", status);
    }

    // Clear the graph.
    connection
        .execute_without_results("MATCH (n) DETACH DELETE n;")
        .unwrap();
    if let Err(e) = connection.commit() {
        println!("Error: {}", e);
    }

    let indexes = vec![
        "CREATE INDEX ON :Developer(id);",
        "CREATE INDEX ON :Technology(id);",
        "CREATE INDEX ON :Developer(name);",
        "CREATE INDEX ON :Technology(name);",
    ];

    let developer_nodes = vec![
        "CREATE (n:Developer {id: 1, name:'Andy'});",
        "CREATE (n:Developer {id: 2, name:'John'});",
        "CREATE (n:Developer {id: 3, name:'Michael'});",
        "CREATE (n:Developer {id: 4, name:'Mário Monteiro'});",
    ];

    let technology_nodes = vec![
        "CREATE (n:Technology {id: 1, name:'Memgraph', description: 'Fastest graph DB in the world!', createdAt: Date()})",
        "CREATE (n:Technology {id: 2, name:'Rust', description: 'Rust programming language ', createdAt: Date()})",
        "CREATE (n:Technology {id: 3, name:'Docker', description: 'Docker containerization engine', createdAt: Date()})",
        "CREATE (n:Technology {id: 4, name:'Kubernetes', description: 'Kubernetes container orchestration engine', createdAt: Date()})",
        "CREATE (n:Technology {id: 5, name:'Python', description: 'Python programming language', createdAt: Date()})",
    ];

    let relationships = vec![
        "MATCH (a:Developer {id: 1}),(b:Technology {id: 1}) CREATE (a)-[r:LOVES]->(b);",
        "MATCH (a:Developer {id: 2}),(b:Technology {id: 3}) CREATE (a)-[r:LOVES]->(b);",
        "MATCH (a:Developer {id: 3}),(b:Technology {id: 1}) CREATE (a)-[r:LOVES]->(b);",
        "MATCH (a:Developer {id: 1}),(b:Technology {id: 5}) CREATE (a)-[r:LOVES]->(b);",
        "MATCH (a:Developer {id: 2}),(b:Technology {id: 2}) CREATE (a)-[r:LOVES]->(b);",
        "MATCH (a:Developer {id: 3}),(b:Technology {id: 4}) CREATE (a)-[r:LOVES]->(b);",
    ];

    for index in indexes {
        connection.execute_without_results(index).unwrap();
    }
    if let Err(e) = connection.commit() {
        println!("Error: {}", e);
    }

    for developer_node in developer_nodes {
        connection.execute_without_results(developer_node).unwrap();
    }
    if let Err(e) = connection.commit() {
        println!("Error: {}", e);
    }

    for technology_node in technology_nodes {
        connection.execute_without_results(technology_node).unwrap();
    }
    if let Err(e) = connection.commit() {
        println!("Error: {}", e);
    }

    for relationship in relationships {
        connection.execute_without_results(relationship).unwrap();
    }
    if let Err(e) = connection.commit() {
        println!("Error: {}", e);
    }

    let create_developer = Developer {
        id: 5,
        name: String::from("Alexandre Monteiro"),
    };
    let created_developer = create_developer
        .create(&mut connection, &create_developer)
        .unwrap();
    println!("created_developer: {:?}", created_developer);

    // Fetch the graph.
    let columns = connection.execute("MATCH (n)-[r]->(m) RETURN n, r, m;", None);
    println!("Columns: {}", columns.unwrap().join(", "));

    while let Ok(result) = connection.fetchall() {
        for record in result {
            for value in record.values {
                match value {
                    Value::Node(node) => {
                        println!("Node: {}", node);
                        println!("Node id: {}", node.id);
                        println!("Node labels: {:?}", node.labels);
                        println!("Node labels: {:?}", node.label_count);
                        println!("Node properties: {:?}", node.properties);
                        println!("Node properties: {:?}", node.properties.get("id"));
                        println!("Node properties: {:?}", node.properties.get("name"));
                        println!("Node properties: {:?}", node.properties.get("description"));
                        println!("Node properties: {:?}", node.properties.get("createdAt"));

                        // let is_developer = node.labels.contains(&NodeType::Developer.to_string());
                        // if NodeType::is_type(&node, NodeType::Developer) {
                        //     let developer = Developer::new(node.id, &node.properties);
                        //     println!("developer id: {}, name: {}", developer.id, developer.name);
                        // }

                        match NodeType::from(&node) {
                            NodeType::Developer => {
                                let developer = Developer::new(node.id, &node.properties);
                                println!("developer id: {}, name: {}", developer.id, developer.name);
                            }
                            NodeType::Technology => {
                                let technology = Technology::new(node.id, &node.properties);
                                println!("technology id: {}, name: {}, technology: {}", technology.id, technology.name, technology.description);
                            }
                            _ => {}
                        }

                        if let Value::Int(object) = node.properties.get("id").unwrap_or(&Value::Int(0)) {
                            let id = *object;
                            println!("id: {id}");
                        }

                        let property_value = node.properties.get("name").unwrap();
                        match property_value {
                            Value::Null => {}
                            Value::Bool(_v) => {}
                            Value::Int(v) => {
                                println!("-> detected int: {}", v)
                            }
                            Value::Float(v) => {
                                println!("-> detected float: {}", v)
                            }
                            Value::String(v) => {
                                println!("-> detected string: {}", v)
                            }
                            Value::List(_v) => {}
                            Value::Date(_v) => {}
                            Value::LocalTime(_v) => {}
                            Value::LocalDateTime(_v) => {}
                            Value::Duration(_v) => {}
                            Value::Map(_v) => {}
                            _ => {}
                        }
                    }
                    Value::Relationship(edge) => println!("Edge: {}", edge),
                    value => println!("Value: {}", value),
                }
            }
        }
    }
    // Close the connection.
    connection.close();
}
