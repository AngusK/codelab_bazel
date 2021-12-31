"""The Python implementation of the GRPC hello_world.Greeter client."""

from __future__ import print_function

from absl import app
from absl import flags
from absl import logging

import grpc
from examples.grpc import person_pb2
from examples.grpc import hello_world_pb2
from examples.grpc import hello_world_pb2_grpc

FLAGS = flags.FLAGS

flags.DEFINE_integer('port', 50051, 'GRPC server port to connect to.')
flags.DEFINE_string('name', 'you', 'Name to greet.')


def run(port: int, name: str):
    # NOTE(gRPC Python Team): .close() is possible on a channel and should be
    # used in circumstances in which the with statement does not fit the needs
    # of the code.
    host_port = 'localhost:%d' % port
    logging.info('Connecting to %s' % host_port)
    person = person_pb2.Person()
    person.name = name
    with grpc.insecure_channel(host_port) as channel:
        stub = hello_world_pb2_grpc.GreeterStub(channel)
        response = stub.SayHello(hello_world_pb2.HelloRequest(person=person))
    print("Greeter client received: " + response.message)


def main(argv):
    run(FLAGS.port, FLAGS.name)


if __name__ == '__main__':
    app.run(main)
