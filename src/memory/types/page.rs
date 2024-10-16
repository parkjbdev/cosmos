use super::address::{Address, AddressType};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
pub struct PageAddress<ADDRESS_TYPE: AddressType> {
    inner: Address<ADDRESS_TYPE>,
}
