#Distributed configuration using Malamute

The idea is to decouple configuration from modules and let have them managed by central components in the system. Powered by cool zeromq technologies, most notably zconfig (http://czmq.zeromq.org/manual:zconfig). Because it can be dumped into chunk. And chunks can be stored into zmsg. Which can be sent through malamute.

##How to build
1. Install latest czmq
2. Install latest malamute
3. Call ./b with proper MLM_INCLUDE and MLM_LDPATH variables
4. Start malamute malamute.cfg, ./m, ./c, ./cc and enjoy (tmux recommended)

## mconfig

1. Does REPly on "config" REQuests
2. Periodically checks for changes and if so, then PUBlish "config" topic

## mclient

1. REQuest "config" from mconfig module
2. SUBscribe to the "config" topic, so it got notified about changes

##Disadvantages
1.	You can’t setup malamute address that way!
2.	It is hard to change things like client’s name, so will require some initial handshake
3.	It creates a SPOF in the system, one component, which can prevent new clients to start if crashed
4.	It is way more complex than calling zconfig_load/zconfig_reload_zconfig_save or equivalent from each module
5.	Working with different formats than zconfig will require writing a de/serialization to chunk.

##Mitigiations
1.	Setup address using environment variable/commandline/other way (gossip discovery?)
2.	Hide it behind a functions
3.	As mconfig does not manage any state it can be restarted. Alternatively this can be a service, so more instances can run in a parallel
4.	N/A
5.	This is trivial to do, given the fact most of config files can be represented as a plain string.

##Advantages
1.	It was fun to design and develop
2.	It  can decouple clients from filesystem, so it does not matter where configuration for each of it is actually stored.
