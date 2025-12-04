use {
    crate::error::ScillaError,
    serde::{Deserialize, Serialize},
    solana_commitment_config::CommitmentLevel,
    std::{fs, path::PathBuf},
};

pub const SCILLA_CONFIG_RELATIVE_PATH: &str = ".config/scilla.toml";

pub fn scilla_config_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Error getting home path");
    path.push(SCILLA_CONFIG_RELATIVE_PATH);
    path
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ScillaConfig {
    pub rpc_url: String,
    pub commitment_level: CommitmentLevel,
    pub keypair_path: PathBuf,
}

impl ScillaConfig {
    pub fn load() -> Result<ScillaConfig, ScillaError> {
        let scilla_config_path = scilla_config_path();
        if !scilla_config_path.exists() {
            return Err(ScillaError::ConfigPathDoesntExists);
        }
        let data = fs::read_to_string(scilla_config_path)?;
        let config: ScillaConfig = toml::from_str(&data)?;
        Ok(config)
    }

    pub fn default() -> Self {
        let mut keypair_path = dirs::home_dir().expect("Error getting home path");
        keypair_path.push(".config/solana/id.json");
        Self {
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            keypair_path,
            commitment_level: solana_commitment_config::CommitmentLevel::Confirmed,
        }
    }

    pub fn save(&self) -> Result<(), ScillaError> {
        let path = scilla_config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let toml_str = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(path, toml_str)?;
        Ok(())
    }
}
