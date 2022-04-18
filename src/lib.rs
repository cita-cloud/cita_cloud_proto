// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod blockchain {
    tonic::include_proto!("blockchain");
}

pub mod common {
    tonic::include_proto!("common");
}

pub mod consensus {
    tonic::include_proto!("consensus");
}

pub mod controller {
    tonic::include_proto!("controller");
}

pub mod executor {
    tonic::include_proto!("executor");
}

pub mod kms {
    tonic::include_proto!("kms");
}

pub mod network {
    tonic::include_proto!("network");
}

pub mod storage {
    tonic::include_proto!("storage");
}

pub mod evm {
    tonic::include_proto!("evm");
}

pub mod health_check {
    tonic::include_proto!("grpc.health.v1");
}
