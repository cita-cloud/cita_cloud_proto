# CITA Cloud Proto Agent Guide

## 项目基本信息
- **项目名称**: cita_cloud_proto
- **项目描述**: 该项目包含 CITA-Cloud 云原生区块链框架的 Protocol Buffers 定义文件。它定义了 CITA-Cloud 架构中各个微服务（如 Controller、Executor、Consensus 等）之间的 gRPC 通信接口和数据结构。
- **主要语言**: Protocol Buffers (proto3)

## 核心能力
本项目定义了以下核心微服务的接口规范：

1.  **Controller (控制层)**:
    -   `RPCService`: 提供给外部客户端的 RPC 接口，包括查询区块/交易信息、发送交易、获取系统配置等。
    -   `Consensus2ControllerService`: 供共识服务调用的接口，用于获取提案、验证提案和提交区块。

2.  **Executor (执行层)**:
    -   `ExecutorService`: 负责区块的执行 (`Exec`) 和只读调用 (`Call`)，处理智能合约逻辑。

3.  **Blockchain (数据结构)**:
    -   定义了区块链的核心数据结构，如 `Block` (区块)、`Transaction` (交易)、`CompactBlock` (紧凑区块) 等。

4.  **Consensus (共识层)**:
    -   定义共识相关的接口和消息格式。

5.  **Network (网络层)**:
    -   定义节点间 P2P 通信的接口。

6.  **Storage (存储层)**:
    -   定义数据持久化的接口。

7.  **Crypto (加密层)**:
    -   定义加密操作相关的接口。

## 运行依赖
要使用或编译本项目中的 proto 文件，通常需要以下工具：

-   **Protocol Buffers Compiler (`protoc`)**: 用于将 `.proto` 文件编译为目标语言代码。
-   **gRPC 库**: 对应目标语言的 gRPC 运行时库。
    -   Python: `grpcio`, `grpcio-tools`
    -   其他语言 (Rust, Go, Java 等) 需安装相应的 gRPC 工具链。

## 使用示例

### 生成 Python 代码
根据 `README.md`，可以使用以下命令生成 Python 客户端/服务端代码：

```bash
# 安装依赖
pip install grpcio grpcio-tools

# 创建输出目录
mkdir -p python
cd python

# 生成代码 (示例)
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/blockchain.proto
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/controller.proto
python3 -m grpc_tools.protoc -I../protos --python_out=. --grpc_python_out=. ../protos/executor.proto
# ... 其他 proto 文件类似
```

### 目录结构说明
-   `protos/`: 存放所有的 `.proto` 定义文件。
    -   `controller.proto`: 控制服务接口。
    -   `executor.proto`: 执行服务接口。
    -   `blockchain.proto`: 基础数据结构。
    -   `common.proto`: 通用数据结构 (Hash, Address 等)。
