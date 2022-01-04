"""The Python implementation of the GRPC hello_world.Greeter server."""
from __future__ import print_function

from concurrent import futures

from absl import app
from absl import flags
from absl import logging

import grpc

from examples.grpc import hello_world_pb2_grpc
from examples.py import greeter_service

FLAGS = flags.FLAGS

flags.DEFINE_integer('port', 50051, 'GRPC server port to listen on.')


def main(argv):
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    hello_world_pb2_grpc.add_GreeterServicer_to_server(
        greeter_service.Greeter(), server)
    server.add_insecure_port('[::]:%d' % FLAGS.port)
    server.start()
    logging.info("Server started on port %d" % FLAGS.port)
    server.wait_for_termination()


if __name__ == '__main__':
    app.run(main)
