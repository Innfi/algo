pub struct ConsistentHash {}

impl ConsistentHash {
  pub fn new() -> Self {
    Self {  }
  }

  pub fn initial_runner(&self) -> String {
    String::from("start from here")
  }
}

//use std::collections::HashMap;

// trait NodeTrait {
//     fn to_type(&self) -> &str;
// }

// struct NodeServer {
//     url: String,
// }

// impl NodeTrait for  NodeServer {
//     fn to_type(&self) -> &str {
//         "node_server"
//     }
// }

// struct NodeHash {
//     key: String,
//     value: String,
// }

// impl NodeTrait for NodeHash {
//     fn to_type(&self) -> &str {
//         "node_hash"
//     }
// }

// struct ConsistentHash {
//     //test_map: HashMap<u32>,
//     //buffer: Vec<Box<dyn NodeTrait>>,
// }

// impl ConsistentHash {
//   pub fn check_runner() {
//     println!("ConsistentHash::check_runner")
//   }

//     // pub fn new() -> Self {
//     //     Self { 
//     //         //test_map: HashMap<u32>::new(),
//     //         buffer: Vec::new()
//     //     }
//     // }

//     // pub fn add_server(&self, new_server: NodeServer) {
//     //     self.add_node_to_buffer(new_server);
//     // }

//     // fn add_node_to_buffer(&self, arg: impl NodeTrait) {

//     // }

//     // pub fn get_value(&mut self) {

//     // }
// }
