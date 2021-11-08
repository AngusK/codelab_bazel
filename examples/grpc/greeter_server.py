"""The Python implementation of the GRPC hello_world.Greeter server."""
from __future__ import print_function

from concurrent import futures

from absl import app
from absl import flags
from absl import logging

import grpc

from examples.grpc import hello_world_pb2
from examples.grpc import hello_world_pb2_grpc


FLAGS = flags.FLAGS

flags.DEFINE_integer('port', 50051, 'GRPC server port to listen on.')


class Greeter(hello_world_pb2_grpc.GreeterServicer):
    def SayHello(self, request, context):
        return hello_world_pb2.HelloReply(message='Hello, %s!' % request.name)


def serve(port: int):
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    hello_world_pb2_grpc.add_GreeterServicer_to_server(Greeter(), server)
    server.add_insecure_port('[::]:%d' % port)
    server.start()
    server.wait_for_termination()


def main(argv):
    serve(FLAGS.port)


if __name__ == '__main__':
    app.run(main)

