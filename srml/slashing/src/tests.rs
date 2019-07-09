// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use crate::mock::*;
use runtime_io::with_externalities;
use srml_support::traits::Currency;

type OnSlash = StakingSlasher<Test>;

#[test]
fn slash_nominator_based_on_exposure() {
	with_externalities(&mut ExtBuilder::default()
		.build(),
	|| {
		let mut misconduct = Unresponsive::new(Test);

		let _ = Balances::make_free_balance_be(&11, 3000);
		let _ = Balances::make_free_balance_be(&21, 1000);

		assert_eq!(3000, Balances::free_balance(&11));
		assert_eq!(1000, Balances::free_balance(&21));

		let misbehaved = vec![(11_u64, 1000_u64), (21_u64, 500)];

		report_misconduct(misbehaved, &mut Unresponsive::<Test>(&mut m);

		// now it is end of era
		// (3K - 3 / n) * 1/20
		//
		// (3*2 - 3 / 50) * 0.05 = 0,003
		let misconduct_level = slash::<_, OnSlash, _, _>(&misconduct);
		assert_eq!(2997, Balances::free_balance(&11));
		assert_eq!(999, Balances::free_balance(&21));
	});
}
