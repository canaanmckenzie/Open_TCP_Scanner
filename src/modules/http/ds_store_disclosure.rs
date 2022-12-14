//writing tests from specification
#[cfg(test)]
mod tests{
	#[test]
	fn is_ds_store(){
		let module = super::DsStoreDisclosure::new();
		let body = "testtesttest";
		let body2=[
			0x00,0x00,0x00,0x01,0x42,0x75,0x31,0x00,0x30,0x00,0x00,0x00,0x08,0x0,
		];

		assert_eq!(false,module.is_ds_store_file(body.as_bytes()));
		assert_eq!(true,module.is_ds_store_file(&body2));
	}
}

use crate::{
	modules::{HttpFinding,HttpModule,Module},
	Error,
};

use async_trait::async_trait;
use reqwest::Client;

pub struct DsStoreDisclosure {}

impl Module for DsStoreDisclosure {
	fn name(&self) -> String {
		String::from("http/ds_store")
	}

	fn description(&self) -> String {
		String::from("Check if a .Ds_Store file disclosure")
	}
}


impl DsStoreDisclosure{
	pub fn new() -> Self{
		DsStoreDisclosure {}
	}

	fn is_ds_store_file(&self, content: &[u8]) -> bool {
		if content.len() < 8{
			return false;
		}

	let signature =[0x0, 0x0, 0x0, 0x1, 0x42, 0x75, 0x64, 0x31];

	return content[0..8] == signature;
	}
}

#[async_trait]
impl HttpModule for DsStoreDisclosure {
	async fn scan(
		&self,
		http_client: &Client,
		endpoint: &str,
	) -> Result<Option<HttpFinding>, Error> {
		let url = format!("{}/.Ds_Store", &endpoint);
		let res = http_client.get(&url).send().await?;

		if !res.status().is_success(){
			return Ok(None);
		}

		let body = res.bytes().await?;
		if self.is_ds_store_file(&body.as_ref()){
			return Ok(Some(HttpFinding::DsStoreDisclosure(url)));
		}

		Ok(None)
	}
}

