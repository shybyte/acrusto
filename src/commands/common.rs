use clap::crate_version;
use crate::api::AcroApi;
use crate::api::AcroApiProps;
use crate::api::ClientInformation;


pub fn connect<S: Into<String>>(server_url: S, token: Option<&str>) -> AcroApi {
    AcroApi::new(AcroApiProps {
        server_url: server_url.into(),
        locale: "en".to_string(),
        client: ClientInformation {
            name: "Acrusto".to_string(),
            signature: "dummyClientSignature".to_string(),
            version: crate_version!().to_string(),
        },
    }, token)
}