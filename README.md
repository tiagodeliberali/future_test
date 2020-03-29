# future_test
Implemented code from async rust book

## Interesting point

I fixed the implementation of FutureTimer to spaw the thread after the first pooling call. By this way, 
we can test our Join and AndThen future structs.

## Example output

<pre><font color="#A6E22E"><b>   Compiling</b></font> future_test v0.1.0 (/home/tiago/projetos/future_test)
<font color="#A6E22E"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.90s
<font color="#A6E22E"><b>     Running</b></font> `target/debug/main`
21:53:20.755042 [EXP-0] Tudo está async mesmo...
21:53:20.755092 [EXP-1] Mixed iniciado dentro do future
21:53:21.755753 [EXP-1] Mixed finaliado dentro do future
21:53:21.755923 [EXP-1] Mixed finalzado - 1s
21:53:23.755509 [EXP-2] TimerFuture finalizado - 3s
21:53:28.755628 [EXP-3] Join finalizado - 8s
21:53:33.755988 [EXP-4] AndThenFut finalizado - 13s
</pre>
