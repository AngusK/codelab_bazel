"""The Python implementation of the GRPC hello_world.Greeter service."""
from __future__ import print_function

from concurrent import futures

from absl import logging

from examples.grpc import hello_world_pb2
from examples.grpc import hello_world_pb2_grpc

from google.protobuf import timestamp_pb2


class Greeter(hello_world_pb2_grpc.GreeterServicer):
    def SayHello(self, request, context):
        timestamp_now = timestamp_pb2.Timestamp()
        timestamp_now.GetCurrentTime()
        return hello_world_pb2.HelloReply(message='Hello, %s!' %
                                          request.person.name,
                                          timestamp=timestamp_now)
