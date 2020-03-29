# future_test
Implemented code from async rust book

## Interesting point

I fixed the implementation of FutureTimer to spaw the thread after the first pooling call. By this way, 
we can test our Join and AndThen future structs.
