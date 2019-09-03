
mod universe;

#[cfg(test)]
mod tests {
	use super::*;

	use universe::WrappingUniverse;

    #[test]
    fn it_works() {
        let u = WrappingUniverse::create(10,10);
    }
}
