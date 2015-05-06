extern crate etcd;

use etcd::{Client, Error};

fn client() -> Client {
    Client::new("http://etcd:2379").unwrap()
}

#[test]
fn lifecycle() {
    let client = client();

    // Creating a key

    let create_response = client.create("/foo", "bar", Some(60)).unwrap();

    assert_eq!(create_response.action, "create".to_string());
    assert_eq!(create_response.node.value.unwrap(), "bar".to_string());
    assert_eq!(create_response.node.ttl.unwrap(), 60);

    // Getting a key

    let get_response = client.get("/foo", false, false).ok().unwrap();

    assert_eq!(get_response.action, "get".to_string());
    assert_eq!(get_response.node.value.unwrap(), "bar".to_string());
    assert_eq!(get_response.node.ttl.unwrap(), 60);

    // Creating a key fails if it already exists

    match client.create("/foo", "bar", None).err().unwrap() {
        Error::Etcd(error) => assert_eq!(error.message, "Key already exists".to_string()),
        _ => panic!("expected EtcdError due to pre-existing key"),
    };

    // Setting a key

    let set_response = client.set("/foo", "baz", None).ok().unwrap();

    assert_eq!(set_response.action, "set".to_string());
    assert_eq!(set_response.node.value.unwrap(), "baz".to_string());
    assert!(set_response.node.ttl.is_none());

    // Updating a key

    let update_response = client.update("/foo", "blah", Some(30)).ok().unwrap();

    assert_eq!(update_response.action, "update".to_string());
    assert_eq!(update_response.node.value.unwrap(), "blah".to_string());
    assert_eq!(update_response.node.ttl.unwrap(), 30);

    // Deleting a key

    let delete_response = client.delete("/foo", false).ok().unwrap();

    assert_eq!(delete_response.action, "delete");

    // Updating a key fails if it doesn't exist

    match client.update("/foo", "bar", None).err().unwrap() {
        Error::Etcd(error) => assert_eq!(error.message, "Key not found".to_string()),
        _ => panic!("expected EtcdError due to missing key"),
    };

    // Creating a directory

    let create_dir_response = client.create_dir("/dir", None).ok().unwrap();

    assert_eq!(create_dir_response.action, "create".to_string());
    assert!(create_dir_response.node.dir.unwrap());
    assert!(create_dir_response.node.value.is_none());

    // Deleting an empty directory

    let delete_dir_response = client.delete_dir("/dir").ok().unwrap();

    assert_eq!(delete_dir_response.action, "delete");

    // Getting keys within directories (recursive listing)

    client.set("/foo", "bar", None).ok();
    client.set("/dir/baz", "blah", None).ok();

    let non_recursive_get_response = client.get("/", false, false).ok().unwrap();

    assert_eq!(non_recursive_get_response.node.dir.unwrap(), true);

    let non_recursive_nodes = non_recursive_get_response.node.nodes.unwrap();

    assert_eq!(non_recursive_nodes[0].clone().key.unwrap(), "/foo".to_string());
    assert_eq!(non_recursive_nodes[0].clone().value.unwrap(), "bar".to_string());
    assert_eq!(non_recursive_nodes[1].clone().key.unwrap(), "/dir".to_string());
    assert_eq!(non_recursive_nodes[1].clone().dir.unwrap(), true);

    let recursive_get_response = client.get("/", false, true).ok().unwrap();
    let recursive_nodes = recursive_get_response.node.nodes.unwrap();

    assert_eq!(
        recursive_nodes[1].clone().nodes.unwrap()[0].clone().value.unwrap(),
        "blah".to_string()
    );

    client.delete("/foo", false).ok();
    client.delete("/dir/baz", false).ok();
    client.delete_dir("/dir").ok();
}

#[test]
fn leader_stats() {
    client().leader_stats().unwrap();
}

#[test]
fn self_stats() {
    client().self_stats().unwrap();
}
