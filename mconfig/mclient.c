
#include <czmq.h>
#include <assert.h>
#include <stdlib.h>
#include <stdio.h>

#include<malamute.h>

int main() {


    static const char* endpoint = "ipc://@/malamute";

    mlm_client_t *client = mlm_client_new(endpoint, 1000, "mclient");
    assert(client);
    //mlm_client_verbose(client);

    //ask for config
    int r;
    zmsg_t *empty = zmsg_new();
    r = mlm_client_sendto(client, "mconfig", "config", NULL, 1000, &empty);
    assert(r==0);

    zmsg_t *msg = mlm_client_recv(client);
    assert(msg);
    const char* command = mlm_client_command(client);
    assert (command);

    printf("command: %s\n", command);
    zmsg_print(msg);
    zmsg_destroy(&msg);
    
    r = mlm_client_set_consumer(client, "config", ".*");
    assert (r == 0);

    //got notified about 'em
    while (!zsys_interrupted) {
        
        zmsg_t *msg = mlm_client_recv(client);
        assert(msg);
        const char* command = mlm_client_command(client);
        assert (command);
    
        printf("command: %s\n", command);
        zmsg_print(msg);
        zmsg_destroy(&msg);

    }

    mlm_client_destroy(&client);

}
