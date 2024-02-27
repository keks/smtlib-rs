use smtlib_derive::SmtDatatype;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(SmtDatatype)]
struct Foo {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
