# Rate limiter: leaky bucket

* characteristics
  - bursting traffic refined to a consistent rate traffic
  - fixed size queue, fixed retrival count
  - parameters: bucket size, tick ratio, retrival count
  - the sender has to wait for the request to be delivered

# TODO

* implentations
  - accept traffic under the physical limit
  - retrieve traffic as a fixed rate

# DONE

