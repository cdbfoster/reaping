//
// This file is part of The Reaping.
//
// The Reaping is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// The Reaping is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with The Reaping. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright 2016 Chris Foster
//

macro_rules! log {
    ($fmt:expr) => (println!(concat!("[Log]: ", $fmt)));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("[Log]: ", $fmt), $($arg)*));
}

macro_rules! debug {
    ($fmt:expr) => {
        if cfg!(debug_assertions) { println!(concat!("[Debug]: ", $fmt)); }
    };
    ($fmt:expr, $($arg:tt)*) => {
        if cfg!(debug_assertions) { println!(concat!("[Debug]: ", $fmt), $($arg)*); }
    };
}
