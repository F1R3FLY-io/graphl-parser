type RhoLangDocument = String;
type GraphDocument = String;

pub fn parse(document: RhoLangDocument) -> GraphDocument {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nil() {
        let document = "{0}";
        let result = parse(document);

        assert_eq!(result, "Nil");
    }
}
