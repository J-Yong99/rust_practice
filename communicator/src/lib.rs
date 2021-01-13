pub mod client;

pub mod network;


#[cfg(test)]
mod tests {
	use super::{client,network};
    #[test]
    fn it_works() {
    assert_eq!(2 + 2, 4);
		network::connect();
    }





}
