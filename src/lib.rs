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
