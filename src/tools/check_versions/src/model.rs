// Copyright (c) 2023 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0
//

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Versions {
    pub description: String,
    pub format: String,
    pub assets: Assets,
    pub externals: Externals,
    pub languages: Languages,
    pub specs: Specs,
    pub plugins: Plugins
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assets {
    pub hypervisor: Hypervisor,
    pub image: ArchitectureProject,
    pub initrd: ArchitectureProject,
    pub kernel: Project,
    #[serde(rename(deserialize = "kernel-experimental"))]
    pub kernel_experimental: Project,
    #[serde(rename(deserialize = "kernel-arm-experimental"))]
    pub kernel_arm_experimental: Project,
    #[serde(rename(deserialize = "kernel-dragonball-experimental"))]
    pub kernel_dragonball_experimental: Project,
    #[serde(rename(deserialize = "kernel-tdx-experimental"))]
    pub kernel_tdx_experimental: Project
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hypervisor {
    pub description: String,
    pub cloud_hypervisor: Project,
    pub firecracker: Project,
    pub qemu: Project,
    #[serde(rename(deserialize = "qemu-experimental"))]
    pub qemu_experimental: Project,
    #[serde(rename(deserialize = "qemu-tdx-experimental"))]
    pub qemu_tdx_experimental: Project
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArchitectureProject {
    pub description: String,
    pub url: String,
    pub architecture: Architecture,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Architecture {
    pub aarch64: Arch,
    pub ppc64le: Arch,
    pub s390x: Arch,
    pub x86_64: Arch 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Arch {
    pub name: String,
    pub version: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Externals {
    pub description: String,
    #[serde(rename(deserialize = "cni-plugins"))]
    pub cni_plugins: Project,
    pub conmon: Project,
    pub crio: Project,
    pub containerd: Project,
    pub critools: Project,
    pub gperf: Project,
    pub kubernetes: Project,
    pub libseccomp: Project,
    pub runc: Project,
    pub nydus: Project,
    #[serde(rename(deserialize = "nydus-snapshotter"))]
    pub nydus_snapshotter: Project,
    pub ovmf: Project,
    #[serde(rename(deserialize = "td-shim"))]
    pub td_shim: Project,
    pub virtiofsd: Project
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Languages {
    pub description: String,
    pub golang: Project,
    pub rust: Project,
    #[serde(rename(deserialize = "golangci-lint"))]
    pub golangci_lint: Project
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Specs {
    pub description: String,
    pub oci: Project
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plugins {
    pub description: String,
    #[serde(rename(deserialize = "sriov-network-device"))]
    pub sriov_network_device: Project
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub description: String,
    pub url: Option<String>,
    pub version: Option<String>,
    pub tag: Option<String>,
    pub branch: Option<String>
}
