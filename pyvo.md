# Unix pipes on steroi^Wzeromq

## About zeromq

 * Sockets on steroids
 * Rich set of communication patterns
   REQ/REP, PUSH/PULL, PUB/SUB, ...
 * A lot of trasports
   + inproc://
   + ipc://
   + tcp://
   + TIPC, UDP, VMCI
 * Engines written in C++ (libzmq), Java and .NET
 * Python API are well maintained, links to C++ engine

## Unix pipes

 * pass data from one process to an another one
   $ cat /etc/passwd | wc -l
   42

## zsubproc

 * Under active development
 * Inspired by Python subprocess module
 * Spawns monitoring actor (thread), which push
   execute program, setup pipes and push data
   from/to zeromq sockets
 * Creates pair of inproc sockets by default
 * But allows to set the read (stdin) or
   write (stdout, stderr) socket
