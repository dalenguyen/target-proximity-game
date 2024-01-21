/// Define a trait to represent printable objects
pub trait Printable {
    fn to_string(&self) -> String;
}