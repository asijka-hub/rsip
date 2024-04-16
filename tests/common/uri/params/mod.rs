use rsip::common::uri::param::{Maddr, Param, Tokenizer};
use std::convert::TryInto;

mod display {
    use rsip::param::{OtherParam, OtherParamValue};
    use super::*;

    #[test]
    fn display1() {
        assert_eq!(
            Param::Maddr(Maddr::new("255.255.255.0")).to_string(),
            String::from(";maddr=255.255.255.0")
        );
    }
}

mod parser {
    use std::convert::TryFrom;
    use rsip::param::{OtherParam, OtherParamValue};
    use super::*;

    #[test]
    fn parser1() {
        assert_eq!(
            Tokenizer::from(("maddr".as_bytes(), Some("255.255.255.0".as_bytes()))).try_into(),
            Ok(Param::Maddr(Maddr::new("255.255.255.0")))
        );
    }

    #[test]
    fn parser2() {
        assert_eq!(
            Tokenizer::from(("maddr".as_bytes(), None,)).try_into(),
            Ok(Param::Other("maddr".into(), None))
        );
    }

    #[test]
    fn other1() {
        assert_eq!(
            Tokenizer::from(("param".as_bytes(), Some("dupa".as_bytes()))).try_into(),
            Ok(Param::Other(OtherParam::new("param"), Some(OtherParamValue::new("dupa"))))
        );
    }

    #[test]
    fn other2() {
        assert_eq!(
            Param::try_from(Tokenizer::tokenize(";param=dupa".as_bytes()).unwrap().1).unwrap().to_string(),
            ";param=dupa"
        );
    }

    #[test]
    fn other3() {
        assert_eq!(
            Param::try_from(Tokenizer::tokenize(";param=//dupa".as_bytes()).unwrap().1).unwrap().to_string(),
            ";param=//dupa"
        );
    }

    #[test]
    fn other4() {
        assert_eq!(
            Param::try_from(Tokenizer::tokenize(";param=[dupa]".as_bytes()).unwrap().1).unwrap().to_string(),
            ";param=[dupa]"
        );
    }

    #[test]
    fn other5() {
        assert_eq!(
            Param::try_from(Tokenizer::tokenize(";param=%[dupa]".as_bytes()).unwrap().1).unwrap().to_string(),
            ";param=%[dupa]"
        );
    }
}

mod tokenizer {
    use super::*;

    #[test]
    fn tokenizer1_u8() {
        assert_eq!(
            Tokenizer::tokenize(";maddr=255.255.255.255;something".as_bytes()),
            Ok((
                ";something".as_bytes(),
                ("maddr".as_bytes(), Some("255.255.255.255".as_bytes())).into()
            )),
        );
    }

    #[test]
    fn tokenizer1_u8_2() {
        assert_eq!(
            Tokenizer::tokenize(";maddr=//[255.255.255.255];something".as_bytes()),
            Ok((
                ";something".as_bytes(),
                ("maddr".as_bytes(), Some("//[255.255.255.255]".as_bytes())).into()
            )),
        );
    }

    #[test]
    fn tokenizer1_str() {
        assert_eq!(
            Tokenizer::tokenize(";maddr=255.255.255.255;something"),
            Ok((";something", ("maddr", Some("255.255.255.255")).into())),
        );
    }

    #[test]
    fn tokenizer2_u8() {
        assert_eq!(
            Tokenizer::tokenize(";maddr=255.255.255.255;something".as_bytes()),
            Ok((
                ";something".as_bytes(),
                ("maddr".as_bytes(), Some("255.255.255.255".as_bytes())).into()
            )),
        );
    }

    #[test]
    fn errors1() {
        assert_eq!(
            Tokenizer::tokenize("hello".as_bytes()),
            Err(nom::Err::Error(rsip::TokenizerError::from(
                "failed to tokenize uri param: hello"
            ))),
        );
    }
}
