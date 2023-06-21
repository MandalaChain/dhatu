use crate::registrar::key_manager::prelude::PublicAddress;

/// traits used to mark and properly encode arbitrary calldata into a pallet function calldata payload.
pub trait ValidateHash {
    /// get the pallet name.
    fn pallet_name() -> &'static str;

    /// get the function name.
    fn function_name() -> &'static str;
}

/// traits used to mark and properly encode rust data structure into
/// a scale encoded byte array.
pub trait ScaleEncodeable {
    /// the implementation of this function should return a scale encoded byte array.
    ///
    /// this is typically done by appending decoded function selector hex string at the start of the byte array.
    /// and then encoding all the function arguments after that.
    ///
    /// # Example
    /// ```no_run
    /// let mut calldata = Vec::new();
    ///
    /// // function selector for some erc721 `transfer` function.
    /// let mut function_selector = Selector::from_raw("0xcfdd9aa2")?;
    ///
    /// // example scale encoded public address.
    /// let mut to =  PublicAddress::from_str("5GHQr1m4X18Y2psW4ysf7XweYnGSGru4JRRHgFVwN4Z4KNcj")?;
    /// let mut to =  subxt::ext::codec::Encode::encode(&to);
    ///     
    /// // example token id
    /// let mut token_id = subxt::ext::codec::Encode::encode(&57);
    ///
    ///
    /// // append function selector
    /// calldata.append(&mut function_selector.encoded());
    ///
    /// // append function arguments in order
    /// calldata.append(&mut to);
    /// calldata.append(&mut token_id);
    /// ```
    fn encode(self) -> Vec<u8>;
}
