extern crate clap;
extern crate rpassword;

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// This file is part of Master Password.

// Master Password is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Master Password is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Master Password.  If not, see <http://www.gnu.org/licenses/>.

mod arg_parse;

use std::io::{self, Write};
use rpassword::read_password;

#[allow(unused_must_use)]
fn main() {
    let (
        site,
        user,
        variant,
        template,
        counter,
        algo,
        context
    ) = arg_parse::get_opts();

    print!("Your master password: ");
    io::stdout().flush();
    let password = read_password().unwrap();
}
