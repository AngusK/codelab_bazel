"""The Python implementation of the GRPC hello_world.Greeter client."""

import unittest

from grpc import StatusCode
import grpc_testing

from examples.grpc import hello_world_pb2
from examples.grpc import hello_world_pb2_grpc
from examples.grpc import person_pb2
from examples.py import greeter_service


class TestCase(unittest.TestCase):
    def __init__(self, methodName) -> None:
        super().__init__(methodName)

        myServicer = greeter_service.Greeter()
        servicers = {
            hello_world_pb2.DESCRIPTOR.services_by_name['Greeter']: myServicer
        }
        self.test_server = grpc_testing.server_from_dictionary(
            servicers, grpc_testing.strict_real_time())

    def test_search(self):
        person = person_pb2.Person(name='test_name')
        request = hello_world_pb2.HelloRequest(person=person)
        method = self.test_server.invoke_unary_unary(method_descriptor=(
            hello_world_pb2.DESCRIPTOR.services_by_name['Greeter'].
            methods_by_name['SayHello']),
                                                     invocation_metadata={},
                                                     request=request,
                                                     timeout=1)

        response, metadata, code, details = method.termination()
        self.assertEqual(response.message, 'Hello, test_name!')
        self.assertEqual(code, StatusCode.OK)


if __name__ == '__main__':
    unittest.main()
