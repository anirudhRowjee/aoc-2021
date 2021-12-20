use anyhow::Result;
use std::str::FromStr;

// our first library function - better input/output handling
pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>>
// here's what's called a Trait Bound - this lets
// us restrict to using this function only on types where the
// FromStr trait is implemented
where
    T: FromStr,
{
    // read a file from the path given, split by newline
    // and then parse into a vector
    Ok(
        std::fs::read_to_string(path)?
        .split("\n")
        // this only returns those mapped values where a Some() of the type 
        // is returned.
        .filter_map(|line| line.parse::<T>().ok())
        .collect()
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
