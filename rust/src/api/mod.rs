use anyhow::Result;
use apk_parser::zip::ZipArchive;
use apk_parser::{parse_android_manifest, ApkSignatureBlock, ApkSigningBlock};
use flutter_rust_bridge::frb;
use log::info;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

pub struct ApkParser {
    path: PathBuf,
}

pub struct AndroidManifestParsed {
    pub package: Option<String>,
    pub version_code: Option<u32>,
    pub version_name: Option<String>,
    pub compile_sdk_version: Option<u32>,
    pub compile_sdk_version_codename: Option<u32>,
    pub platform_build_version_code: Option<u32>,
    pub platform_build_version_name: Option<u32>,
    pub min_sdk_version: Option<u32>,
    pub target_sdk_version: Option<u32>,
    pub max_sdk_version: Option<u32>,
    pub icon: Option<String>,
    pub label: Option<String>,
    pub sigs: Vec<ApkSignature>,
}

pub struct ApkSignature {
    pub digest: Vec<u8>,
    pub certificates: Vec<Vec<u8>>,
    pub signature: Vec<u8>,
    pub algo: String,
}

impl ApkParser {
    #[frb(sync)]
    pub fn new(path: &str) -> Self {
        ApkParser {
            path: PathBuf::from(path),
        }
    }

    #[frb(sync)]
    pub fn load_manifest(&self) -> Result<AndroidManifestParsed> {
        info!("Loading APK: {}", self.path.display());
        let mut file = File::open(&self.path)?;
        let sig_block = ApkSigningBlock::from_reader(&mut file)?;

        let sigs = sig_block.get_signatures()?;
        let mut zip = ZipArchive::new(file)?;
        let mut manifest_entry = zip.by_name("AndroidManifest.xml")?;

        // read manifest to buffer
        let mut manifest_buf = Vec::with_capacity(8192);
        let r = manifest_entry.read_to_end(&mut manifest_buf)?;
        info!("Read {} bytes from AndroidManifest.", r);

        // parse manifest
        let mfs = parse_android_manifest(&manifest_buf[..r])?;
        Ok(AndroidManifestParsed {
            package: mfs.package,
            version_code: mfs.version_code,
            version_name: mfs.version_name,
            compile_sdk_version: mfs.compile_sdk_version,
            compile_sdk_version_codename: mfs.compile_sdk_version_codename,
            platform_build_version_code: mfs.platform_build_version_code,
            platform_build_version_name: mfs.platform_build_version_name,
            min_sdk_version: mfs.sdk.min_sdk_version,
            target_sdk_version: mfs.sdk.target_sdk_version,
            max_sdk_version: mfs.sdk.max_sdk_version,
            icon: mfs.application.icon,
            label: mfs.application.label,
            sigs: sigs
                .into_iter()
                .flat_map(|sig| match sig {
                    ApkSignatureBlock::V2 {
                        signatures,
                        certificates,
                        ..
                    } => signatures
                        .into_iter()
                        .map(|s| ApkSignature {
                            algo: s.algo.to_string(),
                            digest: s.digest,
                            certificates: certificates.clone(),
                            signature: s.signature,
                        })
                        .collect(),
                    ApkSignatureBlock::V3 {
                        signatures,
                        certificates,
                        ..
                    } => signatures
                        .into_iter()
                        .map(|s| ApkSignature {
                            algo: s.algo.to_string(),
                            digest: s.digest,
                            certificates: certificates.clone(),
                            signature: s.signature,
                        })
                        .collect(),
                    _ => vec![],
                })
                .collect(),
        })
    }
}
