use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ssm::Client as SsmClient;

async fn get_config() -> Result<aws_types::SdkConfig, Box<dyn std::error::Error>> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    Ok(shared_config)
}

async fn get_ssm_client() -> Result<SsmClient, Box<dyn std::error::Error>> {
    let shared_config = get_config().await;
    match shared_config {
        Ok(config) => {
            let ssm_client = SsmClient::new(&config);
            Ok(ssm_client)
        }
        Err(e) => Err(e),
    }
}

async fn get_param_as_string(
    param_name: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let ssm_client = get_ssm_client().await?;

    let response = ssm_client
        .get_parameter()
        .name(param_name)
        .with_decryption(true)
        .send()
        .await?;
    if let Some(parameter) = response.parameter() {
        if let Some(value) = parameter.value() {
            return Ok(Some(value.to_string()));
        }
    }
    Ok(None)
}

pub async fn get_binance_api_key() -> Result<Option<String>, Box<dyn std::error::Error>> {
    match get_param_as_string("binance-api-key").await {
        Ok(Some(api_key)) => {
            println!("Using API key {}", api_key);
            Ok(Some(api_key))
        }
        Ok(None) => {
            println!("No API key founded!");
            Ok(None)
        }
        Err(e) => {
            eprintln!("Cannot get API key from AWS {}", e);
            Err(e)
        }
    }
}
