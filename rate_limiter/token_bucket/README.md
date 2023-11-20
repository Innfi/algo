# Rate limiter: token bucket

# TODO

* characteristics
  - how is it different from the leaky bucket algorithm?

* define requirements: 
  - token usage per timestamp
  - total number of tokens
  - concurrency: available request per sec / min / hours
  - number of providers: 1, 2, or n?
  - external interfaces
  - token metadata?

# DONE
* implementations
  - provider must generate unique tokens
  - provider must provide tokens by FIFO order
  - if no tokens available in the bucket, provider must respond with error  