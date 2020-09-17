type bits_t = u32;

trait BitsT {}
impl BitsT for bits_t {}

trait Bits<T: BitsT> {
    fn is_write_set(self) -> bool;
    fn is_read_set(self) -> bool;
    fn is_owner_set(self) -> bool;
    fn is_group_set(self) -> bool;
    fn is_execute_set(self) -> bool;
    fn is_other_set(self) -> bool;
    // fn is_executable_by_someone
    // io::Result
}
