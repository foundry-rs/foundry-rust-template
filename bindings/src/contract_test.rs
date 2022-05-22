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
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"val\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"val\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"val\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"logs\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_TEST\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"failed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"testExample\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static CONTRACTTEST_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040526000805460ff1916600117905534801561001d57600080fd5b506104088061002d6000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80630a9254e4146100515780633f5a4a2a14610053578063ba414fa61461005b578063fa7626d414610077575b600080fd5b005b610051610086565b610063610090565b604051901515815260200160405180910390f35b6000546100639060ff1681565b565b61008460016101bb565b60008054610100900460ff16156100b05750600054610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156101b65760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b8284015282518083038401815260608301909352600092909161013e917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001610379565b60408051601f19818403018152908290526101589161039d565b6000604051808303816000865af19150503d8060008114610195576040519150601f19603f3d011682016040523d82523d6000602084013e61019a565b606091505b50915050808060200190518101906101b291906103b0565b9150505b919050565b8061022f577f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f5060405161021f9060208082526017908201527f4572726f723a20417373657274696f6e204661696c6564000000000000000000604082015260600190565b60405180910390a161022f610232565b50565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561032d5760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b9282019290925260016060820152600091907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f19818403018152908290526102cc9291602001610379565b60408051601f19818403018152908290526102e69161039d565b6000604051808303816000865af19150503d8060008114610323576040519150601f19603f3d011682016040523d82523d6000602084013e610328565b606091505b505050505b6000805461ff001916610100179055565b6000815160005b8181101561035f5760208185018101518683015201610345565b8181111561036e576000828601525b509290920192915050565b6001600160e01b0319831681526000610395600483018461033e565b949350505050565b60006103a9828461033e565b9392505050565b6000602082840312156103c257600080fd5b815180151581146103a957600080fdfea26469706673582212200ca06c8c1dc88824cd33f5e636abdffce192e07f254c88793e25e98c240d1b9564736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ContractTest<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ContractTest<M> {
        fn clone(&self) -> Self {
            ContractTest(self.0.clone())
        }
    }
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
    impl<M: ethers::providers::Middleware> ContractTest<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CONTRACTTEST_ABI.clone(), client).into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                CONTRACTTEST_ABI.clone(),
                CONTRACTTEST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ContractTest<M> {
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
