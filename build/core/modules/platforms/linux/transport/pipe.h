#include <fcntl.h>
#include <types.h>
#include <sys/stat.h>
#include <unistd.h>

#include "../../../include/stdint.h"

#ifndef COSMOS_TRANSPORT_IPC_H
#define COSMOS_TRANSPORT_IPC_H

intptr_t * connection_point;
intptr_t * connection_type;

intmax_t buffer_size;
intmax_t queue_size;

void ipc_initialize ( intptr_t * connection_point, intptr_t * connection_type, intmax_t buffer_size) {}

void ipc_connect () {}

void ipc_send () {}

void ipc_receive () {}

void ipc_disconnect () {}

void ipc_terminate () {}

#endif /* COMSOS_TRANSPORT_IPC_H */