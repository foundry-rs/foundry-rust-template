pub use contracttest_mod::*;
#[allow(clippy::too_many_arguments)]
mod contracttest_mod {
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
    #[doc = "ContractTest was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CONTRACTTEST_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"type\":\"function\",\"name\":\"IS_TEST\",\"inputs\":[],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"failed\",\"inputs\":[],\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setUp\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"testExample\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"event\",\"name\":\"log\",\"inputs\":[{\"name\":\"\",\"type\":\"string\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_address\",\"inputs\":[{\"name\":\"\",\"type\":\"address\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_bytes\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_bytes32\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_int\",\"inputs\":[{\"name\":\"\",\"type\":\"int256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_address\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"address\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_bytes\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"bytes\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_bytes32\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"bytes32\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"int256\",\"indexed\":false},{\"name\":\"decimals\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"uint256\",\"indexed\":false},{\"name\":\"decimals\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_int\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"int256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_string\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"string\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_named_uint\",\"inputs\":[{\"name\":\"key\",\"type\":\"string\",\"indexed\":false},{\"name\":\"val\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_string\",\"inputs\":[{\"name\":\"\",\"type\":\"string\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"log_uint\",\"inputs\":[{\"name\":\"\",\"type\":\"uint256\",\"indexed\":false}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"logs\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes\",\"indexed\":false}],\"anonymous\":false}]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct ContractTest<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ContractTest<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ContractTest<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ContractTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ContractTest<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), CONTRACTTEST_ABI.clone(), client);
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
        #[doc = "Calls the contract's `setUp` (0x0a9254e4) function"]
        pub fn set_up(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `testExample` (0x3f5a4a2a) function"]
        pub fn test_example(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 90, 74, 42], ())
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, ContractTestEvents> {
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
    pub enum ContractTestEvents {
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
    impl ethers::contract::EthLogDecode for ContractTestEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(ContractTestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(ContractTestEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(ContractTestEvents::LogsFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ContractTestEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ContractTestEvents::LogFilter(element) => element.fmt(f),
                ContractTestEvents::LogAddressFilter(element) => element.fmt(f),
                ContractTestEvents::LogBytesFilter(element) => element.fmt(f),
                ContractTestEvents::LogBytes32Filter(element) => element.fmt(f),
                ContractTestEvents::LogIntFilter(element) => element.fmt(f),
                ContractTestEvents::LogNamedAddressFilter(element) => element.fmt(f),
                ContractTestEvents::LogNamedBytesFilter(element) => element.fmt(f),
                ContractTestEvents::LogNamedBytes32Filter(element) => element.fmt(f),
                ContractTestEvents::LogNamedDecimalIntFilter(element) => element.fmt(f),
                ContractTestEvents::LogNamedDecimalUintFilter(element) => element.fmt(f),
                ContractTestEvents::LogNamedIntFilter(element) => element.fmt(f),
                ContractTestEvents::LogNamedStringFilter(element) => element.fmt(f),
                ContractTestEvents::LogNamedUintFilter(element) => element.fmt(f),
                ContractTestEvents::LogStringFilter(element) => element.fmt(f),
                ContractTestEvents::LogUintFilter(element) => element.fmt(f),
                ContractTestEvents::LogsFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `setUp`function with signature `setUp()` and selector `[10, 146, 84, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
    #[doc = "Container type for all input parameters for the `testExample`function with signature `testExample()` and selector `[63, 90, 74, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "testExample", abi = "testExample()")]
    pub struct TestExampleCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ContractTestCalls {
        IsTest(IsTestCall),
        Failed(FailedCall),
        SetUp(SetUpCall),
        TestExample(TestExampleCall),
    }
    impl ethers::core::abi::AbiDecode for ContractTestCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <IsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractTestCalls::IsTest(decoded));
            }
            if let Ok(decoded) = <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractTestCalls::Failed(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractTestCalls::SetUp(decoded));
            }
            if let Ok(decoded) =
                <TestExampleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ContractTestCalls::TestExample(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ContractTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ContractTestCalls::IsTest(element) => element.encode(),
                ContractTestCalls::Failed(element) => element.encode(),
                ContractTestCalls::SetUp(element) => element.encode(),
                ContractTestCalls::TestExample(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ContractTestCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ContractTestCalls::IsTest(element) => element.fmt(f),
                ContractTestCalls::Failed(element) => element.fmt(f),
                ContractTestCalls::SetUp(element) => element.fmt(f),
                ContractTestCalls::TestExample(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsTestCall> for ContractTestCalls {
        fn from(var: IsTestCall) -> Self {
            ContractTestCalls::IsTest(var)
        }
    }
    impl ::std::convert::From<FailedCall> for ContractTestCalls {
        fn from(var: FailedCall) -> Self {
            ContractTestCalls::Failed(var)
        }
    }
    impl ::std::convert::From<SetUpCall> for ContractTestCalls {
        fn from(var: SetUpCall) -> Self {
            ContractTestCalls::SetUp(var)
        }
    }
    impl ::std::convert::From<TestExampleCall> for ContractTestCalls {
        fn from(var: TestExampleCall) -> Self {
            ContractTestCalls::TestExample(var)
        }
    }
}
