use std::fmt;




mod my {
	pub struct WhiteBox<T> {
		pub contents: T
	}

	//#[allow(dead_code)]
	#[derive(Debug)]
	pub struct BlackBox<T> {
		contents: T
	}

	impl<T> BlackBox<T> {
		pub fn new(contents: T) -> BlackBox<T> {
			BlackBox {
				contents: contents 
			}
		}
	}
}

fn main() {

	println!("hello world");
	println!("hello world {}", "me");
	println!("hello world {0}", "me");
	println!("hello world {me}", me="me");
	println!("hello world {1}, {}, {me}", "me", "1", me="abc");

	#[derive(Debug)]
	struct S(i32);
	println!("{:?}", S(3));

	#[derive(Debug)]
	struct S1(i32);

	impl fmt::Display for S1 {
		fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
			write!(f, "{}", self.0)
		}
	}
	println!("{}, {:?}", S1(4), S1(4));

	struct List(Vec<i32>);

	impl fmt::Display for List {
		fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
			let List(ref vec) = *self;
			let len = vec.len();

			for (count, v) in vec.iter().enumerate() {
				if count < len - 1 { try!(write!(f, "{}, ", v)) }
			}

			write!(f, "{}", vec[len-1])
		}
	}


	let v: List = List(vec![1, 2]);
	println!("{}", v);

	// public private 

	let wbox = my::WhiteBox { contents: "pub" };

	println!("wbox {}", wbox.contents);

	// let bbox = my::BlackBox {contents: "priv contents"};

	let _bbox = my::BlackBox::new("priv contents");
	println!("black box {:?}", _bbox);



}
