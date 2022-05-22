pub use test_mod::*;
#[allow(clippy::too_many_arguments)]
mod test_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "Test was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TEST_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"msg\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"WARNING_Deprecated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"val\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"val\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"val\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"logs\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_TEST\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"min\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"max\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"bound\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"result\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"adjust\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"what\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"args\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployCode\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"what\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployCode\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"addr\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"failed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"hoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"time\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rewind\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"time\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"skip\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"origin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startHoax\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"give\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"tip\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vm\",\"outputs\":[{\"internalType\":\"contract Vm\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct Test<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Test<M> {
        fn clone(&self) -> Self {
            Test(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Test<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Test<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Test))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Test<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), TEST_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `IS_TEST` (0xfa7626d4) function"]
        pub fn is_test(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bound` (0x5a6c1eed) function"]
        pub fn bound(
            &self,
            x: ethers::core::types::U256,
            min: ethers::core::types::U256,
            max: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([90, 108, 30, 237], (x, min, max))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deal` (0x6bce989b) function"]
        pub fn deal_with_token(
            &self,
            token: ethers::core::types::Address,
            to: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 206, 152, 155], (token, to, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deal` (0x97754ae9) function"]
        pub fn deal_with_token_and_adjust(
            &self,
            token: ethers::core::types::Address,
            to: ethers::core::types::Address,
            give: ethers::core::types::U256,
            adjust: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 117, 74, 233], (token, to, give, adjust))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deal` (0xc88a5e6d) function"]
        pub fn deal(
            &self,
            to: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 138, 94, 109], (to, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deployCode` (0x29ce9dde) function"]
        pub fn deploy_code_with_args(
            &self,
            what: String,
            args: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([41, 206, 157, 222], (what, args))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deployCode` (0x9a8325a0) function"]
        pub fn deploy_code(
            &self,
            what: String,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([154, 131, 37, 160], what)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `failed` (0xba414fa6) function"]
        pub fn failed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hoax` (0x233240ee) function"]
        pub fn hoax_0(
            &self,
            who: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 50, 64, 238], who)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hoax` (0x29a9e300) function"]
        pub fn hoax_2(
            &self,
            who: ethers::core::types::Address,
            origin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 169, 227, 0], (who, origin))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hoax` (0xaf9bbe5f) function"]
        pub fn hoax_3(
            &self,
            who: ethers::core::types::Address,
            origin: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([175, 155, 190, 95], (who, origin, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hoax` (0xe9a79a7b) function"]
        pub fn hoax_4(
            &self,
            who: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 167, 154, 123], (who, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rewind` (0x2d6c17a3) function"]
        pub fn rewind(
            &self,
            time: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 108, 23, 163], time)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `skip` (0xb9c071b4) function"]
        pub fn skip(
            &self,
            time: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 192, 113, 180], time)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startHoax` (0x108554f2) function"]
        pub fn start_hoax_1(
            &self,
            who: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 133, 84, 242], (who, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startHoax` (0x3bf82db1) function"]
        pub fn start_hoax_2(
            &self,
            who: ethers::core::types::Address,
            origin: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 248, 45, 177], (who, origin, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startHoax` (0x6f597075) function"]
        pub fn start_hoax_0(
            &self,
            who: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 89, 112, 117], who)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startHoax` (0xd06d8229) function"]
        pub fn start_hoax_4(
            &self,
            who: ethers::core::types::Address,
            origin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 109, 130, 41], (who, origin))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tip` (0xd82555f1) function"]
        pub fn tip(
            &self,
            token: ethers::core::types::Address,
            to: ethers::core::types::Address,
            give: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 37, 85, 241], (token, to, give))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vm` (0x3a768463) function"]
        pub fn vm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([58, 118, 132, 99], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `WARNING_Deprecated` event"]
        pub fn warning_deprecated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, WarningDeprecatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log` event"]
        pub fn log_filter(&self) -> ethers::contract::builders::Event<M, LogFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_address` event"]
        pub fn log_address_filter(&self) -> ethers::contract::builders::Event<M, LogAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_bytes` event"]
        pub fn log_bytes_filter(&self) -> ethers::contract::builders::Event<M, LogBytesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_bytes32` event"]
        pub fn log_bytes_32_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_int` event"]
        pub fn log_int_filter(&self) -> ethers::contract::builders::Event<M, LogIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_address` event"]
        pub fn log_named_address_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_bytes` event"]
        pub fn log_named_bytes_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedBytesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_bytes32` event"]
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_decimal_int` event"]
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedDecimalIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_decimal_uint` event"]
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedDecimalUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_int` event"]
        pub fn log_named_int_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_string` event"]
        pub fn log_named_string_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedStringFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_uint` event"]
        pub fn log_named_uint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_string` event"]
        pub fn log_string_filter(&self) -> ethers::contract::builders::Event<M, LogStringFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_uint` event"]
        pub fn log_uint_filter(&self) -> ethers::contract::builders::Event<M, LogUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `logs` event"]
        pub fn logs_filter(&self) -> ethers::contract::builders::Event<M, LogsFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, TestEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Test<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "WARNING_Deprecated", abi = "WARNING_Deprecated(string)")]
    pub struct WarningDeprecatedFilter {
        pub msg: String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub String);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ethers::core::types::Address);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ethers::core::types::Bytes);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub I256);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: String,
        pub val: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: String,
        pub val: ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: String,
        pub val: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: String,
        pub val: I256,
        pub decimals: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: String,
        pub val: ethers::core::types::U256,
        pub decimals: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: String,
        pub val: I256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: String,
        pub val: String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: String,
        pub val: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub String);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ethers::core::types::U256);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ethers::core::types::Bytes);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TestEvents {
        WarningDeprecatedFilter(WarningDeprecatedFilter),
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ethers::contract::EthLogDecode for TestEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = WarningDeprecatedFilter::decode_log(log) {
                return Ok(TestEvents::WarningDeprecatedFilter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(TestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(TestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(TestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(TestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(TestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(TestEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(TestEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(TestEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(TestEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(TestEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(TestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(TestEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(TestEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(TestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(TestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(TestEvents::LogsFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for TestEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TestEvents::WarningDeprecatedFilter(element) => element.fmt(f),
                TestEvents::LogFilter(element) => element.fmt(f),
                TestEvents::LogAddressFilter(element) => element.fmt(f),
                TestEvents::LogBytesFilter(element) => element.fmt(f),
                TestEvents::LogBytes32Filter(element) => element.fmt(f),
                TestEvents::LogIntFilter(element) => element.fmt(f),
                TestEvents::LogNamedAddressFilter(element) => element.fmt(f),
                TestEvents::LogNamedBytesFilter(element) => element.fmt(f),
                TestEvents::LogNamedBytes32Filter(element) => element.fmt(f),
                TestEvents::LogNamedDecimalIntFilter(element) => element.fmt(f),
                TestEvents::LogNamedDecimalUintFilter(element) => element.fmt(f),
                TestEvents::LogNamedIntFilter(element) => element.fmt(f),
                TestEvents::LogNamedStringFilter(element) => element.fmt(f),
                TestEvents::LogNamedUintFilter(element) => element.fmt(f),
                TestEvents::LogStringFilter(element) => element.fmt(f),
                TestEvents::LogUintFilter(element) => element.fmt(f),
                TestEvents::LogsFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `IS_TEST`function with signature `IS_TEST()` and selector `[250, 118, 38, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    #[doc = "Container type for all input parameters for the `bound`function with signature `bound(uint256,uint256,uint256)` and selector `[90, 108, 30, 237]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "bound", abi = "bound(uint256,uint256,uint256)")]
    pub struct BoundCall {
        pub x: ethers::core::types::U256,
        pub min: ethers::core::types::U256,
        pub max: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deal`function with signature `deal(address,address,uint256)` and selector `[107, 206, 152, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deal", abi = "deal(address,address,uint256)")]
    pub struct DealWithTokenCall {
        pub token: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deal`function with signature `deal(address,address,uint256,bool)` and selector `[151, 117, 74, 233]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deal", abi = "deal(address,address,uint256,bool)")]
    pub struct DealWithTokenAndAdjustCall {
        pub token: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
        pub adjust: bool,
    }
    #[doc = "Container type for all input parameters for the `deal`function with signature `deal(address,uint256)` and selector `[200, 138, 94, 109]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deal", abi = "deal(address,uint256)")]
    pub struct DealCall {
        pub to: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deployCode`function with signature `deployCode(string,bytes)` and selector `[41, 206, 157, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deployCode", abi = "deployCode(string,bytes)")]
    pub struct DeployCodeWithArgsCall {
        pub what: String,
        pub args: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `deployCode`function with signature `deployCode(string)` and selector `[154, 131, 37, 160]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deployCode", abi = "deployCode(string)")]
    pub struct DeployCodeCall {
        pub what: String,
    }
    #[doc = "Container type for all input parameters for the `failed`function with signature `failed()` and selector `[186, 65, 79, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    #[doc = "Container type for all input parameters for the `hoax`function with signature `hoax(address)` and selector `[35, 50, 64, 238]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hoax", abi = "hoax(address)")]
    pub struct Hoax0Call {
        pub who: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hoax`function with signature `hoax(address,address)` and selector `[41, 169, 227, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hoax", abi = "hoax(address,address)")]
    pub struct Hoax2Call {
        pub who: ethers::core::types::Address,
        pub origin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hoax`function with signature `hoax(address,address,uint256)` and selector `[175, 155, 190, 95]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hoax", abi = "hoax(address,address,uint256)")]
    pub struct Hoax3Call {
        pub who: ethers::core::types::Address,
        pub origin: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `hoax`function with signature `hoax(address,uint256)` and selector `[233, 167, 154, 123]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hoax", abi = "hoax(address,uint256)")]
    pub struct Hoax4Call {
        pub who: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `rewind`function with signature `rewind(uint256)` and selector `[45, 108, 23, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rewind", abi = "rewind(uint256)")]
    pub struct RewindCall {
        pub time: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `skip`function with signature `skip(uint256)` and selector `[185, 192, 113, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "skip", abi = "skip(uint256)")]
    pub struct SkipCall {
        pub time: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `startHoax`function with signature `startHoax(address,uint256)` and selector `[16, 133, 84, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "startHoax", abi = "startHoax(address,uint256)")]
    pub struct StartHoax1Call {
        pub who: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `startHoax`function with signature `startHoax(address,address,uint256)` and selector `[59, 248, 45, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "startHoax", abi = "startHoax(address,address,uint256)")]
    pub struct StartHoax2Call {
        pub who: ethers::core::types::Address,
        pub origin: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `startHoax`function with signature `startHoax(address)` and selector `[111, 89, 112, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "startHoax", abi = "startHoax(address)")]
    pub struct StartHoax0Call {
        pub who: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `startHoax`function with signature `startHoax(address,address)` and selector `[208, 109, 130, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "startHoax", abi = "startHoax(address,address)")]
    pub struct StartHoax4Call {
        pub who: ethers::core::types::Address,
        pub origin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `tip`function with signature `tip(address,address,uint256)` and selector `[216, 37, 85, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tip", abi = "tip(address,address,uint256)")]
    pub struct TipCall {
        pub token: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub give: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `vm`function with signature `vm()` and selector `[58, 118, 132, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "vm", abi = "vm()")]
    pub struct VmCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TestCalls {
        IsTest(IsTestCall),
        Bound(BoundCall),
        DealWithToken(DealWithTokenCall),
        DealWithTokenAndAdjust(DealWithTokenAndAdjustCall),
        Deal(DealCall),
        DeployCodeWithArgs(DeployCodeWithArgsCall),
        DeployCode(DeployCodeCall),
        Failed(FailedCall),
        Hoax0(Hoax0Call),
        Hoax2(Hoax2Call),
        Hoax3(Hoax3Call),
        Hoax4(Hoax4Call),
        Rewind(RewindCall),
        Skip(SkipCall),
        StartHoax1(StartHoax1Call),
        StartHoax2(StartHoax2Call),
        StartHoax0(StartHoax0Call),
        StartHoax4(StartHoax4Call),
        Tip(TipCall),
        Vm(VmCall),
    }
    impl ethers::core::abi::AbiDecode for TestCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <IsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::IsTest(decoded));
            }
            if let Ok(decoded) = <BoundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::Bound(decoded));
            }
            if let Ok(decoded) =
                <DealWithTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::DealWithToken(decoded));
            }
            if let Ok(decoded) =
                <DealWithTokenAndAdjustCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::DealWithTokenAndAdjust(decoded));
            }
            if let Ok(decoded) = <DealCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(TestCalls::Deal(decoded));
            }
            if let Ok(decoded) =
                <DeployCodeWithArgsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::DeployCodeWithArgs(decoded));
            }
            if let Ok(decoded) =
                <DeployCodeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::DeployCode(decoded));
            }
            if let Ok(decoded) = <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::Failed(decoded));
            }
            if let Ok(decoded) = <Hoax0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::Hoax0(decoded));
            }
            if let Ok(decoded) = <Hoax2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::Hoax2(decoded));
            }
            if let Ok(decoded) = <Hoax3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::Hoax3(decoded));
            }
            if let Ok(decoded) = <Hoax4Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::Hoax4(decoded));
            }
            if let Ok(decoded) = <RewindCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::Rewind(decoded));
            }
            if let Ok(decoded) = <SkipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(TestCalls::Skip(decoded));
            }
            if let Ok(decoded) =
                <StartHoax1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::StartHoax1(decoded));
            }
            if let Ok(decoded) =
                <StartHoax2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::StartHoax2(decoded));
            }
            if let Ok(decoded) =
                <StartHoax0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::StartHoax0(decoded));
            }
            if let Ok(decoded) =
                <StartHoax4Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TestCalls::StartHoax4(decoded));
            }
            if let Ok(decoded) = <TipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(TestCalls::Tip(decoded));
            }
            if let Ok(decoded) = <VmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(TestCalls::Vm(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                TestCalls::IsTest(element) => element.encode(),
                TestCalls::Bound(element) => element.encode(),
                TestCalls::DealWithToken(element) => element.encode(),
                TestCalls::DealWithTokenAndAdjust(element) => element.encode(),
                TestCalls::Deal(element) => element.encode(),
                TestCalls::DeployCodeWithArgs(element) => element.encode(),
                TestCalls::DeployCode(element) => element.encode(),
                TestCalls::Failed(element) => element.encode(),
                TestCalls::Hoax0(element) => element.encode(),
                TestCalls::Hoax2(element) => element.encode(),
                TestCalls::Hoax3(element) => element.encode(),
                TestCalls::Hoax4(element) => element.encode(),
                TestCalls::Rewind(element) => element.encode(),
                TestCalls::Skip(element) => element.encode(),
                TestCalls::StartHoax1(element) => element.encode(),
                TestCalls::StartHoax2(element) => element.encode(),
                TestCalls::StartHoax0(element) => element.encode(),
                TestCalls::StartHoax4(element) => element.encode(),
                TestCalls::Tip(element) => element.encode(),
                TestCalls::Vm(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for TestCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TestCalls::IsTest(element) => element.fmt(f),
                TestCalls::Bound(element) => element.fmt(f),
                TestCalls::DealWithToken(element) => element.fmt(f),
                TestCalls::DealWithTokenAndAdjust(element) => element.fmt(f),
                TestCalls::Deal(element) => element.fmt(f),
                TestCalls::DeployCodeWithArgs(element) => element.fmt(f),
                TestCalls::DeployCode(element) => element.fmt(f),
                TestCalls::Failed(element) => element.fmt(f),
                TestCalls::Hoax0(element) => element.fmt(f),
                TestCalls::Hoax2(element) => element.fmt(f),
                TestCalls::Hoax3(element) => element.fmt(f),
                TestCalls::Hoax4(element) => element.fmt(f),
                TestCalls::Rewind(element) => element.fmt(f),
                TestCalls::Skip(element) => element.fmt(f),
                TestCalls::StartHoax1(element) => element.fmt(f),
                TestCalls::StartHoax2(element) => element.fmt(f),
                TestCalls::StartHoax0(element) => element.fmt(f),
                TestCalls::StartHoax4(element) => element.fmt(f),
                TestCalls::Tip(element) => element.fmt(f),
                TestCalls::Vm(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsTestCall> for TestCalls {
        fn from(var: IsTestCall) -> Self {
            TestCalls::IsTest(var)
        }
    }
    impl ::std::convert::From<BoundCall> for TestCalls {
        fn from(var: BoundCall) -> Self {
            TestCalls::Bound(var)
        }
    }
    impl ::std::convert::From<DealWithTokenCall> for TestCalls {
        fn from(var: DealWithTokenCall) -> Self {
            TestCalls::DealWithToken(var)
        }
    }
    impl ::std::convert::From<DealWithTokenAndAdjustCall> for TestCalls {
        fn from(var: DealWithTokenAndAdjustCall) -> Self {
            TestCalls::DealWithTokenAndAdjust(var)
        }
    }
    impl ::std::convert::From<DealCall> for TestCalls {
        fn from(var: DealCall) -> Self {
            TestCalls::Deal(var)
        }
    }
    impl ::std::convert::From<DeployCodeWithArgsCall> for TestCalls {
        fn from(var: DeployCodeWithArgsCall) -> Self {
            TestCalls::DeployCodeWithArgs(var)
        }
    }
    impl ::std::convert::From<DeployCodeCall> for TestCalls {
        fn from(var: DeployCodeCall) -> Self {
            TestCalls::DeployCode(var)
        }
    }
    impl ::std::convert::From<FailedCall> for TestCalls {
        fn from(var: FailedCall) -> Self {
            TestCalls::Failed(var)
        }
    }
    impl ::std::convert::From<Hoax0Call> for TestCalls {
        fn from(var: Hoax0Call) -> Self {
            TestCalls::Hoax0(var)
        }
    }
    impl ::std::convert::From<Hoax2Call> for TestCalls {
        fn from(var: Hoax2Call) -> Self {
            TestCalls::Hoax2(var)
        }
    }
    impl ::std::convert::From<Hoax3Call> for TestCalls {
        fn from(var: Hoax3Call) -> Self {
            TestCalls::Hoax3(var)
        }
    }
    impl ::std::convert::From<Hoax4Call> for TestCalls {
        fn from(var: Hoax4Call) -> Self {
            TestCalls::Hoax4(var)
        }
    }
    impl ::std::convert::From<RewindCall> for TestCalls {
        fn from(var: RewindCall) -> Self {
            TestCalls::Rewind(var)
        }
    }
    impl ::std::convert::From<SkipCall> for TestCalls {
        fn from(var: SkipCall) -> Self {
            TestCalls::Skip(var)
        }
    }
    impl ::std::convert::From<StartHoax1Call> for TestCalls {
        fn from(var: StartHoax1Call) -> Self {
            TestCalls::StartHoax1(var)
        }
    }
    impl ::std::convert::From<StartHoax2Call> for TestCalls {
        fn from(var: StartHoax2Call) -> Self {
            TestCalls::StartHoax2(var)
        }
    }
    impl ::std::convert::From<StartHoax0Call> for TestCalls {
        fn from(var: StartHoax0Call) -> Self {
            TestCalls::StartHoax0(var)
        }
    }
    impl ::std::convert::From<StartHoax4Call> for TestCalls {
        fn from(var: StartHoax4Call) -> Self {
            TestCalls::StartHoax4(var)
        }
    }
    impl ::std::convert::From<TipCall> for TestCalls {
        fn from(var: TipCall) -> Self {
            TestCalls::Tip(var)
        }
    }
    impl ::std::convert::From<VmCall> for TestCalls {
        fn from(var: VmCall) -> Self {
            TestCalls::Vm(var)
        }
    }
}
