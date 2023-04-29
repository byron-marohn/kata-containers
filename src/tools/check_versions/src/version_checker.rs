// Copyright (c) 2023 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0
//

use serde_json;
use reqwest;

use std::error::Error;

use crate::model::*;
use crate::error::*;
use crate::cli::Args;
use crate::output::write_output;

pub fn check_versions(versions: Versions, args: &Args) -> Result<(), Box<dyn Error>> {
    check_asset_versions(&versions.assets, &args)?;
    check_externals_versions(&versions.externals, &args)?;
    check_languages_versions(&versions.languages, &args)?;
    check_specs_versions(&versions.specs, &args)?;
    check_plugins_versions(&versions.plugins, &args)?;
    Ok(())
}

fn check_asset_versions(assets: &Assets, args: &Args) -> Result<(), Box<dyn Error>> {
    check_hypervisor_versions(&assets.hypervisor, &args)?;
    check_image_versions(&assets.image, &args)?;
    check_initrd_versions(&assets.initrd, &args)?;
    check_kernel_versions(&assets, &args)?;
    Ok(())
}

fn check_hypervisor_versions(hypervisor: &Hypervisor, args: &Args) -> Result<(), Box<dyn Error>> {
    check_project_version(&hypervisor.cloud_hypervisor, "cloud_hypervisor", &args)?;
    check_project_version(&hypervisor.firecracker, "firecracker", &args)?;
    check_project_version(&hypervisor.qemu, "qemu", &args)?;
    check_project_version(&hypervisor.qemu_experimental, "qemu-experimental", &args)?;
    check_project_version(&hypervisor.qemu_tdx_experimental, "qemu-tdx-experimental", &args)?;
    Ok(())
}

fn check_image_versions(image: &ArchitectureProject, args: &Args) -> Result<(), Box<dyn Error>> {
    check_architecture_project_version(&image, &image.architecture.aarch64, "image-aarch64", &args)?;
    check_architecture_project_version(&image, &image.architecture.ppc64le, "image-ppc64le", &args)?;
    check_architecture_project_version(&image, &image.architecture.s390x, "image-s390x", &args)?;
    check_architecture_project_version(&image, &image.architecture.x86_64, "image-x86_64", &args)?;
    Ok(())
}

fn check_initrd_versions(initrd: &ArchitectureProject, args: &Args) -> Result<(), Box<dyn Error>> {
    check_architecture_project_version(&initrd, &initrd.architecture.aarch64, "initrd-aarch64", &args)?;
    check_architecture_project_version(&initrd, &initrd.architecture.ppc64le, "initrd-ppc64le", &args)?;
    check_architecture_project_version(&initrd, &initrd.architecture.s390x, "initrd-s390x", &args)?;
    check_architecture_project_version(&initrd, &initrd.architecture.x86_64, "initrd-x86_64", &args)?;
    Ok(())
}

fn check_kernel_versions(assets: &Assets, args: &Args) -> Result<(), Box<dyn Error>> {
   check_project_version(&assets.kernel, "kernel", &args)?;
   check_project_version(&assets.kernel_experimental, "kernel-experimental", &args)?;
   check_project_version(&assets.kernel_arm_experimental, "kernel-arm-experimental", &args)?;
   check_project_version(&assets.kernel_dragonball_experimental, "kernel-dragonball-experimental", &args)?;
   check_project_version(&assets.kernel_tdx_experimental, "kernel-tdx-experimental", &args)?;
   Ok(())
}

fn check_externals_versions(externals: &Externals, args: &Args) -> Result<(), Box<dyn Error>> {
    check_project_version(&externals.cni_plugins, "cni-plugins", &args)?;
    check_project_version(&externals.conmon, "conmon", &args)?;
    check_project_version(&externals.crio, "crio", &args)?;
    check_project_version(&externals.containerd, "containerd", &args)?;
    check_project_version(&externals.critools, "critools", &args)?;
    check_project_version(&externals.gperf, "gperf", &args)?;
    check_project_version(&externals.kubernetes, "kubernetes", &args)?;
    check_project_version(&externals.libseccomp, "libseccomp", &args)?;
    check_project_version(&externals.runc, "runc", &args)?;
    check_project_version(&externals.nydus, "nydus", &args)?;
    check_project_version(&externals.nydus_snapshotter, "nydus-snapshotter", &args)?;
    check_project_version(&externals.ovmf, "ovmf", &args)?;
    check_project_version(&externals.td_shim, "td-shim", &args)?;
    check_project_version(&externals.virtiofsd, "virtiofsd", &args)?;
    Ok(())
}

fn check_languages_versions(languages: &Languages, args: &Args) -> Result<(), Box<dyn Error>> {
    check_project_version(&languages.golang, "golang", &args)?;
    check_project_version(&languages.rust, "rust", &args)?;
    check_project_version(&languages.golangci_lint, "golangci-lint", &args)?;
    Ok(())
}

fn check_specs_versions(specs: &Specs, args: &Args) -> Result<(), Box<dyn Error>> {
    check_project_version(&specs.oci, "oci", &args)?;
    Ok(())
}

fn check_plugins_versions(plugins: &Plugins, args: &Args) -> Result<(), Box<dyn Error>> {
    check_project_version(&plugins.sriov_network_device, "sriov-network-device", &args)?;
    Ok(())
}

fn check_project_version(project: &Project, name: &str, args: &Args) -> Result<(), Box<dyn Error>> {
    let current_version = match get_version_string(&project) {
        Ok(version) => version,
        Err(_e) => {
            let message = format!("Warning! Failed to read version for {}\n", name);
            write_output(message, &args)?;
            String::from("unknown")
        }
    };

    match &project.url {
        Some(url) => {
            if is_github_url(url.as_str()) {
               check_github_version(url.as_str(), current_version.as_str(), name, &args)?;
            } else {
                match name {
                    "virtiofsd" => check_virtiofsd_version(name, current_version.as_str(), &args)?,
                    _ => ()
                }
            }
        },
        None => {
            // Assume project is a language if url is not present
            check_language_version(name, current_version.as_str(), &args)?;
        }
    }

    Ok(())
}

fn check_language_version(
    name: &str,
    current_version: &str,
    args: &Args) -> Result<(), Box<dyn Error>> {
    match name {
        "golang" => {
            let url = "https://golang.org/VERSION?m=text";
            match get_latest_version(url) {
                Ok(latest_version) => {
                    let message = format!("project: {}, current_version: {}, latest_version: {}\n",
                        name, current_version, latest_version);
                    write_output(message, &args)?;
                },
                Err(_e) => {
                    let message = format!("Warning! Failed to check version for {}\n", name);
                    write_output(message, &args)?;
                }
            }
        },
        "golangci-lint" => {
            let url = "https://github.com/golangci/golangci-lint";
            match get_github_latest_version(url, &args) {
                Ok(latest_version) => {
                    let message = format!("project: {}, current_version: {}, latest_version: {}\n",
                        name, current_version, latest_version);
                    write_output(message, &args)?;
                },
                Err(_e) => {
                    let message = format!("Warning! Failed to check version for {}\n", name);
                    write_output(message, &args)?;
                }
            }
        },
        "rust" => {
            let url = "https://api.github.com/repos/rust-lang/rust/releases/latest";
            match get_rust_latest_version(url, &args) {
                Ok(latest_version) => {
                    let message = format!("project: {}, current_version: {}, latest_version: {}\n",
                        name, current_version, latest_version);
                    write_output(message, &args)?;
                },
                Err(_e) => {
                    let message = format!("Warning! Failed to check version for {}\n", name);
                    write_output(message, &args)?;
                }
            }
        },
        _ => ()
    }

    Ok(())
}

fn check_architecture_project_version(
    project: &ArchitectureProject,
    arch: &Arch,
    name: &str,
    args: &Args) -> Result<(), Box<dyn Error>> {
    if is_github_url(project.url.as_str()) {
        check_github_version(project.url.as_str(), arch.version.as_str(), name, &args)?;
    }

    Ok(())
}

fn get_version_string(project: &Project) -> Result<String, Box<dyn Error>> {
    match &project.tag {
        Some(tag) => Ok(tag.clone()),
        None => match &project.branch {
            Some(branch) => Ok(branch.clone()),
            None => match &project.version {
                Some(version) => Ok(version.clone()),
                None => Err(Box::new(MissingVersionError {}))
            }
        }
    }
}

fn get_github_latest_version(url: &str, args: &Args) -> Result<String, Box<dyn Error>> {
    let github_url = to_github_api_url(url);
    let mut client = reqwest::blocking::Client::new()
        .get(github_url)
        .header("User-Agent", "Check Versions v1.0");

    match &args.github_token {
        Some(github_token) => {
            if !github_token.is_empty() { 
                client = client.header("Authorization", "Bearer ".to_owned() + github_token)
            }
        },
        None => ()
    }

    let versions_response = client.send()?.text()?;
    let versions: serde_json::Value = serde_json::from_str(versions_response.as_str())?;

    let tag = versions.get("tag_name")
        .ok_or(Box::new(ParserError {}))?
        .as_str()
        .ok_or(Box::new(ParserError {}))?;
    Ok(String::from(tag))
}

fn check_github_version(
    url: &str,
    current_version: &str,
    name: &str,
    args: &Args) -> Result<(), Box<dyn Error>> {
    match get_github_latest_version(url, &args) {
        Ok(latest_version) => {
            let message = format!("project: {}, current_version: {}, latest_version: {}\n",
                name, current_version, latest_version);
            write_output(message, &args)?;
        },
        Err(_e) => {
            let message = format!("Warning! Failed to check version for {}\n", name);
            write_output(message, &args)?;
        }
    }

    Ok(())
}

fn check_virtiofsd_version(
    name: &str,
    current_version: &str,
    args: &Args) -> Result<(), Box<dyn Error>> {
    let url = "https://gitlab.com/api/v4/projects/21523468/repository/tags";
    match get_virtiofsd_latest_version(url) {
        Ok(latest_version) => {
            let message = format!("project: {}, current_version: {}, latest_version: {}\n",
                name, current_version, latest_version);
            write_output(message, &args)?;
        },
        Err(_e) => {
            let message = format!("Warning! Failed to check version for {}\n", name);
            write_output(message, &args)?;
        }
    }

    Ok(())
}

fn get_rust_latest_version(url: &str, args: &Args) -> Result<String, Box<dyn Error>> {
    let mut client = reqwest::blocking::Client::new()
        .get(url)
        .header("User-Agent", "Check Versions v1.0");

    match &args.github_token {
        Some(github_token) => if !github_token.is_empty() {client = client.header("Authorization", "Bearer ".to_owned() + github_token)},
        None => ()
    }

    let versions_response = client.send()?.text()?;
    let versions: serde_json::Value = serde_json::from_str(versions_response.as_str())?;

    let tag = versions.get("tag_name")
        .ok_or(Box::new(ParserError {}))?
        .as_str()
        .ok_or(Box::new(ParserError {}))?;
    Ok(String::from(tag))
}

fn get_virtiofsd_latest_version(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new()
        .get(url)
        .header("User-Agent", "Check Versions v1.0");

    let versions_response = client.send()?.text()?;
    let versions: serde_json::Value = serde_json::from_str(versions_response.as_str())?;

    let tag = versions.get(0)
        .ok_or(Box::new(ParserError {}))?
        .get("name")
        .ok_or(Box::new(ParserError {}))?
        .as_str()
        .ok_or(Box::new(ParserError {}))?;
    Ok(String::from(tag))
}


fn get_latest_version(url: &str) -> Result<String, Box<dyn Error>> {
    let version_response = reqwest::blocking::Client::new()
                            .get(url).send()?.text()?;

    Ok(version_response.clone())
}



fn to_github_api_url(url: &str) -> String {
    match url {
        x if x.contains("runtime-spec") => {
            return  url
                .replace("https://github.com", "https://api.github.com/repos")
                .replace("releases", "releases/latest")
                .to_string();
        },
        x if x.contains("containerd/containerd") =>{
            return (url
                .replace("github.com", "https://api.github.com/repos") + "/releases/latest")
                .to_string();
        },
        _ => {
            return (url
                .replace("https://github.com", "https://api.github.com/repos") + "/releases/latest")
                .to_string();
        }
    }
}

fn is_github_url(url: &str) -> bool {
    url.contains("github.com")
}
