# Rate limiter: fixed window counter 

# TODO
* divide time as units
* count incoming requests within current time unit
* if the count is beyond the threshold, reject the request
* else accept and/or relay the request

# DONE