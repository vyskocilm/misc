#include <czmq.h>

int main () {

    zsubproc_t *self = zsubproc_new ();
    assert (self);
    zsubproc_set_stdout (self, NULL);

    char *const xargv[] = {"cat", "/etc/passwd", NULL};
    zsubproc_run (self, "/bin/cat", xargv, NULL);

    while (!zsys_interrupted) {
        zframe_t *frame = zframe_recv (zsubproc_stdout (self));
        if (!frame)
            break;
        write (STDIN_FILENO, zframe_data (frame), zframe_size (frame));
        zframe_destroy (&frame);
    }
    zsubproc_destroy (&self);
}
