# CITA Cloud Proto
Design grpc interface for each Micro Service.

### generate rust code

```
cargo build
```

### generate python code

```
pip install grpcio grpcio-tools
mkdir python
cd python
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/blockchain.proto
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/common.proto
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/consensus.proto
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/executor.proto
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/controller.proto
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/network.proto
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/storage.proto
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/kms.proto
```



