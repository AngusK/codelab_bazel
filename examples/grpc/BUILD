load("@rules_proto//proto:defs.bzl", "proto_library")
load("@build_stack_rules_proto//rules:proto_compile.bzl", "proto_compile")

package(default_visibility = ["//examples:__subpackages__"])

# person proto
proto_library(
    name = "person_proto",
    srcs = ["person.proto"],
)

proto_compile(
    name = "person_proto_compile",
    #output_mappings = ["person.pb.go=github.com/AngusK/codelab_bazel/examples/grpc/person.pb.go"],
    outputs = [
        "person_pb2.py",
        #"person.pb.go",
    ],
    plugins = [
        "@build_stack_rules_proto//plugin/builtin:python",
        #"@build_stack_rules_proto//plugin/golang/protobuf:protoc-gen-go",
    ],
    proto = ":person_proto",
)

py_library(
    name = "person_py_pb2",
    srcs = ["person_pb2.py"],
    deps = ["//third_party/py:protobuf"],
)

# hello_world proto
proto_library(
    name = "hello_world_proto",
    srcs = ["hello_world.proto"],
    deps = [
        ":person_proto",
        # Well known protos should be included as deps in the
        # proto_library rules of the source files importing them.
        # A list of all @com_google_protobuf well known protos can
        # seen with:
        # `bazel query 'kind(proto_library, @com_google_protobuf//:all)'`
        "@com_google_protobuf//:any_proto",
        "@com_google_protobuf//:timestamp_proto",
    ],
)

# only proto
proto_compile(
    name = "hello_world_proto_compile",
    #output_mappings = ["hello_world.pb.go=github.com/AngusK/codelab_bazel/examples/grpc/hello_world.pb.go"],
    outputs = [
        "hello_world_pb2.py",
        #"hello_world.pb.go",
    ],
    plugins = [
        "@build_stack_rules_proto//plugin/builtin:python",
        #"@build_stack_rules_proto//plugin/golang/protobuf:protoc-gen-go",
    ],
    proto = ":hello_world_proto",
)

py_library(
    name = "hello_world_py_pb2",
    srcs = ["hello_world_pb2.py"],
    deps = [
        ":person_py_pb2",
        "//third_party/py:protobuf",
    ],
)

# proto compile for GPRC
proto_compile(
    name = "hello_world_proto_grpc_compile",
    #output_mappings = ["hello_world_grpc.pb.go=github.com/AngusK/codelab_bazel/examples/grpc/hello_world_grpc.pb.go"],
    outputs = [
        "hello_world_pb2_grpc.py",
        #"hello_world_grpc.pb.go",
    ],
    plugins = [
        "@build_stack_rules_proto//plugin/grpc/grpc:protoc-gen-grpc-python",
        #"@build_stack_rules_proto//plugin/grpc/grpc-go:protoc-gen-go-grpc",
    ],
    proto = ":hello_world_proto",
)

py_library(
    name = "hello_world_py_pb2_grpc",
    srcs = ["hello_world_pb2_grpc.py"],
    deps = [
        ":hello_world_py_pb2",
        "//third_party/py:grpcio",
    ],
)
