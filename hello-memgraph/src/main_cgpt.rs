use rsmgclient::{ConnectParams, Connection, ConnectionStatus, SSLMode, Value};

// Define your struct
#[derive(Debug)]
struct Person {
    id: i64,
    name: String,
    age: i16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Example query to fetch a person node
    let result = connection.execute("MATCH (p:Person) RETURN p.id, p.name, p.age;", None)?;

    // Iterate through the result set
    for record in result {
        let record = record?;
        // Extract the properties of the Person node and cast them to the correct types
        let id = match record.get::<i64>(0)? {
            Value::Int(val) => val,
            _ => return Err("ID is not an integer.".into()),
        };

        let name = match record.get::<String>(1)? {
            Value::String(val) => val,
            _ => return Err("Name is not a string.".into()),
        };

        let age = match record.get::<i64>(2)? {
            Value::Int(val) => val as i16,
            _ => return Err("Age is not an integer.".into()),
        };

        // Construct your struct
        let person = Person { id, name, age };

        println!("{:?}", person);
    }

    // Close the connection.
    connection.close();

    Ok(())
}
