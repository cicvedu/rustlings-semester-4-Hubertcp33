// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.



pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE

fn compare_license_types<T:Licensed>(software: T, software_two: T) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software_one = SomeSoftware {};
        let some_software_two = SomeSoftware {};

        assert!(compare_license_types(some_software_one, some_software_two));
    }

    #[test]
    fn compare_license_information_backwards() {
        let other_software_one = OtherSoftware {};
        let other_software_two = OtherSoftware {};

        assert!(compare_license_types(other_software_one, other_software_two));
    }
}
