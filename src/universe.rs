

use vec2dim::Vec2d;
use vec2dim::WrappingVec2d;

use rand::prelude::*;


type CellState = i8;
const EMPTY: CellState = 0;

// Initializer functions
fn random_initializer(row: usize, col: usize) -> CellState {
	random()
}

trait Universe {
	fn step(&mut self);
	fn array(&self) -> &[CellState];
}

pub struct WrappingUniverse {
	data: WrappingVec2d<CellState>,
}

impl WrappingUniverse {
	pub fn create(rows: usize, cols: usize) -> WrappingUniverse {
		let data = Vec2d::new_with_value(rows, cols, EMPTY);
		WrappingUniverse {
			data: WrappingVec2d::from_vec2d(data),
		}
	}
}

impl Universe for WrappingUniverse {
	fn step(&mut self) {
	}
	fn array(&self) -> &[CellState] {
		&self.data.array().data
	}
}




mod tests {
	

	#[test]
	fn test_rand_init() {
		let data = Vec2d::from_fn(10, 8, &random_initializer);
		let _u = WrappingUniverse {
			data: WrappingVec2d::from_vec2d(data),
		};
	}
}