# Swarm

## Summary

Swarm is an implementation of a distributed HTTP load tester in Rust. 

## Approximate time spent

25 hours

## Accomplishments

## Components, structure, design decisions

The structure of this project is heavily influenced by the goals I had set for myself every week. The first week, I had set out to simply create a non-distributed load tester. The architecture of the system was fairly straightforward at this point. There were 3 main components: the main method, a `Swarm`, and a `Member`. 

The main method would parse the options/arguments passed into the program and pass those to a `Swarm`. The `Swarm` would use those arguments to initialize and start a swarm, which involved creating a `Member` per request. The job of the `Member` was to send the actual request to the intended Host/URL. After all the requests have been sent, the `Swarm` also has the job of aggregating all the timing data relevant to each request and developing relevant statistics for the user.

![alt text](https://raw.githubusercontent.com/satyakb/swarm/master/swarm.png "Swarm")

For the next week, I had the goal of transforming the non-distributed load tester into a distributed one. This involved fitting what I had into a master-slave architecture. The slaves would start a standard HTTP web server using `Hyper` and listen for requests from a master node. When a slave receives a request from the master node, it parses the configuration data from that was sent in the request and starts its own `Swarm`. 

The master node had the job of parsing a configuration file to find out the IP addresses of the slaves and sending the configuration data to each slave. This had to be done in a multithreaded fashion because we want each slave to be started simultaneously, rather than sequentially. I decided to make the configuration file use YAML rather than JSON because I prefer the readability of YAML.

![alt text](https://raw.githubusercontent.com/satyakb/swarm/master/master-slave.png "Master-Slave")

During the final week, I wanted to implement 2 new features and spend the rest of the time cleaning up what I already implemented. The first feature was to have the slave nodes send their IP addresses to the master node so the user wouldn't have to enter in the IP's themselves. After playing around with the idea, I realized that it would actually be more inconvenient to have the program work this way. This meant the slaves would have to be restarted every run to ensure they connected with the master. However, I was able to figure out how to call C code in Rust to get the IP address of a particular interface. This proved to be useful because when a `Hyper` server is listening on 127.0.0.1, it is not listening on all interfaces, only on loopback. This meant the master couldn't actually reach slave nodes that weren't on the local machine. Using the C code, I was able to get the slave nodes to bind to public IP addresses (e.g. en0, eth0, wlan0, etc.).

The second feature was to create the ability to test a sequence of requests rather than just one request. This feature was straightforward to add. I modified the config file to take in a sequence of requests that would be parsed by the master node and added to the `Config` struct, which was sent to all the slaves.

## Testing approach and results

## Benchmarks

To benchmark `Swarm`, I compared its output and execution time to that of Apache Bench.

### Swarm 

N | Total | Mean  |  Min | Max | %Failed
--- | --- | --- | --- | --- | ---
1000  |  1004308 | 1004.3 |  607 | 1604 |   0
```
N   Total   Mean    Min Max %Failed
1000    1766872 1766.9  928 3956    0
cargo run --release -- unleash -n 1000 http://google.com/  0.82s user 2.82s system 79% cpu 4.556 total
```
```
cargo run -- master config.yaml  0.60s user 0.21s system 30% cpu 2.635 total
```

### Apache Bench

```
Concurrency Level:      1000
Time taken for tests:   1.995 seconds
Complete requests:      1000
Failed requests:        0
Non-2xx responses:      1000
Total transferred:      540000 bytes
HTML transferred:       219000 bytes
Requests per second:    501.33 [#/sec] (mean)
Time per request:       1994.713 [ms] (mean)
Time per request:       1.995 [ms] (mean, across all concurrent requests)
Transfer rate:          264.37 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:       20  117 118.3     75     384
Processing:    49  497 375.3    690    1850
Waiting:       48  493 376.7    690    1836
Total:         89  614 418.1    741    1979

Percentage of the requests served within a certain time (ms)
  50%    741
  66%    869
  75%    928
  80%    966
  90%   1278
  95%   1330
  98%   1340
  99%   1343
 100%   1979 (longest request)
```
```
ab -n 1000 -c 1000 http://google.com/  0.03s user 0.09s system 5% cpu 2.039 total
```

## Limitations

The big limitation for me throughout this project was time. I would have liked to spend more time on it, but I was swamped with final projects and homework in other classes that restricted the amount of time I could have spent on this. With more time, I could really take this project to the limit.

## Postmortem

### What went well

The base functionality of the project went extremely well. Creating a simple distributed HTTP load tester proved to be a challenge, especially with the concurrent nature of the program. However, I am happy with the implementation of `Swarm` and how it works.

### What I would do differently

If I had more time I would love to be able to add more advanced features to the load tester. One of the main reasons I picked this project was the potential of taking this above and beyond. I would like to create a GUI that would make the program even easier to use. I would like to add more advanced request options that allow the program to do even more complicated sequences.

One thing I would change about my current implementation is the general naming scheme. I'm happy with the name `Swarm` but it is a little ambiguous. That is, `Swarm` could refer to the requests the program sends or all of the slave nodes that the master is controlling. This is something I didn't have the foresight to see when working on the week 1 goal of a non-distributed load tester.

