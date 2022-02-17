pub use dstest_mod::*;
#[allow(clippy::too_many_arguments)]
mod dstest_mod {
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
    #[doc = "DSTest was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DSTEST_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"type\":\"function\",\"name\":\"IS_TEST\",\"inputs\":[],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"failed\",\"inputs\":[],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"log\",\"inputs\":[{\"name\":\"\",\"type\":\"string\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_address\",\"inputs\":[{\"name\":\"\",\"type\":\"address\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_bytes\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_bytes32\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_int\",\"inputs\":[{\"name\":\"\",\"type\":\"int256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_address\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"address\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_bytes\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"bytes\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_bytes32\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"bytes32\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"int256\",\"indexed\":false},{\"name\":\"decimals\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"decimals\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_int\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"int256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_string\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"string\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_uint\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_string\",\"inputs\":[{\"name\":\"\",\"type\":\"string\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_uint\",\"inputs\":[{\"name\":\"\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"logs\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes\",\"indexed\":false}],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct DSTest<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for DSTest<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for DSTest<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DSTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> DSTest<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), DSTEST_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `IS_TEST` (0xfa7626d4) function"]
        pub fn is_test(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `failed` (0xba414fa6) function"]
        pub fn failed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, DSTestEvents> {
            self.0.event_with_filter(Default::default())
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
    pub enum DSTestEvents {
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
    impl ethers::contract::EthLogDecode for DSTestEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(DSTestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(DSTestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(DSTestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(DSTestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(DSTestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(DSTestEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(DSTestEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(DSTestEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(DSTestEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(DSTestEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(DSTestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(DSTestEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(DSTestEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(DSTestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(DSTestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(DSTestEvents::LogsFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for DSTestEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DSTestEvents::LogFilter(element) => element.fmt(f),
                DSTestEvents::LogAddressFilter(element) => element.fmt(f),
                DSTestEvents::LogBytesFilter(element) => element.fmt(f),
                DSTestEvents::LogBytes32Filter(element) => element.fmt(f),
                DSTestEvents::LogIntFilter(element) => element.fmt(f),
                DSTestEvents::LogNamedAddressFilter(element) => element.fmt(f),
                DSTestEvents::LogNamedBytesFilter(element) => element.fmt(f),
                DSTestEvents::LogNamedBytes32Filter(element) => element.fmt(f),
                DSTestEvents::LogNamedDecimalIntFilter(element) => element.fmt(f),
                DSTestEvents::LogNamedDecimalUintFilter(element) => element.fmt(f),
                DSTestEvents::LogNamedIntFilter(element) => element.fmt(f),
                DSTestEvents::LogNamedStringFilter(element) => element.fmt(f),
                DSTestEvents::LogNamedUintFilter(element) => element.fmt(f),
                DSTestEvents::LogStringFilter(element) => element.fmt(f),
                DSTestEvents::LogUintFilter(element) => element.fmt(f),
                DSTestEvents::LogsFilter(element) => element.fmt(f),
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum DSTestCalls {
        IsTest(IsTestCall),
        Failed(FailedCall),
    }
    impl ethers::core::abi::AbiDecode for DSTestCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <IsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DSTestCalls::IsTest(decoded));
            }
            if let Ok(decoded) = <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DSTestCalls::Failed(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for DSTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                DSTestCalls::IsTest(element) => element.encode(),
                DSTestCalls::Failed(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DSTestCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DSTestCalls::IsTest(element) => element.fmt(f),
                DSTestCalls::Failed(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsTestCall> for DSTestCalls {
        fn from(var: IsTestCall) -> Self {
            DSTestCalls::IsTest(var)
        }
    }
    impl ::std::convert::From<FailedCall> for DSTestCalls {
        fn from(var: FailedCall) -> Self {
            DSTestCalls::Failed(var)
        }
    }
}
