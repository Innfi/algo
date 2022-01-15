/**
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

pub struct NodeKey {
    value: String,
}

pub struct NodeServer {
    url: String,
}

pub struct Controller {
    buffer: Vec<String>,
    nodeCount: u32,
}

impl Controller {
    pub fn addServer(&mut self, server: NodeServer) {
        self.buffer.push(server.url);
    }
}

fn main() {
    println!("Hello, world!");
}
