multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AdminModule: crate::storage::StorageModule {
    #[only_owner]
    #[endpoint(enableContract)]
    fn enable_contract(&self) {
        self.enabled().set(true)
    }

    #[only_owner]
    #[endpoint(disableContract)]
    fn disable_contract(&self) {
        self.enabled().clear()
    }
}
