## Rust web frameworks benchmarks

This is totally unscientific and probably pretty useless. In real world business
logic would dominate on performance side. But in any case. i took several web frameworks
for rust and used theirs *hello world* example. 

List of frameworks:

* [Actix](https://github.com/actix/actix-web)
* [Gotham](https://gotham.rs)
* [Iron](http://ironframework.io)
* [Rocket](https://rocket.rs)
* [Shio](https://github.com/mehcode/shio-rs)
* [tokio-minihttp](https://github.com/tokio-rs/tokio-minihttp) (not really a framework, but it is fast)

All projects are compiled with `release` parameter. I didn't test single
thread performance for iron and rocket. I ran all tests on my MacBook Pro with 2.9Gh i7 
with 4 physical cpus and 8 logical cpus. As a testing tool i used *wrk* and
following commands

`wrk -t20 -c100 -d10s http://127.0.0.1:8080/`

`wrk -t20 -c100 -d10s http://127.0.0.1:8080/ -s ./pipeline.lua --latency -- / 128`

Some notes about benchmarks. 

All projects are compiled with release parameter.

I got best performance for sync frameworks with 8 threads, other number of 
threads always gave me worse performance. *Iron* could handle piplined 
requests with lower performace. Interestingly, *Rocket* completely failed in pipelined test.

There are two reasons why i tested pipelined request. First, it is just fun to
see this huge numbers of processed requests :) Second, it is kind of 
pre-requisite for *HTTP/2.0* support.

For asynchronous frameworks i wanted to see how multithreading influence 
performance, but multithreading is actually harder than i expected. For example 
I couldn’t find out how to run *Gotham* in mutiple threads. Seems it just assumes 
developer need to come up with the way how to run it themselves. On other hand is *Shio*, 
i could run it in multiple threads, but there is no difference with 1 thread 
performance. Maybe something is wrong with how macOS handle reuse address 
socket option?. Also, I had to modify shio to make it support “http pipeline”.

Each result in this table is best of five runs. All measurements are in *req/sec*.

Name | 1 thread | 1 pipeline | 3 thread | 3 pipeline | 8 thread | 8 pipeline
---- | -------- | ---------- | -------- | ---------- | -------- | ----------
Actix | 91.200 | 950.000 | 122.100 | 2.083.000 | 107.400 | 2.730.000
Gotham | 61.000 | 178.000 |   |   |   |
Iron |   |   |   |   | 94.500 | 78.000
Rocket |   |   |   |   | 95.500 | failed
Shio | 71.800 | 317.800 |   |   |   |   |
tokio-minihttp | 106.900 | 1.047.000 |   |   |   |

Absolute winner is *tokio-minihttp*. Which is makes sense, it doesn’t do much. 
But it is good reference point.

[![Analytics](https://ga-beacon.appspot.com/UA-110322332-2/benchmarks?pixel)](https://github.com/igrigorik/ga-beacon)
