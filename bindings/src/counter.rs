pub use counter::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod counter {
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
    #[doc = "Counter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increment\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"number\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newNumber\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setNumber\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static COUNTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static COUNTER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5060f78061001f6000396000f3fe6080604052348015600f57600080fd5b5060043610603c5760003560e01c80633fb5c1cb1460415780638381f58a146053578063d09de08a14606d575b600080fd5b6051604c3660046083565b600055565b005b605b60005481565b60405190815260200160405180910390f35b6051600080549080607c83609b565b9190505550565b600060208284031215609457600080fd5b5035919050565b60006001820160ba57634e487b7160e01b600052601160045260246000fd5b506001019056fea26469706673582212208fe2d9e04b045477f5760a998d5008e60f9cd79be7197e337e6e8084a1e3f7c664736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct Counter<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Counter<M> {
        fn clone(&self) -> Self {
            Counter(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Counter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Counter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Counter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Counter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), COUNTER_ABI.clone(), client).into()
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
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                COUNTER_ABI.clone(),
                COUNTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `increment` (0xd09de08a) function"]
        pub fn increment(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 157, 224, 138], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `number` (0x8381f58a) function"]
        pub fn number(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([131, 129, 245, 138], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setNumber` (0x3fb5c1cb) function"]
        pub fn set_number(
            &self,
            new_number: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 181, 193, 203], new_number)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Counter<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `increment` function with signature `increment()` and selector `[208, 157, 224, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "increment", abi = "increment()")]
    pub struct IncrementCall;
    #[doc = "Container type for all input parameters for the `number` function with signature `number()` and selector `[131, 129, 245, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "number", abi = "number()")]
    pub struct NumberCall;
    #[doc = "Container type for all input parameters for the `setNumber` function with signature `setNumber(uint256)` and selector `[63, 181, 193, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setNumber", abi = "setNumber(uint256)")]
    pub struct SetNumberCall {
        pub new_number: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum CounterCalls {
        Increment(IncrementCall),
        Number(NumberCall),
        SetNumber(SetNumberCall),
    }
    impl ethers::core::abi::AbiDecode for CounterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IncrementCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CounterCalls::Increment(decoded));
            }
            if let Ok(decoded) = <NumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CounterCalls::Number(decoded));
            }
            if let Ok(decoded) =
                <SetNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CounterCalls::SetNumber(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CounterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CounterCalls::Increment(element) => element.encode(),
                CounterCalls::Number(element) => element.encode(),
                CounterCalls::SetNumber(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CounterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CounterCalls::Increment(element) => element.fmt(f),
                CounterCalls::Number(element) => element.fmt(f),
                CounterCalls::SetNumber(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IncrementCall> for CounterCalls {
        fn from(var: IncrementCall) -> Self {
            CounterCalls::Increment(var)
        }
    }
    impl ::std::convert::From<NumberCall> for CounterCalls {
        fn from(var: NumberCall) -> Self {
            CounterCalls::Number(var)
        }
    }
    impl ::std::convert::From<SetNumberCall> for CounterCalls {
        fn from(var: SetNumberCall) -> Self {
            CounterCalls::SetNumber(var)
        }
    }
    #[doc = "Container type for all return fields from the `number` function with signature `number()` and selector `[131, 129, 245, 138]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NumberReturn(pub ethers::core::types::U256);
}
