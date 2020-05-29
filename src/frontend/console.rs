// Copyright (C) 2020 iTX Technologies
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::frontend::frontend::FrontEnd;
use crate::util;
use crate::VERSION;

pub struct ConsoleFrontEnd;

impl ConsoleFrontEnd {
    fn log(&self, s: String, prefix: String) {
        println!(" {} [{}] {}", util::get_time_now(), prefix, s);
    }
}

impl FrontEnd for ConsoleFrontEnd {
    fn log_info(&self, s: String) {
        ConsoleFrontEnd::log(self, s, String::from("INFO"));
    }

    fn log_debug(&self, s: String) {
        ConsoleFrontEnd::log(self, s, String::from("DEBUG"));
    }

    fn log_warning(&self, s: String) {
        ConsoleFrontEnd::log(self, s, String::from("WARN"));
    }

    fn log_error(&self, s: String) {
        ConsoleFrontEnd::log(self, s, String::from("ERROR"));
    }

    fn init(&self) {
        ConsoleFrontEnd::log_info(self, format!("Centasus {} by PeratX@iTXTech.org", VERSION));
        ConsoleFrontEnd::log_info(self, String::from("https://github.com/iTXTech/Centasus"));
    }
}
