/*
 * <wello-horld.rs: A hello world app (with a twist) written in Rust>
 * Copyright (C) <2023> <Hashiko Hireo>
 *
 * This file is part of wello-horld.rs
 *
 * wello-horld.rs is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or any later version.
 *
 * wello-horld.rs is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with wello-horld.rs. If not, see <https://www.gnu.org/licenses/>.
 */

pub trait Greet {
    fn salute(&self, name: &str) -> String;
}

pub struct Greeter();

impl Greeter {
    pub fn new() -> Self {
        Self()
    }
}

impl Greet for Greeter {
    fn salute(&self, name: &str) -> String {
        format!("Hello, {}!", name)
    }
}
