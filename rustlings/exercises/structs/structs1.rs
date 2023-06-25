// structs1.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a hint.

// Create a C-like struct
// Access it exactly like in C
// So assign it to a variable then just set the values in the same way
// that you created it
struct ColorClassicStruct {
    red: i32,
    green: i32,
    blue: i32,
}

// This is a tuple struct so you can access the elements as a tuple
// By first assigning it to a variable and then accessing it normally
// let green = ColorTupleStruct and then access it like this green.0
struct ColorTupleStruct(i32, i32, i32);

// This is a unitlike struct which is a struct that contains no fields
#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
