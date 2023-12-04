# Rate limiter: sliding window counter

# TODO
* divide time as units
* store the request count per unit 
* define the start and the end of the window
* calculate the average traffic within the window range
  - how to set up a viable data structure? 
* if the count is beyond the threshold, reject the request
* else accept and/or relay the request

# DONE