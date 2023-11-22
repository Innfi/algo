use log;

const COUNTER_MAX: usize = 10;

pub struct Request {}

pub struct Handler {
  counter: usize,
}

impl Handler {
  pub fn new() -> Self {
    Self { counter: 0 }
  }

  pub fn run(&mut self) {
    loop {
      if self.counter >= COUNTER_MAX {
        log::debug!("threshold. continue");
        continue;
      }

      let req = self.receive_request().unwrap();

      self.increment_counter();
      let _ = self.relay_request(req);
    }
  }

  pub fn receive_request(&self) -> Result<Request, &'static str> {
    Ok(Request{}) //FIXME
  }

  fn increment_counter(&mut self) {
    self.counter += 1; //FIXME
  }

  fn relay_request(&self, _req: Request) -> Result<(), &'static str> {

    Ok(())
  }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  Ok(())
}