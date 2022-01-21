/*
TODO
-------------------------------------------------------------------------------
ring buffer 
assign a node (cache server, specifically) to buffer

assign a key to node
assign a hash value to node 
new node to buffer
remove node to buffer 
apply virtual node to buffer

DONE
-------------------------------------------------------------------------------
 */


trait NodeTrait {
    fn to_type(&self) -> &str;
}

struct NodeServer {
    url: String,
}

impl NodeTrait for  NodeServer {
    fn to_type(&self) -> &str {
        "node_server"
    }
}

struct NodeHash {
    key: String,
    value: String,
}

impl NodeTrait for NodeHash {
    fn to_type(&self) -> &str {
        "node_hash"
    }
}

struct ConsistentHash {
    buffer: Vec<Box<dyn NodeTrait>>,
}

impl ConsistentHash {
    pub fn new() -> Self {
        Self { buffer: Vec::new()}
    }

    pub fn add_server(&self, new_server: NodeServer) {
        self.add_node_to_buffer(new_server);
    }

    fn add_node_to_buffer(&self, arg: impl NodeTrait) {

    }

    pub fn get_value(&mut self) {

    }
}

fn main() {
    println!("Hello, world!");
}
