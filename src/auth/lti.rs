use percent_encoding::{utf8_percent_encode as perc_encode, AsciiSet, NON_ALPHANUMERIC};
use hmac::{Hmac, Mac, NewMac};
use crypto_hashes::sha2::Sha256;
use std::collections::BTreeMap;
use crate::tools::epoch;

fn validate_lti(uri: &str, params: &str, secret: &str) -> Option<String> {
    let mut params: BTreeMap<String, String> = serde_urlencoded::from_str(&params).unwrap();

    let studip_signature = params.remove("oauth_signature").unwrap();
    let timestamp = params["oauth_timestamp"].parse::<i64>().unwrap();

    // We want to percent encode all characters except 'A-Z', 'a-z',
    // '0-9', '-', ',', '_', '~' (see RFC 3986)
    const FRAGMENT: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'.')
        .remove(b'_')
        .remove(b'~');

    let params_encoded = params.iter()
        .map(|(k, v)| format!("{}={}", k, perc_encode(&v, FRAGMENT)))
        .collect::<Vec<_>>()
        .join("&");

    let base_string = format!("POST&{}&{}",
                              perc_encode(uri, FRAGMENT),
                              perc_encode(&params_encoded, FRAGMENT)
    );

    // Calculate the signature of our request.
    let secret = format!("{}&", perc_encode(secret, FRAGMENT));
    let mut mac = Hmac::<Sha256>::new_varkey(secret.as_bytes()).unwrap();
    mac.update(base_string.as_bytes());
    let request_signature = base64::encode(mac.finalize().into_bytes());

    // Check if the request data is valid
    if request_signature != studip_signature || timestamp + 6000 < epoch() {
        return None;
    }

    params.remove("lis_person_sourcedid")
}