# -*- coding: utf-8 -*-
from __future__ import absolute_import
from __future__ import division
from __future__ import print_function

from absl.testing import absltest

from build_examples.proto import person_pb2
from build_examples.proto import address_pb2


class PersonProtoTests(absltest.TestCase):
    def test_person_attributes(self):
        #        a = person_pb2.Person()
        #        a.name = 'test name'
        #        a.address.city = 'EVYD City'
        #        a.address.zip_code.code = '0923'
        #        print(a)
        a = address_pb2.Address()
        a.city = 'EVYD City'
        a.zip_code.code = '0923'
        print(a)


if __name__ == '__main__':
    absltest.main()
