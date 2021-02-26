#include "ipc.h"

#ifndef COSMOS_TRANSPORT_PIPE_C
#define COSMOS_TRANSPORT_PIPE_C

/* This is the Cosmos pipe module. It will use named pipes on all
operating systems. There will be other implimentations of interprocess communication using other technologies.
The implementation employed in this module as stated before
are named pipes or FIFOs. Thought has to be given also to how this transport module will fit in the grander scheme of things.
And what methods we will use to ensure that transport moldules have similar structural qualities to ease development/generation
and interoperability, while maintaining all of the unique functionality of each transport. */


intmax_t buffer_size;
intmax_t queue_size;

typedef struct pipe_input {
  intmax_t id;
  char *path;
  int16_t permissions;
  char *mode;
  int32_t file_descriptor;
};

typedef struct pipe_output {
  intmax_t id;
  char *path;    
  int16_t permissions;
  char *mode;
  int32_t file_descriptor;
};

typedef struct connection {
  intmax_t id;
  char *type;

  
};

pipe_input pipe_input_initialize() {
  pipe_input input;
  input.mode = "r";
  input.path = "/tmp/pipe_input"; /* This needs to be a dynamically generated random file
  which is ensured to not collied with existing files. So that means checking whether the
  file created already exists, and if it does create a file point that has a different name.
  The easiest way of doing this would probably be appending the id of the input/output to the
  name of the file path. Which would make most sense to be named something like pipe_input etc.
  and would be best located in a Cosmos session specific folder to reduce the likelyhood that
  it would collied with files created by other programs or other Cosmos instances. Each pipe node
  would keep track of all of the connections it creates to use when creating other connections
  or managing existing connections. */
  input.permissions = "0666"
  mkfifo(input.path, input.permissions);
  input.file_descriptor = open(input.path, input.mode);
  /* Need to create an algorithem that generates a sudo random id. This id can be used
  to prepend the path name of the pipe with a unique identifier to avoid issues with sending
  data to the wrong pipe etc. */
  return input;
}

pipe_output pipe_output_initialize() {
  pipe_output output;
  output.mode = "w";
  output.path = "/tmp/pipe_output";
  output.permissions = "0666";
  mkfifo(output.path, output.permissions);
  input.file_descriptor = open(output.path, output.mode);
  return output;
}

connection pipe_initialize (intptr_t *connection, intmax_t buffer_size) {
  
  switch (connection_type) {
    
    case "input" :
      pipe_input input = pipe_input_initialize();
      return input;
      break;
    
    case "output" :
      pipe_output output = pipe_output_initialize();
      break;

    case "bidirectional" :
      pipe_input input = pipe_input_initialize();
      pipe_output output = pipe_output_initialize();
      break;
  }


}

void pipe_connect () {}

void pipe_send () {
  write()
}

void pipe_receive () {}

void pipe_disconnect () {

}

void pipe_terminate () {}

#endif /* COMSOS_TRANSPORT_PIPE_C