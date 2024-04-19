pub mod auth;
pub mod host_with_port;
pub mod params;
pub mod scheme;
pub mod uri_with_params;
pub mod uri_with_params_list;

use rsip::common::uri::{param::Maddr, Param, Scheme, Tokenizer, Uri};
use std::convert::TryInto;

mod whole {
    use std::convert::TryFrom;
    use rsip::{UriWithParams, UriWithParamsList};
    use rsip::typed::tokenizers::UriWithParamsTokenizer;
    use super::*;

    #[test]
    fn test<>() {
        // let uri = Uri::try_from("sip:alice;day=tuesday@atlanta.com");
        // let uri = Uri::try_from("sip:2222;user:password@10.219.12.179:5060;user=phone;transport=SCTP;yop=00.00.D23F7134.0000.7015 something").unwrap();
        // let uri = UriWithParams::try_from((
        //     Uri::try_from("sips:client.biloxi.example.com:5061").unwrap(),
        //     vec![Param::from(Maddr::new("255.255.255.0"))]
        // ));


        let uri_with_params_raw = "sips:client.biloxi.example.com:5061;maddr=255.255.255.0;foo=192.0.2.201;lr".to_string();
        let tokenizer = UriWithParamsTokenizer::tokenize(&uri_with_params_raw)
            .unwrap()
            .1;

        let uri_with_params_raw = "<sip:alice@atlanta.example.com;s=2>;level=low".to_string();
        let tokenizer = UriWithParamsTokenizer::tokenize(&uri_with_params_raw)
            .unwrap()
            .1;

        let t = Tokenizer::tokenize("sips:client.biloxi.example.com:5061;maddr=255.255.255.0;foo=192.0.2.201;lr").unwrap().1;

        // let x: UriWithParams = tokenizer.try_into().unwrap();


        let x = UriWithParams::try_from(tokenizer).unwrap();


        // let uri = UriWithParams::try_from(t);

        // let uri = Uri::try_from("sip:alice;day=tuesday@atlanta.com");

        println!("{:#?}", x);

        // assert_eq!(
        //     Uri::try_from("sip:2222;phone-context=unknown;voicexml=http%3A//10.220.90.229%3A8080/web-ivr-api-1.0/vxml/Dostepgoscia_pojedynczy_48514748636.vxml@10.219.12.179:5060;user=phone;transport=SCTP;yop=00.00.D23F7134.0000.7015").unwrap().to_string(),
        //     String::from("sip:2222;phone-context=unknown;voicexml=http%3A//10.220.90.229%3A8080/web-ivr-api-1.0/vxml/Dostepgoscia_pojedynczy_48514748636.vxml@10.219.12.179:5060;user=phone;transport=SCTP;yop=00.00.D23F7134.0000.7015")
        // );
    }

    #[test]
    fn act1() {
        assert_eq!(
            Uri::try_from("sip:2222;phone-context=unknown;voicexml=http%3A//10.220.90.229@10.219.12.179:5060;user=phone;transport=SCTP;yop=00.00.D23F7134.0000.7015").unwrap().to_string(),
            String::from("sip:2222;phone-context=unknown;voicexml=http%3A//10.220.90.229@10.219.12.179:5060;user=phone;transport=SCTP;yop=00.00.D23F7134.0000.7015")
        );
    }

    #[test]
    fn rfc1() {
        assert_eq!(
            Uri::try_from("sip:alice;day=tuesday@atlanta.com").unwrap().to_string(),
            String::from("sip:alice;day=tuesday@atlanta.com")
        );
    }

    #[test]
    fn actual1() {
        assert_eq!(
            Uri::try_from("sip:2222;phone-context=unknown@10.219.12.179:5060;user=phone;transport=SCTP;yop=00.00.D23F7134.0000.7015").unwrap().to_string(),
            String::from("sip:2222;phone-context=unknown@10.219.12.179:5060;user=phone;transport=SCTP;yop=00.00.D23F7134.0000.7015")
        );
    }

    #[test]
    fn actual2() {
        assert_eq!(
            Uri::try_from("sip:2222;phone-context=unknown;voicexml=http%3A//10.220.90.229%3A8080/web-ivr-api-1.0/vxml/Dostepgoscia_pojedynczy_48514748636.vxml@10.219.12.179:5060;user=phone;transport=SCTP;yop=00.00.D23F7134.0000.7015").unwrap().to_string(),
            String::from("sip:2222;phone-context=unknown;voicexml=http%3A//10.220.90.229%3A8080/web-ivr-api-1.0/vxml/Dostepgoscia_pojedynczy_48514748636.vxml@10.219.12.179:5060;user=phone;transport=SCTP;yop=00.00.D23F7134.0000.7015")
        );
    }
}

mod display {
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            Uri {
                scheme: None,
                auth: None,
                host_with_port: ("server2.com", Option::<u16>::None).into(),
                params: Default::default(),
                headers: Default::default()
            }
            .to_string(),
            String::from("server2.com")
        );
    }

    #[test]
    fn display2() {
        assert_eq!(
            Uri {
                scheme: None,
                auth: Some(("user", Option::<String>::None).into()),
                host_with_port: ("server2.com", Option::<u16>::None).into(),
                params: Default::default(),
                headers: Default::default()
            }
            .to_string(),
            String::from("user@server2.com")
        );
    }

    #[test]
    fn display3() {
        assert_eq!(
            Uri {
                scheme: None,
                auth: Some(("user", Some("password")).into()),
                host_with_port: ("server2.com", Option::<u16>::None).into(),
                params: Default::default(),
                headers: Default::default()
            }
            .to_string(),
            String::from("user:password@server2.com")
        );
    }

    #[test]
    fn display4() {
        assert_eq!(
            Uri {
                scheme: None,
                auth: Some(("user", Some("password")).into()),
                host_with_port: ("server2.com", Some(5060)).into(),
                params: Default::default(),
                headers: Default::default()
            }
            .to_string(),
            String::from("user:password@server2.com:5060")
        );
    }

    #[test]
    fn display5() {
        assert_eq!(
            Uri {
                scheme: Some(Scheme::Sips),
                auth: None,
                host_with_port: ("client.biloxi.example.com", Some(5061)).into(),
                params: vec![
                    Param::Maddr(Maddr::new("255.255.255.0")),
                    Param::Other("foo".into(), Some("192.0.2.201".into())),
                ],
                headers: Default::default()
            }
            .to_string(),
            String::from("sips:client.biloxi.example.com:5061;maddr=255.255.255.0;foo=192.0.2.201")
        );
    }
}

mod parser {
    use super::*;

    #[test]
    fn parser1() {
        assert_eq!(
            Tokenizer {
                scheme: None,
                auth: None,
                host_with_port: ("server2.com".as_bytes(), None).into(),
                params: vec![],
                headers: None,
                ..Default::default()
            }
            .try_into(),
            Ok(Uri {
                scheme: None,
                auth: None,
                host_with_port: ("server2.com", Option::<u16>::None).into(),
                params: Default::default(),
                headers: Default::default()
            })
        );
    }

    #[test]
    fn parser2() {
        assert_eq!(
            Tokenizer {
                scheme: None,
                auth: Some(("user".as_bytes(), None).into()),
                host_with_port: ("server2.com".as_bytes(), None).into(),
                params: vec![],
                headers: None,
                ..Default::default()
            }
            .try_into(),
            Ok(Uri {
                scheme: None,
                auth: Some(("user", Option::<String>::None).into()),
                host_with_port: ("server2.com", Option::<u16>::None).into(),
                params: Default::default(),
                headers: Default::default()
            })
        );
    }

    #[test]
    fn parser3() {
        assert_eq!(
            Tokenizer {
                scheme: None,
                auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                host_with_port: ("server2.com".as_bytes(), None).into(),
                params: vec![],
                headers: None,
                ..Default::default()
            }
            .try_into(),
            Ok(Uri {
                scheme: None,
                auth: Some(("user", Some("password")).into()),
                host_with_port: ("server2.com", Option::<u16>::None).into(),
                params: Default::default(),
                headers: Default::default()
            })
        );
    }

    #[test]
    fn parser4() {
        assert_eq!(
            Tokenizer {
                scheme: None,
                auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                params: vec![],
                headers: None,
                ..Default::default()
            }
            .try_into(),
            Ok(Uri {
                scheme: None,
                auth: Some(("user", Some("password")).into()),
                host_with_port: ("server2.com", Some(5060)).into(),
                params: Default::default(),
                headers: Default::default()
            })
        );
    }

    #[test]
    fn parser5() {
        assert_eq!(
            Tokenizer {
                scheme: Some("sip".as_bytes().into()),
                auth: Some(("user".as_bytes(), None).into()),
                host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                params: vec![],
                headers: None,
                ..Default::default()
            }
            .try_into(),
            Ok(Uri {
                scheme: Some(Scheme::Sip),
                auth: Some(("user", Option::<String>::None).into()),
                host_with_port: ("server2.com", Some(5060)).into(),
                params: Default::default(),
                headers: Default::default()
            })
        );
    }

    #[test]
    fn parser6() {
        assert_eq!(
            Tokenizer {
                scheme: Some("sip".as_bytes().into()),
                auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                params: vec![],
                headers: None,
                ..Default::default()
            }
            .try_into(),
            Ok(Uri {
                scheme: Some(Scheme::Sip),
                auth: Some(("user", Some("password")).into()),
                host_with_port: ("server2.com", Some(5060)).into(),
                params: Default::default(),
                headers: Default::default()
            })
        );
    }

    #[test]
    fn parser7() {
        assert_eq!(
            Tokenizer {
                scheme: Some("sips".as_bytes().into()),
                auth: None,
                host_with_port: (
                    "client.biloxi.example.com".as_bytes(),
                    Some("5061".as_bytes())
                )
                    .into(),
                params: vec![
                    ("maddr".as_bytes(), Some("255.255.255.0".as_bytes())).into(),
                    ("foo".as_bytes(), Some("192.0.2.201".as_bytes())).into()
                ],
                headers: None,
                ..Default::default()
            }
            .try_into(),
            Ok(Uri {
                scheme: Some(Scheme::Sips),
                auth: None,
                host_with_port: ("client.biloxi.example.com", Some(5061)).into(),
                params: vec![
                    Param::Maddr(Maddr::new("255.255.255.0")),
                    Param::Other("foo".into(), Some("192.0.2.201".into())),
                ],
                headers: Default::default()
            })
        );
    }

    #[cfg(feature = "test-utils")]
    #[test]
    fn parser_cycle() {
        use testing_utils::Randomize;

        let uri = Uri::random();
        let uri_raw = uri.to_string();
        assert_eq!(
            Tokenizer::tokenize(uri_raw.as_str()).unwrap().1.try_into(),
            Ok(uri)
        );
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1_u8() {
        assert_eq!(
            Tokenizer::tokenize("server2.com something".as_bytes()),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: None,
                    auth: None,
                    host_with_port: ("server2.com".as_bytes(), None).into(),
                    params: vec![],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer1_str() {
        assert_eq!(
            Tokenizer::tokenize("server2.com something"),
            Ok((
                " something",
                Tokenizer {
                    scheme: None,
                    auth: None,
                    host_with_port: ("server2.com", None).into(),
                    params: vec![],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer2_u8() {
        assert_eq!(
            Tokenizer::tokenize("user@server2.com something".as_bytes()),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: None,
                    auth: Some(("user".as_bytes(), None).into()),
                    host_with_port: ("server2.com".as_bytes(), None).into(),
                    params: vec![],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer3_u8() {
        assert_eq!(
            Tokenizer::tokenize("user:password@server2.com something".as_bytes()),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: None,
                    auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                    host_with_port: ("server2.com".as_bytes(), None).into(),
                    params: vec![],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer4_u8() {
        assert_eq!(
            Tokenizer::tokenize("user:password@server2.com:5060 something".as_bytes()),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: None,
                    auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                    host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                    params: vec![],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer5_u8() {
        assert_eq!(
            Tokenizer::tokenize("sip:user@server2.com:5060 something".as_bytes()),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sip".as_bytes().into()),
                    auth: Some(("user".as_bytes(), None).into()),
                    host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                    params: vec![],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer6_u8() {
        assert_eq!(
            Tokenizer::tokenize("sip:user:password@server2.com:5060 something".as_bytes()),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sip".as_bytes().into()),
                    auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                    host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                    params: vec![],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer6_u88() {
        assert_eq!(
            Tokenizer::tokenize("sips:user:password@server2.com:5060 something".as_bytes()),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: Some(("user".as_bytes(), Some("password".as_bytes())).into()),
                    host_with_port: ("server2.com".as_bytes(), Some("5060".as_bytes())).into(),
                    params: vec![],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer7_u8() {
        assert_eq!(
            Tokenizer::tokenize("sips:ss2.biloxi.example.com something".as_bytes()),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: ("ss2.biloxi.example.com".as_bytes(), None).into(),
                    params: vec![],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer8_u8() {
        assert_eq!(
            Tokenizer::tokenize(
                "sips:ss2.biloxi.example.com;maddr=255.255.255.0 something".as_bytes()
            ),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: ("ss2.biloxi.example.com".as_bytes(), None).into(),
                    params: vec![("maddr".as_bytes(), Some("255.255.255.0".as_bytes())).into()],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer9_u8() {
        assert_eq!(
            Tokenizer::tokenize("sips:client.biloxi.example.com:5061 something".as_bytes()),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: (
                        "client.biloxi.example.com".as_bytes(),
                        Some("5061".as_bytes())
                    )
                        .into(),
                    params: vec![],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer10_u8() {
        assert_eq!(
            Tokenizer::tokenize(
                "sips:client.biloxi.example.com:5061;maddr=255.255.255.0;foo=192.0.2.201 something"
                    .as_bytes()
            ),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: (
                        "client.biloxi.example.com".as_bytes(),
                        Some("5061".as_bytes())
                    )
                        .into(),
                    params: vec![
                        ("maddr".as_bytes(), Some("255.255.255.0".as_bytes())).into(),
                        ("foo".as_bytes(), Some("192.0.2.201".as_bytes())).into()
                    ],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer11_u8() {
        assert_eq!(
            Tokenizer::tokenize(
                "sips:client.biloxi.example.com:5061;maddr=255.255.255.0;foo=192.0.2.201;lr something".as_bytes()
            ),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: (
                        "client.biloxi.example.com".as_bytes(),
                        Some("5061".as_bytes())
                    )
                        .into(),
                    params: vec![
                        ("maddr".as_bytes(), Some("255.255.255.0".as_bytes())).into(),
                        ("foo".as_bytes(), Some("192.0.2.201".as_bytes())).into(),
                        ("lr".as_bytes(), None).into()
                    ],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer12_u8() {
        assert_eq!(
            Tokenizer::tokenize(
                "sips:client.biloxi.example.com:5061;maddr=[255.255.255.0];foo=192.0.2.201;lr something".as_bytes()
            ),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sips".as_bytes().into()),
                    auth: None,
                    host_with_port: (
                        "client.biloxi.example.com".as_bytes(),
                        Some("5061".as_bytes())
                    )
                        .into(),
                    params: vec![
                        ("maddr".as_bytes(), Some("[255.255.255.0]".as_bytes())).into(),
                        ("foo".as_bytes(), Some("192.0.2.201".as_bytes())).into(),
                        ("lr".as_bytes(), None).into()
                    ],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }

    #[test]
    fn tokenizer13_u8() {
        assert_eq!(
            // Tokenizer::tokenize(
            //     "sip:2222;phone-context=unknown;voicexml=http%3A//10.220.90.229%3A8080/web-ivr-api-1.0/vxml/Dostepgoscia_pojedynczy_48514748636.vxml@10.219.12.179:5060;user=phone;transport=SCTP;yop=00.00.D23F7134.0000.7015 something".as_bytes()
            // ),
            Tokenizer::tokenize("sip:2222;user:password@10.219.12.179:5060;user=phone;transport=SCTP;yop=00.00.D23F7134.0000.7015 something".as_bytes()),
            Ok((
                " something".as_bytes(),
                Tokenizer {
                    scheme: Some("sip".as_bytes().into()),
                    auth: Some(("2222;user".as_bytes(), Some("password".as_bytes())).into()),
                    host_with_port: ("10.219.12.179".as_bytes(), Some("5060".as_bytes())).into(),
                    params: vec![
                        ("user".as_bytes(), Some("phone".as_bytes())).into(),
                        ("transport".as_bytes(), Some("SCTP".as_bytes())).into(),
                        ("yop".as_bytes(), Some("00.00.D23F7134.0000.7015".as_bytes())).into(),
                    ],
                    headers: None,
                    ..Default::default()
                }
            )),
        );
    }
}
