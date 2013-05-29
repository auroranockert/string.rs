#[link(name = "string", vers = "0.1", uuid = "86237a28-e038-4119-a0be-df3fda9521de")];

#[license = "MIT"];
#[crate_type = "lib"];

#[author = "Jens Nockert"];

#[comment = "Simplify working with strings in Rust"];
#[desc = "Additional methods and functions that operate on strings"];

pub trait Split {
    fn split(&self, separator: char) -> ~[~str];
}

impl Split for ~str {
    fn split(&self, separator: char) -> ~[~str] {
        let mut result = ~[];

        for str::each_split_char_no_trailing(*self, separator) |s| {
            if (s.len() > 0) {
                result.push(s.to_owned())
            }
        }

        io::println(fmt!("Split! %?", result));

        return result;
    }
}