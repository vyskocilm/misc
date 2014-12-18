#include <czmq.h>
#include <assert.h>
#include <stdlib.h>
#include <stdio.h>

#include <czmq.h>
#include <malamute.h>

static zmsg_t*
s_xzconfig_msg_save(zconfig_t* config) {
    assert (config);
    zchunk_t* chunk = zconfig_chunk_save( config );
    assert (chunk);
    zmsg_t *msg = zmsg_new();
    assert (msg);
        
    zmsg_addmem(msg, zchunk_data(chunk), zchunk_size(chunk));
    zchunk_destroy(&chunk);
    return msg;
}

static zconfig_t*
s_xzconfig_msg_load(zmsg_t* msg) {
    assert(msg);
    zframe_t* frame = zmsg_pop(msg);
    assert(msg);

    zchunk_t* chunk = zchunk_new(zframe_data(frame), zframe_size(frame));
    assert(chunk);

    zconfig_t* config = zconfig_chunk_load(chunk);

    zframe_destroy(&frame);
    zchunk_destroy(&chunk);
    return config;
}

static zmsg_t* s_load_config(zconfig_t *config) {
    assert(config);

    zmsg_t *msg_config = s_xzconfig_msg_save(config);
    return msg_config;
}

int main() {

    static const char* endpoint = "ipc://@/malamute";
    static const char* address  = "mconfig";
    static const char* subject  = "config";
    int r;

    mlm_client_t *client = mlm_client_new(endpoint, 5000, "mconfig");
    assert(client);
    //mlm_client_verbose(client);

    zconfig_t * config = zconfig_load ("mconfig.conf");
    assert(config);

    zmsg_t *msg_config = s_load_config(config);
    assert (msg_config);

    r =  mlm_client_set_producer (client, "config");
    assert (r == 0);

    zsock_t *msgpipe = mlm_client_msgpipe(client);
    assert(msgpipe);

    zpoller_t *poller = zpoller_new(msgpipe, NULL);
    assert(poller);

    while (!zpoller_terminated(poller)) {

        zsock_t *sock = (zsock_t*) zpoller_wait (poller, 700);

        if (!sock) {
            //polling is not that nice, but enough for our case ...
            if (zconfig_has_changed(config)) {
                r = zconfig_reload(&config);

                if (r != 0) {
                    continue;
                }

                zmsg_destroy(&msg_config);
                msg_config = s_load_config(config);
                assert (msg_config);

                zmsg_t *msg2 = zmsg_dup(msg_config);
                assert(msg2);
                mlm_client_send(client, "config", &msg2);
                printf("PUB:config: %s\n", zconfig_resolve(config, "root/value", "(null)"));
            }
            continue;
        }

        assert(sock == msgpipe); // for sure, there is nothing added into poller right now

        zmsg_t *msg = mlm_client_recv(client);
        assert(msg);
        const char* command = mlm_client_command(client);
        assert (command);

        if (!streq(command, "MAILBOX DELIVER")) {
            fprintf(stderr, "INFO: command '%s' != MAILBOX_DELIVER, ignoring\n", command);
            zmsg_destroy(&msg);
            continue;
        }
        printf("command: %s\n", command);

        const char* subject = mlm_client_subject(client);
        if (subject && ! streq(subject, "config")) {
            fprintf(stderr, "INFO: subject '%s' != config, ignoring\n", subject);
            zmsg_destroy(&msg);
            continue;
        }
        printf("subject: %s\n", subject);

        const char* sender = mlm_client_sender(client);
        if (!sender) {
            fprintf(stderr, "INFO: missing sender, ignoring\n");
            zmsg_destroy(&msg);
            continue;
        }
        zmsg_destroy(&msg);
        printf("sender: %s\n", sender);

        zmsg_t *msg2 = zmsg_dup(msg_config);
        assert(msg2);
        zmsg_print(msg2);
        r = mlm_client_sendto (client, sender, "config", NULL, 100, &msg2);
        printf("REP:config: %s\n", zconfig_resolve(config, "root/value", "(null)"));
        zclock_sleep(1000);
        assert (r == 0);

    }

    
    mlm_client_destroy(&client);
    zmsg_destroy(&msg_config);
    mlm_client_destroy(&client);
    zconfig_destroy(&config);

}
