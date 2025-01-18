///Module containing a contract's types and functions.
/**

```solidity
library ISignatureUtils {
    struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISignatureUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureWithSaltAndExpiry {
        pub signature: alloy::sol_types::private::Bytes,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureWithSaltAndExpiry> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureWithSaltAndExpiry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { signature: tuple.0, salt: tuple.1, expiry: tuple.2 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignatureWithSaltAndExpiry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SignatureWithSaltAndExpiry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SignatureWithSaltAndExpiry {
            const NAME: &'static str = "SignatureWithSaltAndExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithSaltAndExpiry(bytes signature,bytes32 salt,uint256 expiry)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiry)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignatureWithSaltAndExpiry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiry,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiry,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

    See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsInstance<T, P, N> {
        ISignatureUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtils`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ISignatureUtils`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISignatureUtilsInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

        See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self { address, provider, _network_transport: ::core::marker::PhantomData }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsInstance<T, P, N> {
            ISignatureUtilsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISignatureUtilsInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISignatureUtilsInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library ISignatureUtils {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

interface ECDSAStakeRegistry {
    struct Quorum {
        StrategyParams[] strategies;
    }
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }

    error InsufficientSignedStake();
    error InsufficientWeight();
    error InvalidLength();
    error InvalidQuorum();
    error InvalidReferenceBlock();
    error InvalidSignature();
    error InvalidSignedWeight();
    error InvalidThreshold();
    error LengthMismatch();
    error MustUpdateAllOperators();
    error NotSorted();
    error OperatorAlreadyRegistered();
    error OperatorNotRegistered();

    event Initialized(uint8 version);
    event MinimumWeightUpdated(uint256 _old, uint256 _new);
    event OperatorDeregistered(address indexed _operator, address indexed _avs);
    event OperatorRegistered(address indexed _operator, address indexed _avs);
    event OperatorWeightUpdated(address indexed _operator, uint256 oldWeight, uint256 newWeight);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event QuorumUpdated(Quorum _old, Quorum _new);
    event SigningKeyUpdate(address indexed operator, uint256 indexed updateBlock, address indexed newSigningKey, address oldSigningKey);
    event ThresholdWeightUpdated(uint256 _thresholdWeight);
    event TotalWeightUpdated(uint256 oldTotalWeight, uint256 newTotalWeight);
    event UpdateMinimumWeight(uint256 oldMinimumWeight, uint256 newMinimumWeight);

    constructor(address _delegationManager);

    function deregisterOperator() external;
    function getLastCheckpointOperatorWeight(address _operator) external view returns (uint256);
    function getLastCheckpointThresholdWeight() external view returns (uint256);
    function getLastCheckpointThresholdWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
    function getLastCheckpointTotalWeight() external view returns (uint256);
    function getLastCheckpointTotalWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
    function getLastestOperatorSigningKey(address _operator) external view returns (address);
    function getOperatorSigningKeyAtBlock(address _operator, uint256 _blockNumber) external view returns (address);
    function getOperatorWeight(address _operator) external view returns (uint256);
    function getOperatorWeightAtBlock(address _operator, uint32 _blockNumber) external view returns (uint256);
    function initialize(address _serviceManager, uint256 _thresholdWeight, Quorum memory _quorum) external;
    function isValidSignature(bytes32 _dataHash, bytes memory _signatureData) external view returns (bytes4);
    function minimumWeight() external view returns (uint256);
    function operatorRegistered(address _operator) external view returns (bool);
    function owner() external view returns (address);
    function quorum() external view returns (Quorum memory);
    function registerOperatorWithSignature(ISignatureUtils.SignatureWithSaltAndExpiry memory _operatorSignature, address _signingKey) external;
    function renounceOwnership() external;
    function transferOwnership(address newOwner) external;
    function updateMinimumWeight(uint256 _newMinimumWeight, address[] memory _operators) external;
    function updateOperatorSigningKey(address _newSigningKey) external;
    function updateOperators(address[] memory _operators) external;
    function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory) external;
    function updateQuorumConfig(Quorum memory _quorum, address[] memory _operators) external;
    function updateStakeThreshold(uint256 _thresholdWeight) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_delegationManager",
        "type": "address",
        "internalType": "contract IDelegationManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperator",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getLastCheckpointOperatorWeight",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointThresholdWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointThresholdWeightAtBlock",
    "inputs": [
      {
        "name": "_blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointTotalWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointTotalWeightAtBlock",
    "inputs": [
      {
        "name": "_blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastestOperatorSigningKey",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorSigningKeyAtBlock",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_blockNumber",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorWeight",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorWeightAtBlock",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_serviceManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_quorum",
        "type": "tuple",
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isValidSignature",
    "inputs": [
      {
        "name": "_dataHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_signatureData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "minimumWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorRegistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quorum",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registerOperatorWithSignature",
    "inputs": [
      {
        "name": "_operatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithSaltAndExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "_signingKey",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateMinimumWeight",
    "inputs": [
      {
        "name": "_newMinimumWeight",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorSigningKey",
    "inputs": [
      {
        "name": "_newSigningKey",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperators",
    "inputs": [
      {
        "name": "_operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorsForQuorum",
    "inputs": [
      {
        "name": "operatorsPerQuorum",
        "type": "address[][]",
        "internalType": "address[][]"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateQuorumConfig",
    "inputs": [
      {
        "name": "_quorum",
        "type": "tuple",
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      },
      {
        "name": "_operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateStakeThreshold",
    "inputs": [
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MinimumWeightUpdated",
    "inputs": [
      {
        "name": "_old",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "_new",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorDeregistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "_avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRegistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "_avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorWeightUpdated",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumUpdated",
    "inputs": [
      {
        "name": "_old",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      },
      {
        "name": "_new",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SigningKeyUpdate",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "updateBlock",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "newSigningKey",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldSigningKey",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ThresholdWeightUpdated",
    "inputs": [
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TotalWeightUpdated",
    "inputs": [
      {
        "name": "oldTotalWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newTotalWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdateMinimumWeight",
    "inputs": [
      {
        "name": "oldMinimumWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newMinimumWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "InsufficientSignedStake",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientWeight",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidQuorum",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidReferenceBlock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignedWeight",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MustUpdateAllOperators",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorAlreadyRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegistered",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ECDSAStakeRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a0604052348015600e575f80fd5b5060405161281e38038061281e833981016040819052602b91603b565b6001600160a01b03166080526066565b5f60208284031215604a575f80fd5b81516001600160a01b0381168114605f575f80fd5b9392505050565b6080516127a061007e5f395f6106f201526127a05ff3fe608060405234801561000f575f80fd5b506004361061016c575f3560e01c8063696255be116100d957806398ec1ac911610093578063cdcd35811161006e578063cdcd35811461030e578063dec5d1f614610321578063ec7fbb3114610334578063f2fde38b1461036f575f80fd5b806398ec1ac9146102e0578063ab118995146102f3578063b933fa7414610306575f80fd5b8063696255be14610286578063715018a614610299578063743c31f4146102a1578063857dc190146102b45780638da5cb5b146102bc578063955f2d90146102cd575f80fd5b80633b242e4a1161012a5780633b242e4a146102075780633d5611f61461021a57806340bf2fb71461022d5780635140a548146102355780635e1042e8146102485780635ef5332914610273575f80fd5b8062cf2ab5146101705780630dba3394146101855780631626ba7e146101ab5780631703a018146101d75780631e4cd85e146101ec578063314f3a49146101ff575b5f80fd5b61018361017e366004611d26565b610382565b005b610198610193366004611d70565b61038e565b6040519081526020015b60405180910390f35b6101be6101b9366004611dfe565b6103a9565b6040516001600160e01b031990911681526020016101a2565b6101df6103e5565b6040516101a29190611e9e565b6101986101fa366004611d70565b610476565b61019861048b565b610198610215366004611eb0565b61049b565b610183610228366004611ecb565b6104bb565b606754610198565b610183610243366004611f7b565b6104ca565b61025b61025636600461203e565b6104ec565b6040516001600160a01b0390911681526020016101a2565b610183610281366004612068565b610514565b61018361029436600461207f565b610525565b61018361053f565b6101836102af366004611eb0565b610552565b61018361058b565b6033546001600160a01b031661025b565b6101986102db3660046120b8565b610594565b6101986102ee366004611eb0565b6105be565b6101836103013660046121c6565b610803565b610198610919565b61025b61031c366004611eb0565b610924565b61018361032f36600461221a565b610944565b61035f610342366004611eb0565b6001600160a01b03165f908152606e602052604090205460ff1690565b60405190151581526020016101a2565b61018361037d366004611eb0565b610955565b61038b816109cb565b50565b5f6103a3606b63ffffffff80851690610a1716565b92915050565b5f805f80848060200190518101906103c1919061234f565b9250925092506103d386848484610b1f565b50630b135d3f60e11b95945050505050565b604080516020810190915260608152604080516066805460208181028401850185528301818152929391928492909184915f9085015b82821015610469575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b03168183015282526001909201910161041b565b5050505081525050905090565b5f6103a3606c63ffffffff80851690610a1716565b5f610496606b610bce565b905090565b6001600160a01b0381165f908152606d602052604081206103a390610bce565b6104c6338383610c27565b5050565b6104c6825f815181106104df576104df612423565b6020026020010151610d56565b6001600160a01b0382165f908152606a6020526040812061050d9083610a17565b9392505050565b61051c610d79565b61038b81610dd3565b61052d610d79565b61053682610e16565b6104c6816109cb565b610547610d79565b6105505f610e5c565b565b335f908152606e602052604090205460ff16610581576040516325ec6c1f60e01b815260040160405180910390fd5b61038b3382610ead565b61055033610f5d565b6001600160a01b0382165f908152606d6020526040812061050d9063ffffffff80851690610a1716565b5f8060665f01805480602002602001604051908101604052809291908181526020015f905b82821015610631575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b0316818301528252600190920191016105e3565b5050505090505f8082516001600160401b0381111561065257610652611bef565b60405190808252806020026020018201604052801561067b578160200160208202803683370190505b5090505f5b83518110156106d85783818151811061069b5761069b612423565b60200260200101515f01518282815181106106b8576106b8612423565b6001600160a01b0390921660209283029190910190910152600101610680565b50604051639004134760e01b81525f906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906390041347906107299089908690600401612437565b5f60405180830381865afa158015610743573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261076a9190810190612495565b90505f5b84518110156107d65784818151811061078957610789612423565b6020026020010151602001516001600160601b03168282815181106107b0576107b0612423565b60200260200101516107c29190612539565b6107cc9085612550565b935060010161076e565b506107e361271084612563565b925060675483106107f8575090949350505050565b505f95945050505050565b5f54610100900460ff161580801561082157505f54600160ff909116105b8061083a5750303b15801561083a57505f5460ff166001145b6108a25760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156108c3575f805461ff0019166101001790555b6108ce848484611077565b8015610913575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b5f610496606c610bce565b6001600160a01b0381165f908152606a602052604081206103a390610bce565b61094c610d79565b610536826110d7565b61095d610d79565b6001600160a01b0381166109c25760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610899565b61038b81610e5c565b5f805b8251811015610a0d576109f98382815181106109ec576109ec612423565b6020026020010151611229565b610a039083612582565b91506001016109ce565b506109138161134c565b5f438210610a675760405162461bcd60e51b815260206004820181905260248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e65646044820152606401610899565b82545f5b81811015610ac8575f610a7e82846113b6565b905084865f018281548110610a9557610a95612423565b5f9182526020909120015463ffffffff161115610ab457809250610ac2565b610abf816001612550565b91505b50610a6b565b8115610b0b5784610ada6001846125a9565b81548110610aea57610aea612423565b5f9182526020909120015464010000000090046001600160e01b0316610b0d565b5f5b6001600160e01b031695945050505050565b5f835190505f805f80610b338588516113d0565b5f5b85811015610bb857888181518110610b4f57610b4f612423565b60200260200101519450610b638588611413565b9250610b6f8486611464565b610b93838b8a8481518110610b8657610b86612423565b6020026020010151611496565b8493505f610ba186896114c7565b9050610bad8184612550565b925050600101610b35565b50610bc38187611518565b505050505050505050565b80545f908015610c155782610be46001836125a9565b81548110610bf457610bf4612423565b5f9182526020909120015464010000000090046001600160e01b0316610c17565b5f5b6001600160e01b03169392505050565b6001600160a01b0383165f908152606e602052604090205460ff1615610c60576040516342ee68b560e01b815260040160405180910390fd5b60658054905f610c6f836125bc565b90915550506001600160a01b0383165f908152606e60205260408120805460ff19166001179055610c9f84611229565b9050610caa8161134c565b5050610cb68483610ead565b606854604051639926ee7d60e01b81526001600160a01b0390911690639926ee7d90610ce890879087906004016125ff565b5f604051808303815f87803b158015610cff575f80fd5b505af1158015610d11573d5f803e3d5ffd5b50506068546040516001600160a01b03918216935090871691507fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c1905f90a350505050565b6065548151146103825760405163169efb5b60e11b815260040160405180910390fd5b6033546001600160a01b031633146105505760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610899565b610dde606c82611572565b50506040518181527f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b9060200160405180910390a150565b606780549082905560408051828152602081018490527f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f91015b60405180910390a15050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b6001600160a01b0382165f908152606a60205260408120610ecd90610bce565b9050806001600160a01b0316826001600160a01b031603610eed57505050565b6001600160a01b038381165f908152606a60205260409020610f10918416611572565b50506040516001600160a01b0382811682528084169143918616907fd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea13150029060200160405180910390a4505050565b6001600160a01b0381165f908152606e602052604090205460ff16610f95576040516325ec6c1f60e01b815260040160405180910390fd5b60658054905f610fa483612649565b90915550506001600160a01b0381165f908152606e60205260408120805460ff19169055610fd182611229565b9050610fdc8161134c565b50506068546040516351b27a6d60e11b81526001600160a01b0384811660048301529091169063a364f4da906024015f604051808303815f87803b158015611022575f80fd5b505af1158015611034573d5f803e3d5ffd5b50506068546040516001600160a01b03918216935090851691507f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed580905f90a35050565b5f54610100900460ff1661109d5760405162461bcd60e51b81526004016108999061265e565b606880546001600160a01b0319166001600160a01b0385161790556110c182610dd3565b6110ca816110d7565b6110d2611695565b505050565b6110e0816116c3565b6110fd5760405163d173577960e01b815260040160405180910390fd5b6040805160668054602081810284018501855283018181525f9484928491879085015b8282101561116e575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b031681830152825260019092019101611120565b505050915250909150606690505f6111868282611bc5565b50505f5b8251518110156111f7578251805160669190839081106111ac576111ac612423565b6020908102919091018101518254600181810185555f94855293839020825192909301516001600160601b0316600160a01b026001600160a01b03909216919091179101550161118a565b507f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e8183604051610e509291906126a9565b6001600160a01b0381165f908152606d6020526040812081908190819061124f90610bce565b6001600160a01b0386165f908152606e602052604090205490915060ff166112b45761127b81846126d6565b9250825f0361128d5750909392505050565b6001600160a01b0385165f908152606d602052604081206112ad91611572565b50506112ff565b6112bd856105be565b91506112c981836126d6565b9250825f036112db5750909392505050565b6001600160a01b0385165f908152606d602052604090206112fc9083611572565b50505b60408051828152602081018490526001600160a01b038716917f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594910160405180910390a250909392505050565b5f80611358606b610bce565b91505f6113658484612582565b9150819050611375606b82611572565b505060408051848152602081018490527f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b910160405180910390a150915091565b5f6113c46002848418612563565b61050d90848416612550565b8082146113f3576040516001621398b960e31b0319815260040160405180910390fd5b815f036104c65760405163251f56a160e21b815260040160405180910390fd5b5f438263ffffffff161061143a5760405163e64f180f60e01b815260040160405180910390fd5b6001600160a01b0383165f908152606a6020526040902061050d9063ffffffff80851690610a1716565b806001600160a01b0316826001600160a01b0316106104c65760405163ba50f91160e01b815260040160405180910390fd5b6114aa6001600160a01b0384168383611786565b6110d257604051638baa579f60e01b815260040160405180910390fd5b5f438263ffffffff16106114ee5760405163e64f180f60e01b815260040160405180910390fd5b6001600160a01b0383165f908152606d6020526040902061050d9063ffffffff80851690610a1716565b5f611522826118cb565b90508083111561154557604051634b05a0f760e11b815260040160405180910390fd5b5f61154f83611906565b9050838111156109135760405163e121632f60e01b815260040160405180910390fd5b81545f9081908161158286610bce565b90505f821180156115be5750438661159b6001856125a9565b815481106115ab576115ab612423565b5f9182526020909120015463ffffffff16145b1561161b576115cc85611941565b866115d86001856125a9565b815481106115e8576115e8612423565b905f5260205f20015f0160046101000a8154816001600160e01b0302191690836001600160e01b03160217905550611687565b855f016040518060400160405280611632436119ad565b63ffffffff16815260200161164688611941565b6001600160e01b0390811690915282546001810184555f938452602093849020835194909301519091166401000000000263ffffffff909316929092179101555b9250839150505b9250929050565b5f54610100900460ff166116bb5760405162461bcd60e51b81526004016108999061265e565b610550611a11565b80515f90818080805b8451811015611765578481815181106116e7576116e7612423565b60200260200101515f01519250826001600160a01b0316846001600160a01b0316106117265760405163ba50f91160e01b815260040160405180910390fd5b82935084818151811061173b5761173b612423565b6020026020010151602001516001600160601b03168261175b9190612550565b91506001016116cc565b50612710811461177a57505f95945050505050565b50600195945050505050565b5f805f6117938585611a40565b90925090505f8160048111156117ab576117ab6126fc565b1480156117c95750856001600160a01b0316826001600160a01b0316145b156117d95760019250505061050d565b5f80876001600160a01b0316631626ba7e60e01b8888604051602401611800929190612710565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905161183e9190612728565b5f60405180830381855afa9150503d805f8114611876576040519150601f19603f3d011682016040523d82523d5f602084013e61187b565b606091505b509150915081801561188e575080516020145b80156118bf57508051630b135d3f60e11b906118b39083016020908101908401612743565b6001600160e01b031916145b98975050505050505050565b5f438263ffffffff16106118f25760405163e64f180f60e01b815260040160405180910390fd5b6103a3606b63ffffffff80851690610a1716565b5f438263ffffffff161061192d5760405163e64f180f60e01b815260040160405180910390fd5b6103a3606c63ffffffff80851690610a1716565b5f6001600160e01b038211156119a95760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608401610899565b5090565b5f63ffffffff8211156119a95760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608401610899565b5f54610100900460ff16611a375760405162461bcd60e51b81526004016108999061265e565b61055033610e5c565b5f808251604103611a74576020830151604084015160608501515f1a611a6887828585611aa8565b9450945050505061168e565b8251604003611a9d5760208301516040840151611a92868383611b8d565b93509350505061168e565b505f9050600261168e565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0831115611add57505f90506003611b84565b8460ff16601b14158015611af557508460ff16601c14155b15611b0557505f90506004611b84565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015611b56573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b038116611b7e575f60019250925050611b84565b91505f90505b94509492505050565b5f806001600160ff1b03831681611ba960ff86901c601b612550565b9050611bb787828885611aa8565b935093505050935093915050565b5080545f8255905f5260205f209081019061038b91905b808211156119a9575f8155600101611bdc565b634e487b7160e01b5f52604160045260245ffd5b604051602081016001600160401b0381118282101715611c2557611c25611bef565b60405290565b604080519081016001600160401b0381118282101715611c2557611c25611bef565b604051601f8201601f191681016001600160401b0381118282101715611c7557611c75611bef565b604052919050565b5f6001600160401b03821115611c9557611c95611bef565b5060051b60200190565b6001600160a01b038116811461038b575f80fd5b5f82601f830112611cc2575f80fd5b8135611cd5611cd082611c7d565b611c4d565b8082825260208201915060208360051b860101925085831115611cf6575f80fd5b602085015b83811015611d1c578035611d0e81611c9f565b835260209283019201611cfb565b5095945050505050565b5f60208284031215611d36575f80fd5b81356001600160401b03811115611d4b575f80fd5b611d5784828501611cb3565b949350505050565b63ffffffff8116811461038b575f80fd5b5f60208284031215611d80575f80fd5b813561050d81611d5f565b5f6001600160401b03821115611da357611da3611bef565b50601f01601f191660200190565b5f82601f830112611dc0575f80fd5b8135611dce611cd082611d8b565b818152846020838601011115611de2575f80fd5b816020850160208301375f918101602001919091529392505050565b5f8060408385031215611e0f575f80fd5b8235915060208301356001600160401b03811115611e2b575f80fd5b611e3785828601611db1565b9150509250929050565b8051602080845281518482018190525f9290910190829060408601905b80831015611d1c57835180516001600160a01b031683526020908101516001600160601b0316818401529093019260019290920191604090910190611e5e565b602081525f61050d6020830184611e41565b5f60208284031215611ec0575f80fd5b813561050d81611c9f565b5f8060408385031215611edc575f80fd5b82356001600160401b03811115611ef1575f80fd5b830160608186031215611f02575f80fd5b604051606081016001600160401b0381118282101715611f2457611f24611bef565b60405281356001600160401b03811115611f3c575f80fd5b611f4887828501611db1565b82525060208281013581830152604092830135928201929092529250830135611f7081611c9f565b809150509250929050565b5f8060408385031215611f8c575f80fd5b82356001600160401b03811115611fa1575f80fd5b8301601f81018513611fb1575f80fd5b8035611fbf611cd082611c7d565b8082825260208201915060208360051b850101925087831115611fe0575f80fd5b602084015b838110156120205780356001600160401b03811115612002575f80fd5b6120118a602083890101611cb3565b84525060209283019201611fe5565b50945050505060208301356001600160401b03811115611e2b575f80fd5b5f806040838503121561204f575f80fd5b823561205a81611c9f565b946020939093013593505050565b5f60208284031215612078575f80fd5b5035919050565b5f8060408385031215612090575f80fd5b8235915060208301356001600160401b038111156120ac575f80fd5b611e3785828601611cb3565b5f80604083850312156120c9575f80fd5b82356120d481611c9f565b91506020830135611f7081611d5f565b5f602082840312156120f4575f80fd5b6120fc611c03565b905081356001600160401b03811115612113575f80fd5b8201601f81018413612123575f80fd5b8035612131611cd082611c7d565b8082825260208201915060208360061b850101925086831115612152575f80fd5b6020840193505b828410156121ba5760408488031215612170575f80fd5b612178611c2b565b843561218381611c9f565b815260208501356001600160601b038116811461219e575f80fd5b8060208301525080835250602082019150604084019350612159565b84525091949350505050565b5f805f606084860312156121d8575f80fd5b83356121e381611c9f565b92506020840135915060408401356001600160401b03811115612204575f80fd5b612210868287016120e4565b9150509250925092565b5f806040838503121561222b575f80fd5b82356001600160401b03811115612240575f80fd5b61224c858286016120e4565b92505060208301356001600160401b038111156120ac575f80fd5b5f5b83811015612281578181015183820152602001612269565b50505f910152565b5f82601f830112612298575f80fd5b81516122a6611cd082611c7d565b8082825260208201915060208360051b8601019250858311156122c7575f80fd5b602085015b83811015611d1c5780516001600160401b038111156122e9575f80fd5b8601603f810188136122f9575f80fd5b602081015161230a611cd082611d8b565b8181526040838301018a101561231e575f80fd5b61232f826020830160408601612267565b85525050602092830192016122cc565b805161234a81611d5f565b919050565b5f805f60608486031215612361575f80fd5b83516001600160401b03811115612376575f80fd5b8401601f81018613612386575f80fd5b8051612394611cd082611c7d565b8082825260208201915060208360051b8501019250888311156123b5575f80fd5b6020840193505b828410156123e05783516123cf81611c9f565b8252602093840193909101906123bc565b8096505050505060208401516001600160401b038111156123ff575f80fd5b61240b86828701612289565b92505061241a6040850161233f565b90509250925092565b634e487b7160e01b5f52603260045260245ffd5b6001600160a01b03831681526040602080830182905283519183018290525f91908401906060840190835b818110156124895783516001600160a01b0316835260209384019390920191600101612462565b50909695505050505050565b5f602082840312156124a5575f80fd5b81516001600160401b038111156124ba575f80fd5b8201601f810184136124ca575f80fd5b80516124d8611cd082611c7d565b8082825260208201915060208360051b8501019250868311156124f9575f80fd5b6020840193505b8284101561251b578351825260209384019390910190612500565b9695505050505050565b634e487b7160e01b5f52601160045260245ffd5b80820281158282048414176103a3576103a3612525565b808201808211156103a3576103a3612525565b5f8261257d57634e487b7160e01b5f52601260045260245ffd5b500490565b8082018281125f8312801582168215821617156125a1576125a1612525565b505092915050565b818103818111156103a3576103a3612525565b5f600182016125cd576125cd612525565b5060010190565b5f81518084526125eb816020860160208601612267565b601f01601f19169290920160200192915050565b60018060a01b0383168152604060208201525f82516060604084015261262860a08401826125d4565b90506020840151606084015260408401516080840152809150509392505050565b5f8161265757612657612525565b505f190190565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b604081525f6126bb6040830185611e41565b82810360208401526126cd8185611e41565b95945050505050565b8181035f8312801583831316838312821617156126f5576126f5612525565b5092915050565b634e487b7160e01b5f52602160045260245ffd5b828152604060208201525f611d5760408301846125d4565b5f8251612739818460208701612267565b9190910192915050565b5f60208284031215612753575f80fd5b81516001600160e01b03198116811461050d575f80fdfea2646970667358221220220a39bccd90c5a21dd46ead5b0a146a378b244f139ed46b2180a9689dc86ac364736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15`\x0EW_\x80\xFD[P`@Qa(\x1E8\x03\x80a(\x1E\x839\x81\x01`@\x81\x90R`+\x91`;V[`\x01`\x01`\xA0\x1B\x03\x16`\x80R`fV[_` \x82\x84\x03\x12\x15`JW_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`_W_\x80\xFD[\x93\x92PPPV[`\x80Qa'\xA0a\0~_9_a\x06\xF2\x01Ra'\xA0_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01lW_5`\xE0\x1C\x80cibU\xBE\x11a\0\xD9W\x80c\x98\xEC\x1A\xC9\x11a\0\x93W\x80c\xCD\xCD5\x81\x11a\0nW\x80c\xCD\xCD5\x81\x14a\x03\x0EW\x80c\xDE\xC5\xD1\xF6\x14a\x03!W\x80c\xEC\x7F\xBB1\x14a\x034W\x80c\xF2\xFD\xE3\x8B\x14a\x03oW_\x80\xFD[\x80c\x98\xEC\x1A\xC9\x14a\x02\xE0W\x80c\xAB\x11\x89\x95\x14a\x02\xF3W\x80c\xB93\xFAt\x14a\x03\x06W_\x80\xFD[\x80cibU\xBE\x14a\x02\x86W\x80cqP\x18\xA6\x14a\x02\x99W\x80ct<1\xF4\x14a\x02\xA1W\x80c\x85}\xC1\x90\x14a\x02\xB4W\x80c\x8D\xA5\xCB[\x14a\x02\xBCW\x80c\x95_-\x90\x14a\x02\xCDW_\x80\xFD[\x80c;$.J\x11a\x01*W\x80c;$.J\x14a\x02\x07W\x80c=V\x11\xF6\x14a\x02\x1AW\x80c@\xBF/\xB7\x14a\x02-W\x80cQ@\xA5H\x14a\x025W\x80c^\x10B\xE8\x14a\x02HW\x80c^\xF53)\x14a\x02sW_\x80\xFD[\x80b\xCF*\xB5\x14a\x01pW\x80c\r\xBA3\x94\x14a\x01\x85W\x80c\x16&\xBA~\x14a\x01\xABW\x80c\x17\x03\xA0\x18\x14a\x01\xD7W\x80c\x1EL\xD8^\x14a\x01\xECW\x80c1O:I\x14a\x01\xFFW[_\x80\xFD[a\x01\x83a\x01~6`\x04a\x1D&V[a\x03\x82V[\0[a\x01\x98a\x01\x936`\x04a\x1DpV[a\x03\x8EV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xBEa\x01\xB96`\x04a\x1D\xFEV[a\x03\xA9V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x01\xA2V[a\x01\xDFa\x03\xE5V[`@Qa\x01\xA2\x91\x90a\x1E\x9EV[a\x01\x98a\x01\xFA6`\x04a\x1DpV[a\x04vV[a\x01\x98a\x04\x8BV[a\x01\x98a\x02\x156`\x04a\x1E\xB0V[a\x04\x9BV[a\x01\x83a\x02(6`\x04a\x1E\xCBV[a\x04\xBBV[`gTa\x01\x98V[a\x01\x83a\x02C6`\x04a\x1F{V[a\x04\xCAV[a\x02[a\x02V6`\x04a >V[a\x04\xECV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xA2V[a\x01\x83a\x02\x816`\x04a hV[a\x05\x14V[a\x01\x83a\x02\x946`\x04a \x7FV[a\x05%V[a\x01\x83a\x05?V[a\x01\x83a\x02\xAF6`\x04a\x1E\xB0V[a\x05RV[a\x01\x83a\x05\x8BV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02[V[a\x01\x98a\x02\xDB6`\x04a \xB8V[a\x05\x94V[a\x01\x98a\x02\xEE6`\x04a\x1E\xB0V[a\x05\xBEV[a\x01\x83a\x03\x016`\x04a!\xC6V[a\x08\x03V[a\x01\x98a\t\x19V[a\x02[a\x03\x1C6`\x04a\x1E\xB0V[a\t$V[a\x01\x83a\x03/6`\x04a\"\x1AV[a\tDV[a\x03_a\x03B6`\x04a\x1E\xB0V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`n` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xA2V[a\x01\x83a\x03}6`\x04a\x1E\xB0V[a\tUV[a\x03\x8B\x81a\t\xCBV[PV[_a\x03\xA3`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[\x92\x91PPV[_\x80_\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03\xC1\x91\x90a#OV[\x92P\x92P\x92Pa\x03\xD3\x86\x84\x84\x84a\x0B\x1FV[Pc\x0B\x13]?`\xE1\x1B\x95\x94PPPPPV[`@\x80Q` \x81\x01\x90\x91R``\x81R`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R\x92\x93\x91\x92\x84\x92\x90\x91\x84\x91_\x90\x85\x01[\x82\x82\x10\x15a\x04iW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x04\x1BV[PPPP\x81RPP\x90P\x90V[_a\x03\xA3`lc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[_a\x04\x96`ka\x0B\xCEV[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`m` R`@\x81 a\x03\xA3\x90a\x0B\xCEV[a\x04\xC63\x83\x83a\x0C'V[PPV[a\x04\xC6\x82_\x81Q\x81\x10a\x04\xDFWa\x04\xDFa$#V[` \x02` \x01\x01Qa\rVV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`j` R`@\x81 a\x05\r\x90\x83a\n\x17V[\x93\x92PPPV[a\x05\x1Ca\ryV[a\x03\x8B\x81a\r\xD3V[a\x05-a\ryV[a\x056\x82a\x0E\x16V[a\x04\xC6\x81a\t\xCBV[a\x05Ga\ryV[a\x05P_a\x0E\\V[V[3_\x90\x81R`n` R`@\x90 T`\xFF\x16a\x05\x81W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x8B3\x82a\x0E\xADV[a\x05P3a\x0F]V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`m` R`@\x81 a\x05\r\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[_\x80`f_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x061W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x05\xE3V[PPPP\x90P_\x80\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06RWa\x06Ra\x1B\xEFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06{W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\xD8W\x83\x81\x81Q\x81\x10a\x06\x9BWa\x06\x9Ba$#V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a\x06\xB8Wa\x06\xB8a$#V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x06\x80V[P`@Qc\x90\x04\x13G`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x90\x04\x13G\x90a\x07)\x90\x89\x90\x86\x90`\x04\x01a$7V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07CW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07j\x91\x90\x81\x01\x90a$\x95V[\x90P_[\x84Q\x81\x10\x15a\x07\xD6W\x84\x81\x81Q\x81\x10a\x07\x89Wa\x07\x89a$#V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x07\xB0Wa\x07\xB0a$#V[` \x02` \x01\x01Qa\x07\xC2\x91\x90a%9V[a\x07\xCC\x90\x85a%PV[\x93P`\x01\x01a\x07nV[Pa\x07\xE3a'\x10\x84a%cV[\x92P`gT\x83\x10a\x07\xF8WP\x90\x94\x93PPPPV[P_\x95\x94PPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x08!WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x08:WP0;\x15\x80\x15a\x08:WP_T`\xFF\x16`\x01\x14[a\x08\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x08\xC3W_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x08\xCE\x84\x84\x84a\x10wV[\x80\x15a\t\x13W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[_a\x04\x96`la\x0B\xCEV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`j` R`@\x81 a\x03\xA3\x90a\x0B\xCEV[a\tLa\ryV[a\x056\x82a\x10\xD7V[a\t]a\ryV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\t\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\x99V[a\x03\x8B\x81a\x0E\\V[_\x80[\x82Q\x81\x10\x15a\n\rWa\t\xF9\x83\x82\x81Q\x81\x10a\t\xECWa\t\xECa$#V[` \x02` \x01\x01Qa\x12)V[a\n\x03\x90\x83a%\x82V[\x91P`\x01\x01a\t\xCEV[Pa\t\x13\x81a\x13LV[_C\x82\x10a\ngW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R`d\x01a\x08\x99V[\x82T_[\x81\x81\x10\x15a\n\xC8W_a\n~\x82\x84a\x13\xB6V[\x90P\x84\x86_\x01\x82\x81T\x81\x10a\n\x95Wa\n\x95a$#V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\n\xB4W\x80\x92Pa\n\xC2V[a\n\xBF\x81`\x01a%PV[\x91P[Pa\nkV[\x81\x15a\x0B\x0BW\x84a\n\xDA`\x01\x84a%\xA9V[\x81T\x81\x10a\n\xEAWa\n\xEAa$#V[_\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0B\rV[_[`\x01`\x01`\xE0\x1B\x03\x16\x95\x94PPPPPV[_\x83Q\x90P_\x80_\x80a\x0B3\x85\x88Qa\x13\xD0V[_[\x85\x81\x10\x15a\x0B\xB8W\x88\x81\x81Q\x81\x10a\x0BOWa\x0BOa$#V[` \x02` \x01\x01Q\x94Pa\x0Bc\x85\x88a\x14\x13V[\x92Pa\x0Bo\x84\x86a\x14dV[a\x0B\x93\x83\x8B\x8A\x84\x81Q\x81\x10a\x0B\x86Wa\x0B\x86a$#V[` \x02` \x01\x01Qa\x14\x96V[\x84\x93P_a\x0B\xA1\x86\x89a\x14\xC7V[\x90Pa\x0B\xAD\x81\x84a%PV[\x92PP`\x01\x01a\x0B5V[Pa\x0B\xC3\x81\x87a\x15\x18V[PPPPPPPPPV[\x80T_\x90\x80\x15a\x0C\x15W\x82a\x0B\xE4`\x01\x83a%\xA9V[\x81T\x81\x10a\x0B\xF4Wa\x0B\xF4a$#V[_\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0C\x17V[_[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`n` R`@\x90 T`\xFF\x16\x15a\x0C`W`@QcB\xEEh\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90_a\x0Co\x83a%\xBCV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`n` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0C\x9F\x84a\x12)V[\x90Pa\x0C\xAA\x81a\x13LV[PPa\x0C\xB6\x84\x83a\x0E\xADV[`hT`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x99&\xEE}\x90a\x0C\xE8\x90\x87\x90\x87\x90`\x04\x01a%\xFFV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xFFW_\x80\xFD[PZ\xF1\x15\x80\x15a\r\x11W=_\x80>=_\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x87\x16\x91P\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90_\x90\xA3PPPPV[`eT\x81Q\x14a\x03\x82W`@Qc\x16\x9E\xFB[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08\x99V[a\r\xDE`l\x82a\x15rV[PP`@Q\x81\x81R\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`g\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8F\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`j` R`@\x81 a\x0E\xCD\x90a\x0B\xCEV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0E\xEDWPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`j` R`@\x90 a\x0F\x10\x91\x84\x16a\x15rV[PP`@Q`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82R\x80\x84\x16\x91C\x91\x86\x16\x90\x7F\xD0a\x16\x82R\xF4As6X\xF0\x9EM\x8F[-\x99\x8E\xD4\xEF$\xA2\xBB\xFDl\xEC\xA5.\xA11P\x02\x90` \x01`@Q\x80\x91\x03\x90\xA4PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`n` R`@\x90 T`\xFF\x16a\x0F\x95W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90_a\x0F\xA4\x83a&IV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`n` R`@\x81 \x80T`\xFF\x19\x16\x90Ua\x0F\xD1\x82a\x12)V[\x90Pa\x0F\xDC\x81a\x13LV[PP`hT`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xA3d\xF4\xDA\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\"W_\x80\xFD[PZ\xF1\x15\x80\x15a\x104W=_\x80>=_\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x85\x16\x91P\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80\x90_\x90\xA3PPV[_Ta\x01\0\x90\x04`\xFF\x16a\x10\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x99\x90a&^V[`h\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua\x10\xC1\x82a\r\xD3V[a\x10\xCA\x81a\x10\xD7V[a\x10\xD2a\x16\x95V[PPPV[a\x10\xE0\x81a\x16\xC3V[a\x10\xFDW`@Qc\xD1sWy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R_\x94\x84\x92\x84\x91\x87\x90\x85\x01[\x82\x82\x10\x15a\x11nW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x11 V[PPP\x91RP\x90\x91P`f\x90P_a\x11\x86\x82\x82a\x1B\xC5V[PP_[\x82QQ\x81\x10\x15a\x11\xF7W\x82Q\x80Q`f\x91\x90\x83\x90\x81\x10a\x11\xACWa\x11\xACa$#V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x81\x01\x85U_\x94\x85R\x93\x83\x90 \x82Q\x92\x90\x93\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x91\x01U\x01a\x11\x8AV[P\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x81\x83`@Qa\x0EP\x92\x91\x90a&\xA9V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`m` R`@\x81 \x81\x90\x81\x90\x81\x90a\x12O\x90a\x0B\xCEV[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`n` R`@\x90 T\x90\x91P`\xFF\x16a\x12\xB4Wa\x12{\x81\x84a&\xD6V[\x92P\x82_\x03a\x12\x8DWP\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`m` R`@\x81 a\x12\xAD\x91a\x15rV[PPa\x12\xFFV[a\x12\xBD\x85a\x05\xBEV[\x91Pa\x12\xC9\x81\x83a&\xD6V[\x92P\x82_\x03a\x12\xDBWP\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`m` R`@\x90 a\x12\xFC\x90\x83a\x15rV[PP[`@\x80Q\x82\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16\x91\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x91\x01`@Q\x80\x91\x03\x90\xA2P\x90\x93\x92PPPV[_\x80a\x13X`ka\x0B\xCEV[\x91P_a\x13e\x84\x84a%\x82V[\x91P\x81\x90Pa\x13u`k\x82a\x15rV[PP`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B\x91\x01`@Q\x80\x91\x03\x90\xA1P\x91P\x91V[_a\x13\xC4`\x02\x84\x84\x18a%cV[a\x05\r\x90\x84\x84\x16a%PV[\x80\x82\x14a\x13\xF3W`@Q`\x01b\x13\x98\xB9`\xE3\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81_\x03a\x04\xC6W`@Qc%\x1FV\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x14:W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`j` R`@\x90 a\x05\r\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x10a\x04\xC6W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xAA`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a\x17\x86V[a\x10\xD2W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x14\xEEW`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`m` R`@\x90 a\x05\r\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[_a\x15\"\x82a\x18\xCBV[\x90P\x80\x83\x11\x15a\x15EW`@QcK\x05\xA0\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x15O\x83a\x19\x06V[\x90P\x83\x81\x11\x15a\t\x13W`@Qc\xE1!c/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T_\x90\x81\x90\x81a\x15\x82\x86a\x0B\xCEV[\x90P_\x82\x11\x80\x15a\x15\xBEWPC\x86a\x15\x9B`\x01\x85a%\xA9V[\x81T\x81\x10a\x15\xABWa\x15\xABa$#V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x16\x1BWa\x15\xCC\x85a\x19AV[\x86a\x15\xD8`\x01\x85a%\xA9V[\x81T\x81\x10a\x15\xE8Wa\x15\xE8a$#V[\x90_R` _ \x01_\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xE0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xE0\x1B\x03\x16\x02\x17\x90UPa\x16\x87V[\x85_\x01`@Q\x80`@\x01`@R\x80a\x162Ca\x19\xADV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x16F\x88a\x19AV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U_\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[\x92P\x83\x91PP[\x92P\x92\x90PV[_Ta\x01\0\x90\x04`\xFF\x16a\x16\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x99\x90a&^V[a\x05Pa\x1A\x11V[\x80Q_\x90\x81\x80\x80\x80[\x84Q\x81\x10\x15a\x17eW\x84\x81\x81Q\x81\x10a\x16\xE7Wa\x16\xE7a$#V[` \x02` \x01\x01Q_\x01Q\x92P\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x17&W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x93P\x84\x81\x81Q\x81\x10a\x17;Wa\x17;a$#V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82a\x17[\x91\x90a%PV[\x91P`\x01\x01a\x16\xCCV[Pa'\x10\x81\x14a\x17zWP_\x95\x94PPPPPV[P`\x01\x95\x94PPPPPV[_\x80_a\x17\x93\x85\x85a\x1A@V[\x90\x92P\x90P_\x81`\x04\x81\x11\x15a\x17\xABWa\x17\xABa&\xFCV[\x14\x80\x15a\x17\xC9WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x17\xD9W`\x01\x92PPPa\x05\rV[_\x80\x87`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01a\x18\0\x92\x91\x90a'\x10V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x18>\x91\x90a'(V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a\x18vW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x18{V[``\x91P[P\x91P\x91P\x81\x80\x15a\x18\x8EWP\x80Q` \x14[\x80\x15a\x18\xBFWP\x80Qc\x0B\x13]?`\xE1\x1B\x90a\x18\xB3\x90\x83\x01` \x90\x81\x01\x90\x84\x01a'CV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14[\x98\x97PPPPPPPPV[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x18\xF2W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xA3`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x19-W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xA3`lc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[_`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x19\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x08\x99V[P\x90V[_c\xFF\xFF\xFF\xFF\x82\x11\x15a\x19\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\x99V[_Ta\x01\0\x90\x04`\xFF\x16a\x1A7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x99\x90a&^V[a\x05P3a\x0E\\V[_\x80\x82Q`A\x03a\x1AtW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa\x1Ah\x87\x82\x85\x85a\x1A\xA8V[\x94P\x94PPPPa\x16\x8EV[\x82Q`@\x03a\x1A\x9DW` \x83\x01Q`@\x84\x01Qa\x1A\x92\x86\x83\x83a\x1B\x8DV[\x93P\x93PPPa\x16\x8EV[P_\x90P`\x02a\x16\x8EV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1A\xDDWP_\x90P`\x03a\x1B\x84V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a\x1A\xF5WP\x84`\xFF\x16`\x1C\x14\x15[\x15a\x1B\x05WP_\x90P`\x04a\x1B\x84V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1BVW=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1B~W_`\x01\x92P\x92PPa\x1B\x84V[\x91P_\x90P[\x94P\x94\x92PPPV[_\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a\x1B\xA9`\xFF\x86\x90\x1C`\x1Ba%PV[\x90Pa\x1B\xB7\x87\x82\x88\x85a\x1A\xA8V[\x93P\x93PPP\x93P\x93\x91PPV[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a\x03\x8B\x91\x90[\x80\x82\x11\x15a\x19\xA9W_\x81U`\x01\x01a\x1B\xDCV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C%Wa\x1C%a\x1B\xEFV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C%Wa\x1C%a\x1B\xEFV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1CuWa\x1Cua\x1B\xEFV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1C\x95Wa\x1C\x95a\x1B\xEFV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x8BW_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x1C\xC2W_\x80\xFD[\x815a\x1C\xD5a\x1C\xD0\x82a\x1C}V[a\x1CMV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x1C\xF6W_\x80\xFD[` \x85\x01[\x83\x81\x10\x15a\x1D\x1CW\x805a\x1D\x0E\x81a\x1C\x9FV[\x83R` \x92\x83\x01\x92\x01a\x1C\xFBV[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x1D6W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DKW_\x80\xFD[a\x1DW\x84\x82\x85\x01a\x1C\xB3V[\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\x8BW_\x80\xFD[_` \x82\x84\x03\x12\x15a\x1D\x80W_\x80\xFD[\x815a\x05\r\x81a\x1D_V[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1D\xA3Wa\x1D\xA3a\x1B\xEFV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x1D\xC0W_\x80\xFD[\x815a\x1D\xCEa\x1C\xD0\x82a\x1D\x8BV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1D\xE2W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x1E\x0FW_\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E+W_\x80\xFD[a\x1E7\x85\x82\x86\x01a\x1D\xB1V[\x91PP\x92P\x92\x90PV[\x80Q` \x80\x84R\x81Q\x84\x82\x01\x81\x90R_\x92\x90\x91\x01\x90\x82\x90`@\x86\x01\x90[\x80\x83\x10\x15a\x1D\x1CW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x84\x01R\x90\x93\x01\x92`\x01\x92\x90\x92\x01\x91`@\x90\x91\x01\x90a\x1E^V[` \x81R_a\x05\r` \x83\x01\x84a\x1EAV[_` \x82\x84\x03\x12\x15a\x1E\xC0W_\x80\xFD[\x815a\x05\r\x81a\x1C\x9FV[_\x80`@\x83\x85\x03\x12\x15a\x1E\xDCW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xF1W_\x80\xFD[\x83\x01``\x81\x86\x03\x12\x15a\x1F\x02W_\x80\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1F$Wa\x1F$a\x1B\xEFV[`@R\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F<W_\x80\xFD[a\x1FH\x87\x82\x85\x01a\x1D\xB1V[\x82RP` \x82\x81\x015\x81\x83\x01R`@\x92\x83\x015\x92\x82\x01\x92\x90\x92R\x92P\x83\x015a\x1Fp\x81a\x1C\x9FV[\x80\x91PP\x92P\x92\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x1F\x8CW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xA1W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x1F\xB1W_\x80\xFD[\x805a\x1F\xBFa\x1C\xD0\x82a\x1C}V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a\x1F\xE0W_\x80\xFD[` \x84\x01[\x83\x81\x10\x15a  W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a \x02W_\x80\xFD[a \x11\x8A` \x83\x89\x01\x01a\x1C\xB3V[\x84RP` \x92\x83\x01\x92\x01a\x1F\xE5V[P\x94PPPP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E+W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a OW_\x80\xFD[\x825a Z\x81a\x1C\x9FV[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a xW_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a \x90W_\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \xACW_\x80\xFD[a\x1E7\x85\x82\x86\x01a\x1C\xB3V[_\x80`@\x83\x85\x03\x12\x15a \xC9W_\x80\xFD[\x825a \xD4\x81a\x1C\x9FV[\x91P` \x83\x015a\x1Fp\x81a\x1D_V[_` \x82\x84\x03\x12\x15a \xF4W_\x80\xFD[a \xFCa\x1C\x03V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x13W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a!#W_\x80\xFD[\x805a!1a\x1C\xD0\x82a\x1C}V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a!RW_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a!\xBAW`@\x84\x88\x03\x12\x15a!pW_\x80\xFD[a!xa\x1C+V[\x845a!\x83\x81a\x1C\x9FV[\x81R` \x85\x015`\x01`\x01``\x1B\x03\x81\x16\x81\x14a!\x9EW_\x80\xFD[\x80` \x83\x01RP\x80\x83RP` \x82\x01\x91P`@\x84\x01\x93Pa!YV[\x84RP\x91\x94\x93PPPPV[_\x80_``\x84\x86\x03\x12\x15a!\xD8W_\x80\xFD[\x835a!\xE3\x81a\x1C\x9FV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x04W_\x80\xFD[a\"\x10\x86\x82\x87\x01a \xE4V[\x91PP\x92P\x92P\x92V[_\x80`@\x83\x85\x03\x12\x15a\"+W_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\"@W_\x80\xFD[a\"L\x85\x82\x86\x01a \xE4V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \xACW_\x80\xFD[_[\x83\x81\x10\x15a\"\x81W\x81\x81\x01Q\x83\x82\x01R` \x01a\"iV[PP_\x91\x01RV[_\x82`\x1F\x83\x01\x12a\"\x98W_\x80\xFD[\x81Qa\"\xA6a\x1C\xD0\x82a\x1C}V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\"\xC7W_\x80\xFD[` \x85\x01[\x83\x81\x10\x15a\x1D\x1CW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xE9W_\x80\xFD[\x86\x01`?\x81\x01\x88\x13a\"\xF9W_\x80\xFD[` \x81\x01Qa#\na\x1C\xD0\x82a\x1D\x8BV[\x81\x81R`@\x83\x83\x01\x01\x8A\x10\x15a#\x1EW_\x80\xFD[a#/\x82` \x83\x01`@\x86\x01a\"gV[\x85RPP` \x92\x83\x01\x92\x01a\"\xCCV[\x80Qa#J\x81a\x1D_V[\x91\x90PV[_\x80_``\x84\x86\x03\x12\x15a#aW_\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#vW_\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a#\x86W_\x80\xFD[\x80Qa#\x94a\x1C\xD0\x82a\x1C}V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a#\xB5W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a#\xE0W\x83Qa#\xCF\x81a\x1C\x9FV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a#\xBCV[\x80\x96PPPPP` \x84\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xFFW_\x80\xFD[a$\x0B\x86\x82\x87\x01a\"\x89V[\x92PPa$\x1A`@\x85\x01a#?V[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x80\x83\x01\x82\x90R\x83Q\x91\x83\x01\x82\x90R_\x91\x90\x84\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a$\x89W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a$bV[P\x90\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a$\xA5W_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xBAW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a$\xCAW_\x80\xFD[\x80Qa$\xD8a\x1C\xD0\x82a\x1C}V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a$\xF9W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a%\x1BW\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a%\0V[\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xA3Wa\x03\xA3a%%V[\x80\x82\x01\x80\x82\x11\x15a\x03\xA3Wa\x03\xA3a%%V[_\x82a%}WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a%\xA1Wa%\xA1a%%V[PP\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x03\xA3Wa\x03\xA3a%%V[_`\x01\x82\x01a%\xCDWa%\xCDa%%V[P`\x01\x01\x90V[_\x81Q\x80\x84Ra%\xEB\x81` \x86\x01` \x86\x01a\"gV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q```@\x84\x01Ra&(`\xA0\x84\x01\x82a%\xD4V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[_\x81a&WWa&Wa%%V[P_\x19\x01\x90V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_a&\xBB`@\x83\x01\x85a\x1EAV[\x82\x81\x03` \x84\x01Ra&\xCD\x81\x85a\x1EAV[\x95\x94PPPPPV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&\xF5Wa&\xF5a%%V[P\x92\x91PPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82\x81R`@` \x82\x01R_a\x1DW`@\x83\x01\x84a%\xD4V[_\x82Qa'9\x81\x84` \x87\x01a\"gV[\x91\x90\x91\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15a'SW_\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x05\rW_\x80\xFD\xFE\xA2dipfsX\"\x12 \"\n9\xBC\xCD\x90\xC5\xA2\x1D\xD4n\xAD[\n\x14j7\x8B$O\x13\x9E\xD4k!\x80\xA9h\x9D\xC8j\xC3dsolcC\0\x08\x1A\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b506004361061016c575f3560e01c8063696255be116100d957806398ec1ac911610093578063cdcd35811161006e578063cdcd35811461030e578063dec5d1f614610321578063ec7fbb3114610334578063f2fde38b1461036f575f80fd5b806398ec1ac9146102e0578063ab118995146102f3578063b933fa7414610306575f80fd5b8063696255be14610286578063715018a614610299578063743c31f4146102a1578063857dc190146102b45780638da5cb5b146102bc578063955f2d90146102cd575f80fd5b80633b242e4a1161012a5780633b242e4a146102075780633d5611f61461021a57806340bf2fb71461022d5780635140a548146102355780635e1042e8146102485780635ef5332914610273575f80fd5b8062cf2ab5146101705780630dba3394146101855780631626ba7e146101ab5780631703a018146101d75780631e4cd85e146101ec578063314f3a49146101ff575b5f80fd5b61018361017e366004611d26565b610382565b005b610198610193366004611d70565b61038e565b6040519081526020015b60405180910390f35b6101be6101b9366004611dfe565b6103a9565b6040516001600160e01b031990911681526020016101a2565b6101df6103e5565b6040516101a29190611e9e565b6101986101fa366004611d70565b610476565b61019861048b565b610198610215366004611eb0565b61049b565b610183610228366004611ecb565b6104bb565b606754610198565b610183610243366004611f7b565b6104ca565b61025b61025636600461203e565b6104ec565b6040516001600160a01b0390911681526020016101a2565b610183610281366004612068565b610514565b61018361029436600461207f565b610525565b61018361053f565b6101836102af366004611eb0565b610552565b61018361058b565b6033546001600160a01b031661025b565b6101986102db3660046120b8565b610594565b6101986102ee366004611eb0565b6105be565b6101836103013660046121c6565b610803565b610198610919565b61025b61031c366004611eb0565b610924565b61018361032f36600461221a565b610944565b61035f610342366004611eb0565b6001600160a01b03165f908152606e602052604090205460ff1690565b60405190151581526020016101a2565b61018361037d366004611eb0565b610955565b61038b816109cb565b50565b5f6103a3606b63ffffffff80851690610a1716565b92915050565b5f805f80848060200190518101906103c1919061234f565b9250925092506103d386848484610b1f565b50630b135d3f60e11b95945050505050565b604080516020810190915260608152604080516066805460208181028401850185528301818152929391928492909184915f9085015b82821015610469575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b03168183015282526001909201910161041b565b5050505081525050905090565b5f6103a3606c63ffffffff80851690610a1716565b5f610496606b610bce565b905090565b6001600160a01b0381165f908152606d602052604081206103a390610bce565b6104c6338383610c27565b5050565b6104c6825f815181106104df576104df612423565b6020026020010151610d56565b6001600160a01b0382165f908152606a6020526040812061050d9083610a17565b9392505050565b61051c610d79565b61038b81610dd3565b61052d610d79565b61053682610e16565b6104c6816109cb565b610547610d79565b6105505f610e5c565b565b335f908152606e602052604090205460ff16610581576040516325ec6c1f60e01b815260040160405180910390fd5b61038b3382610ead565b61055033610f5d565b6001600160a01b0382165f908152606d6020526040812061050d9063ffffffff80851690610a1716565b5f8060665f01805480602002602001604051908101604052809291908181526020015f905b82821015610631575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b0316818301528252600190920191016105e3565b5050505090505f8082516001600160401b0381111561065257610652611bef565b60405190808252806020026020018201604052801561067b578160200160208202803683370190505b5090505f5b83518110156106d85783818151811061069b5761069b612423565b60200260200101515f01518282815181106106b8576106b8612423565b6001600160a01b0390921660209283029190910190910152600101610680565b50604051639004134760e01b81525f906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906390041347906107299089908690600401612437565b5f60405180830381865afa158015610743573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261076a9190810190612495565b90505f5b84518110156107d65784818151811061078957610789612423565b6020026020010151602001516001600160601b03168282815181106107b0576107b0612423565b60200260200101516107c29190612539565b6107cc9085612550565b935060010161076e565b506107e361271084612563565b925060675483106107f8575090949350505050565b505f95945050505050565b5f54610100900460ff161580801561082157505f54600160ff909116105b8061083a5750303b15801561083a57505f5460ff166001145b6108a25760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156108c3575f805461ff0019166101001790555b6108ce848484611077565b8015610913575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b5f610496606c610bce565b6001600160a01b0381165f908152606a602052604081206103a390610bce565b61094c610d79565b610536826110d7565b61095d610d79565b6001600160a01b0381166109c25760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610899565b61038b81610e5c565b5f805b8251811015610a0d576109f98382815181106109ec576109ec612423565b6020026020010151611229565b610a039083612582565b91506001016109ce565b506109138161134c565b5f438210610a675760405162461bcd60e51b815260206004820181905260248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e65646044820152606401610899565b82545f5b81811015610ac8575f610a7e82846113b6565b905084865f018281548110610a9557610a95612423565b5f9182526020909120015463ffffffff161115610ab457809250610ac2565b610abf816001612550565b91505b50610a6b565b8115610b0b5784610ada6001846125a9565b81548110610aea57610aea612423565b5f9182526020909120015464010000000090046001600160e01b0316610b0d565b5f5b6001600160e01b031695945050505050565b5f835190505f805f80610b338588516113d0565b5f5b85811015610bb857888181518110610b4f57610b4f612423565b60200260200101519450610b638588611413565b9250610b6f8486611464565b610b93838b8a8481518110610b8657610b86612423565b6020026020010151611496565b8493505f610ba186896114c7565b9050610bad8184612550565b925050600101610b35565b50610bc38187611518565b505050505050505050565b80545f908015610c155782610be46001836125a9565b81548110610bf457610bf4612423565b5f9182526020909120015464010000000090046001600160e01b0316610c17565b5f5b6001600160e01b03169392505050565b6001600160a01b0383165f908152606e602052604090205460ff1615610c60576040516342ee68b560e01b815260040160405180910390fd5b60658054905f610c6f836125bc565b90915550506001600160a01b0383165f908152606e60205260408120805460ff19166001179055610c9f84611229565b9050610caa8161134c565b5050610cb68483610ead565b606854604051639926ee7d60e01b81526001600160a01b0390911690639926ee7d90610ce890879087906004016125ff565b5f604051808303815f87803b158015610cff575f80fd5b505af1158015610d11573d5f803e3d5ffd5b50506068546040516001600160a01b03918216935090871691507fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c1905f90a350505050565b6065548151146103825760405163169efb5b60e11b815260040160405180910390fd5b6033546001600160a01b031633146105505760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610899565b610dde606c82611572565b50506040518181527f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b9060200160405180910390a150565b606780549082905560408051828152602081018490527f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f91015b60405180910390a15050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b6001600160a01b0382165f908152606a60205260408120610ecd90610bce565b9050806001600160a01b0316826001600160a01b031603610eed57505050565b6001600160a01b038381165f908152606a60205260409020610f10918416611572565b50506040516001600160a01b0382811682528084169143918616907fd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea13150029060200160405180910390a4505050565b6001600160a01b0381165f908152606e602052604090205460ff16610f95576040516325ec6c1f60e01b815260040160405180910390fd5b60658054905f610fa483612649565b90915550506001600160a01b0381165f908152606e60205260408120805460ff19169055610fd182611229565b9050610fdc8161134c565b50506068546040516351b27a6d60e11b81526001600160a01b0384811660048301529091169063a364f4da906024015f604051808303815f87803b158015611022575f80fd5b505af1158015611034573d5f803e3d5ffd5b50506068546040516001600160a01b03918216935090851691507f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed580905f90a35050565b5f54610100900460ff1661109d5760405162461bcd60e51b81526004016108999061265e565b606880546001600160a01b0319166001600160a01b0385161790556110c182610dd3565b6110ca816110d7565b6110d2611695565b505050565b6110e0816116c3565b6110fd5760405163d173577960e01b815260040160405180910390fd5b6040805160668054602081810284018501855283018181525f9484928491879085015b8282101561116e575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b031681830152825260019092019101611120565b505050915250909150606690505f6111868282611bc5565b50505f5b8251518110156111f7578251805160669190839081106111ac576111ac612423565b6020908102919091018101518254600181810185555f94855293839020825192909301516001600160601b0316600160a01b026001600160a01b03909216919091179101550161118a565b507f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e8183604051610e509291906126a9565b6001600160a01b0381165f908152606d6020526040812081908190819061124f90610bce565b6001600160a01b0386165f908152606e602052604090205490915060ff166112b45761127b81846126d6565b9250825f0361128d5750909392505050565b6001600160a01b0385165f908152606d602052604081206112ad91611572565b50506112ff565b6112bd856105be565b91506112c981836126d6565b9250825f036112db5750909392505050565b6001600160a01b0385165f908152606d602052604090206112fc9083611572565b50505b60408051828152602081018490526001600160a01b038716917f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594910160405180910390a250909392505050565b5f80611358606b610bce565b91505f6113658484612582565b9150819050611375606b82611572565b505060408051848152602081018490527f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b910160405180910390a150915091565b5f6113c46002848418612563565b61050d90848416612550565b8082146113f3576040516001621398b960e31b0319815260040160405180910390fd5b815f036104c65760405163251f56a160e21b815260040160405180910390fd5b5f438263ffffffff161061143a5760405163e64f180f60e01b815260040160405180910390fd5b6001600160a01b0383165f908152606a6020526040902061050d9063ffffffff80851690610a1716565b806001600160a01b0316826001600160a01b0316106104c65760405163ba50f91160e01b815260040160405180910390fd5b6114aa6001600160a01b0384168383611786565b6110d257604051638baa579f60e01b815260040160405180910390fd5b5f438263ffffffff16106114ee5760405163e64f180f60e01b815260040160405180910390fd5b6001600160a01b0383165f908152606d6020526040902061050d9063ffffffff80851690610a1716565b5f611522826118cb565b90508083111561154557604051634b05a0f760e11b815260040160405180910390fd5b5f61154f83611906565b9050838111156109135760405163e121632f60e01b815260040160405180910390fd5b81545f9081908161158286610bce565b90505f821180156115be5750438661159b6001856125a9565b815481106115ab576115ab612423565b5f9182526020909120015463ffffffff16145b1561161b576115cc85611941565b866115d86001856125a9565b815481106115e8576115e8612423565b905f5260205f20015f0160046101000a8154816001600160e01b0302191690836001600160e01b03160217905550611687565b855f016040518060400160405280611632436119ad565b63ffffffff16815260200161164688611941565b6001600160e01b0390811690915282546001810184555f938452602093849020835194909301519091166401000000000263ffffffff909316929092179101555b9250839150505b9250929050565b5f54610100900460ff166116bb5760405162461bcd60e51b81526004016108999061265e565b610550611a11565b80515f90818080805b8451811015611765578481815181106116e7576116e7612423565b60200260200101515f01519250826001600160a01b0316846001600160a01b0316106117265760405163ba50f91160e01b815260040160405180910390fd5b82935084818151811061173b5761173b612423565b6020026020010151602001516001600160601b03168261175b9190612550565b91506001016116cc565b50612710811461177a57505f95945050505050565b50600195945050505050565b5f805f6117938585611a40565b90925090505f8160048111156117ab576117ab6126fc565b1480156117c95750856001600160a01b0316826001600160a01b0316145b156117d95760019250505061050d565b5f80876001600160a01b0316631626ba7e60e01b8888604051602401611800929190612710565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905161183e9190612728565b5f60405180830381855afa9150503d805f8114611876576040519150601f19603f3d011682016040523d82523d5f602084013e61187b565b606091505b509150915081801561188e575080516020145b80156118bf57508051630b135d3f60e11b906118b39083016020908101908401612743565b6001600160e01b031916145b98975050505050505050565b5f438263ffffffff16106118f25760405163e64f180f60e01b815260040160405180910390fd5b6103a3606b63ffffffff80851690610a1716565b5f438263ffffffff161061192d5760405163e64f180f60e01b815260040160405180910390fd5b6103a3606c63ffffffff80851690610a1716565b5f6001600160e01b038211156119a95760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608401610899565b5090565b5f63ffffffff8211156119a95760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608401610899565b5f54610100900460ff16611a375760405162461bcd60e51b81526004016108999061265e565b61055033610e5c565b5f808251604103611a74576020830151604084015160608501515f1a611a6887828585611aa8565b9450945050505061168e565b8251604003611a9d5760208301516040840151611a92868383611b8d565b93509350505061168e565b505f9050600261168e565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0831115611add57505f90506003611b84565b8460ff16601b14158015611af557508460ff16601c14155b15611b0557505f90506004611b84565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015611b56573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b038116611b7e575f60019250925050611b84565b91505f90505b94509492505050565b5f806001600160ff1b03831681611ba960ff86901c601b612550565b9050611bb787828885611aa8565b935093505050935093915050565b5080545f8255905f5260205f209081019061038b91905b808211156119a9575f8155600101611bdc565b634e487b7160e01b5f52604160045260245ffd5b604051602081016001600160401b0381118282101715611c2557611c25611bef565b60405290565b604080519081016001600160401b0381118282101715611c2557611c25611bef565b604051601f8201601f191681016001600160401b0381118282101715611c7557611c75611bef565b604052919050565b5f6001600160401b03821115611c9557611c95611bef565b5060051b60200190565b6001600160a01b038116811461038b575f80fd5b5f82601f830112611cc2575f80fd5b8135611cd5611cd082611c7d565b611c4d565b8082825260208201915060208360051b860101925085831115611cf6575f80fd5b602085015b83811015611d1c578035611d0e81611c9f565b835260209283019201611cfb565b5095945050505050565b5f60208284031215611d36575f80fd5b81356001600160401b03811115611d4b575f80fd5b611d5784828501611cb3565b949350505050565b63ffffffff8116811461038b575f80fd5b5f60208284031215611d80575f80fd5b813561050d81611d5f565b5f6001600160401b03821115611da357611da3611bef565b50601f01601f191660200190565b5f82601f830112611dc0575f80fd5b8135611dce611cd082611d8b565b818152846020838601011115611de2575f80fd5b816020850160208301375f918101602001919091529392505050565b5f8060408385031215611e0f575f80fd5b8235915060208301356001600160401b03811115611e2b575f80fd5b611e3785828601611db1565b9150509250929050565b8051602080845281518482018190525f9290910190829060408601905b80831015611d1c57835180516001600160a01b031683526020908101516001600160601b0316818401529093019260019290920191604090910190611e5e565b602081525f61050d6020830184611e41565b5f60208284031215611ec0575f80fd5b813561050d81611c9f565b5f8060408385031215611edc575f80fd5b82356001600160401b03811115611ef1575f80fd5b830160608186031215611f02575f80fd5b604051606081016001600160401b0381118282101715611f2457611f24611bef565b60405281356001600160401b03811115611f3c575f80fd5b611f4887828501611db1565b82525060208281013581830152604092830135928201929092529250830135611f7081611c9f565b809150509250929050565b5f8060408385031215611f8c575f80fd5b82356001600160401b03811115611fa1575f80fd5b8301601f81018513611fb1575f80fd5b8035611fbf611cd082611c7d565b8082825260208201915060208360051b850101925087831115611fe0575f80fd5b602084015b838110156120205780356001600160401b03811115612002575f80fd5b6120118a602083890101611cb3565b84525060209283019201611fe5565b50945050505060208301356001600160401b03811115611e2b575f80fd5b5f806040838503121561204f575f80fd5b823561205a81611c9f565b946020939093013593505050565b5f60208284031215612078575f80fd5b5035919050565b5f8060408385031215612090575f80fd5b8235915060208301356001600160401b038111156120ac575f80fd5b611e3785828601611cb3565b5f80604083850312156120c9575f80fd5b82356120d481611c9f565b91506020830135611f7081611d5f565b5f602082840312156120f4575f80fd5b6120fc611c03565b905081356001600160401b03811115612113575f80fd5b8201601f81018413612123575f80fd5b8035612131611cd082611c7d565b8082825260208201915060208360061b850101925086831115612152575f80fd5b6020840193505b828410156121ba5760408488031215612170575f80fd5b612178611c2b565b843561218381611c9f565b815260208501356001600160601b038116811461219e575f80fd5b8060208301525080835250602082019150604084019350612159565b84525091949350505050565b5f805f606084860312156121d8575f80fd5b83356121e381611c9f565b92506020840135915060408401356001600160401b03811115612204575f80fd5b612210868287016120e4565b9150509250925092565b5f806040838503121561222b575f80fd5b82356001600160401b03811115612240575f80fd5b61224c858286016120e4565b92505060208301356001600160401b038111156120ac575f80fd5b5f5b83811015612281578181015183820152602001612269565b50505f910152565b5f82601f830112612298575f80fd5b81516122a6611cd082611c7d565b8082825260208201915060208360051b8601019250858311156122c7575f80fd5b602085015b83811015611d1c5780516001600160401b038111156122e9575f80fd5b8601603f810188136122f9575f80fd5b602081015161230a611cd082611d8b565b8181526040838301018a101561231e575f80fd5b61232f826020830160408601612267565b85525050602092830192016122cc565b805161234a81611d5f565b919050565b5f805f60608486031215612361575f80fd5b83516001600160401b03811115612376575f80fd5b8401601f81018613612386575f80fd5b8051612394611cd082611c7d565b8082825260208201915060208360051b8501019250888311156123b5575f80fd5b6020840193505b828410156123e05783516123cf81611c9f565b8252602093840193909101906123bc565b8096505050505060208401516001600160401b038111156123ff575f80fd5b61240b86828701612289565b92505061241a6040850161233f565b90509250925092565b634e487b7160e01b5f52603260045260245ffd5b6001600160a01b03831681526040602080830182905283519183018290525f91908401906060840190835b818110156124895783516001600160a01b0316835260209384019390920191600101612462565b50909695505050505050565b5f602082840312156124a5575f80fd5b81516001600160401b038111156124ba575f80fd5b8201601f810184136124ca575f80fd5b80516124d8611cd082611c7d565b8082825260208201915060208360051b8501019250868311156124f9575f80fd5b6020840193505b8284101561251b578351825260209384019390910190612500565b9695505050505050565b634e487b7160e01b5f52601160045260245ffd5b80820281158282048414176103a3576103a3612525565b808201808211156103a3576103a3612525565b5f8261257d57634e487b7160e01b5f52601260045260245ffd5b500490565b8082018281125f8312801582168215821617156125a1576125a1612525565b505092915050565b818103818111156103a3576103a3612525565b5f600182016125cd576125cd612525565b5060010190565b5f81518084526125eb816020860160208601612267565b601f01601f19169290920160200192915050565b60018060a01b0383168152604060208201525f82516060604084015261262860a08401826125d4565b90506020840151606084015260408401516080840152809150509392505050565b5f8161265757612657612525565b505f190190565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b604081525f6126bb6040830185611e41565b82810360208401526126cd8185611e41565b95945050505050565b8181035f8312801583831316838312821617156126f5576126f5612525565b5092915050565b634e487b7160e01b5f52602160045260245ffd5b828152604060208201525f611d5760408301846125d4565b5f8251612739818460208701612267565b9190910192915050565b5f60208284031215612753575f80fd5b81516001600160e01b03198116811461050d575f80fdfea2646970667358221220220a39bccd90c5a21dd46ead5b0a146a378b244f139ed46b2180a9689dc86ac364736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01lW_5`\xE0\x1C\x80cibU\xBE\x11a\0\xD9W\x80c\x98\xEC\x1A\xC9\x11a\0\x93W\x80c\xCD\xCD5\x81\x11a\0nW\x80c\xCD\xCD5\x81\x14a\x03\x0EW\x80c\xDE\xC5\xD1\xF6\x14a\x03!W\x80c\xEC\x7F\xBB1\x14a\x034W\x80c\xF2\xFD\xE3\x8B\x14a\x03oW_\x80\xFD[\x80c\x98\xEC\x1A\xC9\x14a\x02\xE0W\x80c\xAB\x11\x89\x95\x14a\x02\xF3W\x80c\xB93\xFAt\x14a\x03\x06W_\x80\xFD[\x80cibU\xBE\x14a\x02\x86W\x80cqP\x18\xA6\x14a\x02\x99W\x80ct<1\xF4\x14a\x02\xA1W\x80c\x85}\xC1\x90\x14a\x02\xB4W\x80c\x8D\xA5\xCB[\x14a\x02\xBCW\x80c\x95_-\x90\x14a\x02\xCDW_\x80\xFD[\x80c;$.J\x11a\x01*W\x80c;$.J\x14a\x02\x07W\x80c=V\x11\xF6\x14a\x02\x1AW\x80c@\xBF/\xB7\x14a\x02-W\x80cQ@\xA5H\x14a\x025W\x80c^\x10B\xE8\x14a\x02HW\x80c^\xF53)\x14a\x02sW_\x80\xFD[\x80b\xCF*\xB5\x14a\x01pW\x80c\r\xBA3\x94\x14a\x01\x85W\x80c\x16&\xBA~\x14a\x01\xABW\x80c\x17\x03\xA0\x18\x14a\x01\xD7W\x80c\x1EL\xD8^\x14a\x01\xECW\x80c1O:I\x14a\x01\xFFW[_\x80\xFD[a\x01\x83a\x01~6`\x04a\x1D&V[a\x03\x82V[\0[a\x01\x98a\x01\x936`\x04a\x1DpV[a\x03\x8EV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xBEa\x01\xB96`\x04a\x1D\xFEV[a\x03\xA9V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x01\xA2V[a\x01\xDFa\x03\xE5V[`@Qa\x01\xA2\x91\x90a\x1E\x9EV[a\x01\x98a\x01\xFA6`\x04a\x1DpV[a\x04vV[a\x01\x98a\x04\x8BV[a\x01\x98a\x02\x156`\x04a\x1E\xB0V[a\x04\x9BV[a\x01\x83a\x02(6`\x04a\x1E\xCBV[a\x04\xBBV[`gTa\x01\x98V[a\x01\x83a\x02C6`\x04a\x1F{V[a\x04\xCAV[a\x02[a\x02V6`\x04a >V[a\x04\xECV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xA2V[a\x01\x83a\x02\x816`\x04a hV[a\x05\x14V[a\x01\x83a\x02\x946`\x04a \x7FV[a\x05%V[a\x01\x83a\x05?V[a\x01\x83a\x02\xAF6`\x04a\x1E\xB0V[a\x05RV[a\x01\x83a\x05\x8BV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02[V[a\x01\x98a\x02\xDB6`\x04a \xB8V[a\x05\x94V[a\x01\x98a\x02\xEE6`\x04a\x1E\xB0V[a\x05\xBEV[a\x01\x83a\x03\x016`\x04a!\xC6V[a\x08\x03V[a\x01\x98a\t\x19V[a\x02[a\x03\x1C6`\x04a\x1E\xB0V[a\t$V[a\x01\x83a\x03/6`\x04a\"\x1AV[a\tDV[a\x03_a\x03B6`\x04a\x1E\xB0V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`n` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xA2V[a\x01\x83a\x03}6`\x04a\x1E\xB0V[a\tUV[a\x03\x8B\x81a\t\xCBV[PV[_a\x03\xA3`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[\x92\x91PPV[_\x80_\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03\xC1\x91\x90a#OV[\x92P\x92P\x92Pa\x03\xD3\x86\x84\x84\x84a\x0B\x1FV[Pc\x0B\x13]?`\xE1\x1B\x95\x94PPPPPV[`@\x80Q` \x81\x01\x90\x91R``\x81R`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R\x92\x93\x91\x92\x84\x92\x90\x91\x84\x91_\x90\x85\x01[\x82\x82\x10\x15a\x04iW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x04\x1BV[PPPP\x81RPP\x90P\x90V[_a\x03\xA3`lc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[_a\x04\x96`ka\x0B\xCEV[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`m` R`@\x81 a\x03\xA3\x90a\x0B\xCEV[a\x04\xC63\x83\x83a\x0C'V[PPV[a\x04\xC6\x82_\x81Q\x81\x10a\x04\xDFWa\x04\xDFa$#V[` \x02` \x01\x01Qa\rVV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`j` R`@\x81 a\x05\r\x90\x83a\n\x17V[\x93\x92PPPV[a\x05\x1Ca\ryV[a\x03\x8B\x81a\r\xD3V[a\x05-a\ryV[a\x056\x82a\x0E\x16V[a\x04\xC6\x81a\t\xCBV[a\x05Ga\ryV[a\x05P_a\x0E\\V[V[3_\x90\x81R`n` R`@\x90 T`\xFF\x16a\x05\x81W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x8B3\x82a\x0E\xADV[a\x05P3a\x0F]V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`m` R`@\x81 a\x05\r\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[_\x80`f_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x061W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x05\xE3V[PPPP\x90P_\x80\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06RWa\x06Ra\x1B\xEFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06{W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\xD8W\x83\x81\x81Q\x81\x10a\x06\x9BWa\x06\x9Ba$#V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a\x06\xB8Wa\x06\xB8a$#V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x06\x80V[P`@Qc\x90\x04\x13G`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x90\x04\x13G\x90a\x07)\x90\x89\x90\x86\x90`\x04\x01a$7V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07CW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07j\x91\x90\x81\x01\x90a$\x95V[\x90P_[\x84Q\x81\x10\x15a\x07\xD6W\x84\x81\x81Q\x81\x10a\x07\x89Wa\x07\x89a$#V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x07\xB0Wa\x07\xB0a$#V[` \x02` \x01\x01Qa\x07\xC2\x91\x90a%9V[a\x07\xCC\x90\x85a%PV[\x93P`\x01\x01a\x07nV[Pa\x07\xE3a'\x10\x84a%cV[\x92P`gT\x83\x10a\x07\xF8WP\x90\x94\x93PPPPV[P_\x95\x94PPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x08!WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x08:WP0;\x15\x80\x15a\x08:WP_T`\xFF\x16`\x01\x14[a\x08\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x08\xC3W_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x08\xCE\x84\x84\x84a\x10wV[\x80\x15a\t\x13W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[_a\x04\x96`la\x0B\xCEV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`j` R`@\x81 a\x03\xA3\x90a\x0B\xCEV[a\tLa\ryV[a\x056\x82a\x10\xD7V[a\t]a\ryV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\t\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\x99V[a\x03\x8B\x81a\x0E\\V[_\x80[\x82Q\x81\x10\x15a\n\rWa\t\xF9\x83\x82\x81Q\x81\x10a\t\xECWa\t\xECa$#V[` \x02` \x01\x01Qa\x12)V[a\n\x03\x90\x83a%\x82V[\x91P`\x01\x01a\t\xCEV[Pa\t\x13\x81a\x13LV[_C\x82\x10a\ngW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R`d\x01a\x08\x99V[\x82T_[\x81\x81\x10\x15a\n\xC8W_a\n~\x82\x84a\x13\xB6V[\x90P\x84\x86_\x01\x82\x81T\x81\x10a\n\x95Wa\n\x95a$#V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\n\xB4W\x80\x92Pa\n\xC2V[a\n\xBF\x81`\x01a%PV[\x91P[Pa\nkV[\x81\x15a\x0B\x0BW\x84a\n\xDA`\x01\x84a%\xA9V[\x81T\x81\x10a\n\xEAWa\n\xEAa$#V[_\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0B\rV[_[`\x01`\x01`\xE0\x1B\x03\x16\x95\x94PPPPPV[_\x83Q\x90P_\x80_\x80a\x0B3\x85\x88Qa\x13\xD0V[_[\x85\x81\x10\x15a\x0B\xB8W\x88\x81\x81Q\x81\x10a\x0BOWa\x0BOa$#V[` \x02` \x01\x01Q\x94Pa\x0Bc\x85\x88a\x14\x13V[\x92Pa\x0Bo\x84\x86a\x14dV[a\x0B\x93\x83\x8B\x8A\x84\x81Q\x81\x10a\x0B\x86Wa\x0B\x86a$#V[` \x02` \x01\x01Qa\x14\x96V[\x84\x93P_a\x0B\xA1\x86\x89a\x14\xC7V[\x90Pa\x0B\xAD\x81\x84a%PV[\x92PP`\x01\x01a\x0B5V[Pa\x0B\xC3\x81\x87a\x15\x18V[PPPPPPPPPV[\x80T_\x90\x80\x15a\x0C\x15W\x82a\x0B\xE4`\x01\x83a%\xA9V[\x81T\x81\x10a\x0B\xF4Wa\x0B\xF4a$#V[_\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0C\x17V[_[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`n` R`@\x90 T`\xFF\x16\x15a\x0C`W`@QcB\xEEh\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90_a\x0Co\x83a%\xBCV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`n` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0C\x9F\x84a\x12)V[\x90Pa\x0C\xAA\x81a\x13LV[PPa\x0C\xB6\x84\x83a\x0E\xADV[`hT`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x99&\xEE}\x90a\x0C\xE8\x90\x87\x90\x87\x90`\x04\x01a%\xFFV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xFFW_\x80\xFD[PZ\xF1\x15\x80\x15a\r\x11W=_\x80>=_\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x87\x16\x91P\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90_\x90\xA3PPPPV[`eT\x81Q\x14a\x03\x82W`@Qc\x16\x9E\xFB[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08\x99V[a\r\xDE`l\x82a\x15rV[PP`@Q\x81\x81R\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`g\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8F\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`j` R`@\x81 a\x0E\xCD\x90a\x0B\xCEV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0E\xEDWPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`j` R`@\x90 a\x0F\x10\x91\x84\x16a\x15rV[PP`@Q`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82R\x80\x84\x16\x91C\x91\x86\x16\x90\x7F\xD0a\x16\x82R\xF4As6X\xF0\x9EM\x8F[-\x99\x8E\xD4\xEF$\xA2\xBB\xFDl\xEC\xA5.\xA11P\x02\x90` \x01`@Q\x80\x91\x03\x90\xA4PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`n` R`@\x90 T`\xFF\x16a\x0F\x95W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90_a\x0F\xA4\x83a&IV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`n` R`@\x81 \x80T`\xFF\x19\x16\x90Ua\x0F\xD1\x82a\x12)V[\x90Pa\x0F\xDC\x81a\x13LV[PP`hT`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xA3d\xF4\xDA\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\"W_\x80\xFD[PZ\xF1\x15\x80\x15a\x104W=_\x80>=_\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x85\x16\x91P\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80\x90_\x90\xA3PPV[_Ta\x01\0\x90\x04`\xFF\x16a\x10\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x99\x90a&^V[`h\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua\x10\xC1\x82a\r\xD3V[a\x10\xCA\x81a\x10\xD7V[a\x10\xD2a\x16\x95V[PPPV[a\x10\xE0\x81a\x16\xC3V[a\x10\xFDW`@Qc\xD1sWy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R_\x94\x84\x92\x84\x91\x87\x90\x85\x01[\x82\x82\x10\x15a\x11nW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x11 V[PPP\x91RP\x90\x91P`f\x90P_a\x11\x86\x82\x82a\x1B\xC5V[PP_[\x82QQ\x81\x10\x15a\x11\xF7W\x82Q\x80Q`f\x91\x90\x83\x90\x81\x10a\x11\xACWa\x11\xACa$#V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x81\x01\x85U_\x94\x85R\x93\x83\x90 \x82Q\x92\x90\x93\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x91\x01U\x01a\x11\x8AV[P\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x81\x83`@Qa\x0EP\x92\x91\x90a&\xA9V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`m` R`@\x81 \x81\x90\x81\x90\x81\x90a\x12O\x90a\x0B\xCEV[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`n` R`@\x90 T\x90\x91P`\xFF\x16a\x12\xB4Wa\x12{\x81\x84a&\xD6V[\x92P\x82_\x03a\x12\x8DWP\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`m` R`@\x81 a\x12\xAD\x91a\x15rV[PPa\x12\xFFV[a\x12\xBD\x85a\x05\xBEV[\x91Pa\x12\xC9\x81\x83a&\xD6V[\x92P\x82_\x03a\x12\xDBWP\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`m` R`@\x90 a\x12\xFC\x90\x83a\x15rV[PP[`@\x80Q\x82\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16\x91\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x91\x01`@Q\x80\x91\x03\x90\xA2P\x90\x93\x92PPPV[_\x80a\x13X`ka\x0B\xCEV[\x91P_a\x13e\x84\x84a%\x82V[\x91P\x81\x90Pa\x13u`k\x82a\x15rV[PP`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B\x91\x01`@Q\x80\x91\x03\x90\xA1P\x91P\x91V[_a\x13\xC4`\x02\x84\x84\x18a%cV[a\x05\r\x90\x84\x84\x16a%PV[\x80\x82\x14a\x13\xF3W`@Q`\x01b\x13\x98\xB9`\xE3\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81_\x03a\x04\xC6W`@Qc%\x1FV\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x14:W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`j` R`@\x90 a\x05\r\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x10a\x04\xC6W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xAA`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a\x17\x86V[a\x10\xD2W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x14\xEEW`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`m` R`@\x90 a\x05\r\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[_a\x15\"\x82a\x18\xCBV[\x90P\x80\x83\x11\x15a\x15EW`@QcK\x05\xA0\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x15O\x83a\x19\x06V[\x90P\x83\x81\x11\x15a\t\x13W`@Qc\xE1!c/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T_\x90\x81\x90\x81a\x15\x82\x86a\x0B\xCEV[\x90P_\x82\x11\x80\x15a\x15\xBEWPC\x86a\x15\x9B`\x01\x85a%\xA9V[\x81T\x81\x10a\x15\xABWa\x15\xABa$#V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x16\x1BWa\x15\xCC\x85a\x19AV[\x86a\x15\xD8`\x01\x85a%\xA9V[\x81T\x81\x10a\x15\xE8Wa\x15\xE8a$#V[\x90_R` _ \x01_\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xE0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xE0\x1B\x03\x16\x02\x17\x90UPa\x16\x87V[\x85_\x01`@Q\x80`@\x01`@R\x80a\x162Ca\x19\xADV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x16F\x88a\x19AV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U_\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[\x92P\x83\x91PP[\x92P\x92\x90PV[_Ta\x01\0\x90\x04`\xFF\x16a\x16\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x99\x90a&^V[a\x05Pa\x1A\x11V[\x80Q_\x90\x81\x80\x80\x80[\x84Q\x81\x10\x15a\x17eW\x84\x81\x81Q\x81\x10a\x16\xE7Wa\x16\xE7a$#V[` \x02` \x01\x01Q_\x01Q\x92P\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x17&W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x93P\x84\x81\x81Q\x81\x10a\x17;Wa\x17;a$#V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82a\x17[\x91\x90a%PV[\x91P`\x01\x01a\x16\xCCV[Pa'\x10\x81\x14a\x17zWP_\x95\x94PPPPPV[P`\x01\x95\x94PPPPPV[_\x80_a\x17\x93\x85\x85a\x1A@V[\x90\x92P\x90P_\x81`\x04\x81\x11\x15a\x17\xABWa\x17\xABa&\xFCV[\x14\x80\x15a\x17\xC9WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x17\xD9W`\x01\x92PPPa\x05\rV[_\x80\x87`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01a\x18\0\x92\x91\x90a'\x10V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x18>\x91\x90a'(V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a\x18vW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x18{V[``\x91P[P\x91P\x91P\x81\x80\x15a\x18\x8EWP\x80Q` \x14[\x80\x15a\x18\xBFWP\x80Qc\x0B\x13]?`\xE1\x1B\x90a\x18\xB3\x90\x83\x01` \x90\x81\x01\x90\x84\x01a'CV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14[\x98\x97PPPPPPPPV[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x18\xF2W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xA3`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x19-W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xA3`lc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x17\x16V[_`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x19\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x08\x99V[P\x90V[_c\xFF\xFF\xFF\xFF\x82\x11\x15a\x19\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\x99V[_Ta\x01\0\x90\x04`\xFF\x16a\x1A7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\x99\x90a&^V[a\x05P3a\x0E\\V[_\x80\x82Q`A\x03a\x1AtW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa\x1Ah\x87\x82\x85\x85a\x1A\xA8V[\x94P\x94PPPPa\x16\x8EV[\x82Q`@\x03a\x1A\x9DW` \x83\x01Q`@\x84\x01Qa\x1A\x92\x86\x83\x83a\x1B\x8DV[\x93P\x93PPPa\x16\x8EV[P_\x90P`\x02a\x16\x8EV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1A\xDDWP_\x90P`\x03a\x1B\x84V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a\x1A\xF5WP\x84`\xFF\x16`\x1C\x14\x15[\x15a\x1B\x05WP_\x90P`\x04a\x1B\x84V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1BVW=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1B~W_`\x01\x92P\x92PPa\x1B\x84V[\x91P_\x90P[\x94P\x94\x92PPPV[_\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a\x1B\xA9`\xFF\x86\x90\x1C`\x1Ba%PV[\x90Pa\x1B\xB7\x87\x82\x88\x85a\x1A\xA8V[\x93P\x93PPP\x93P\x93\x91PPV[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a\x03\x8B\x91\x90[\x80\x82\x11\x15a\x19\xA9W_\x81U`\x01\x01a\x1B\xDCV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C%Wa\x1C%a\x1B\xEFV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C%Wa\x1C%a\x1B\xEFV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1CuWa\x1Cua\x1B\xEFV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1C\x95Wa\x1C\x95a\x1B\xEFV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x8BW_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x1C\xC2W_\x80\xFD[\x815a\x1C\xD5a\x1C\xD0\x82a\x1C}V[a\x1CMV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x1C\xF6W_\x80\xFD[` \x85\x01[\x83\x81\x10\x15a\x1D\x1CW\x805a\x1D\x0E\x81a\x1C\x9FV[\x83R` \x92\x83\x01\x92\x01a\x1C\xFBV[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x1D6W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DKW_\x80\xFD[a\x1DW\x84\x82\x85\x01a\x1C\xB3V[\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\x8BW_\x80\xFD[_` \x82\x84\x03\x12\x15a\x1D\x80W_\x80\xFD[\x815a\x05\r\x81a\x1D_V[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1D\xA3Wa\x1D\xA3a\x1B\xEFV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x1D\xC0W_\x80\xFD[\x815a\x1D\xCEa\x1C\xD0\x82a\x1D\x8BV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1D\xE2W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x1E\x0FW_\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E+W_\x80\xFD[a\x1E7\x85\x82\x86\x01a\x1D\xB1V[\x91PP\x92P\x92\x90PV[\x80Q` \x80\x84R\x81Q\x84\x82\x01\x81\x90R_\x92\x90\x91\x01\x90\x82\x90`@\x86\x01\x90[\x80\x83\x10\x15a\x1D\x1CW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x84\x01R\x90\x93\x01\x92`\x01\x92\x90\x92\x01\x91`@\x90\x91\x01\x90a\x1E^V[` \x81R_a\x05\r` \x83\x01\x84a\x1EAV[_` \x82\x84\x03\x12\x15a\x1E\xC0W_\x80\xFD[\x815a\x05\r\x81a\x1C\x9FV[_\x80`@\x83\x85\x03\x12\x15a\x1E\xDCW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xF1W_\x80\xFD[\x83\x01``\x81\x86\x03\x12\x15a\x1F\x02W_\x80\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1F$Wa\x1F$a\x1B\xEFV[`@R\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F<W_\x80\xFD[a\x1FH\x87\x82\x85\x01a\x1D\xB1V[\x82RP` \x82\x81\x015\x81\x83\x01R`@\x92\x83\x015\x92\x82\x01\x92\x90\x92R\x92P\x83\x015a\x1Fp\x81a\x1C\x9FV[\x80\x91PP\x92P\x92\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x1F\x8CW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xA1W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x1F\xB1W_\x80\xFD[\x805a\x1F\xBFa\x1C\xD0\x82a\x1C}V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a\x1F\xE0W_\x80\xFD[` \x84\x01[\x83\x81\x10\x15a  W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a \x02W_\x80\xFD[a \x11\x8A` \x83\x89\x01\x01a\x1C\xB3V[\x84RP` \x92\x83\x01\x92\x01a\x1F\xE5V[P\x94PPPP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E+W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a OW_\x80\xFD[\x825a Z\x81a\x1C\x9FV[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a xW_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a \x90W_\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \xACW_\x80\xFD[a\x1E7\x85\x82\x86\x01a\x1C\xB3V[_\x80`@\x83\x85\x03\x12\x15a \xC9W_\x80\xFD[\x825a \xD4\x81a\x1C\x9FV[\x91P` \x83\x015a\x1Fp\x81a\x1D_V[_` \x82\x84\x03\x12\x15a \xF4W_\x80\xFD[a \xFCa\x1C\x03V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x13W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a!#W_\x80\xFD[\x805a!1a\x1C\xD0\x82a\x1C}V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a!RW_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a!\xBAW`@\x84\x88\x03\x12\x15a!pW_\x80\xFD[a!xa\x1C+V[\x845a!\x83\x81a\x1C\x9FV[\x81R` \x85\x015`\x01`\x01``\x1B\x03\x81\x16\x81\x14a!\x9EW_\x80\xFD[\x80` \x83\x01RP\x80\x83RP` \x82\x01\x91P`@\x84\x01\x93Pa!YV[\x84RP\x91\x94\x93PPPPV[_\x80_``\x84\x86\x03\x12\x15a!\xD8W_\x80\xFD[\x835a!\xE3\x81a\x1C\x9FV[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x04W_\x80\xFD[a\"\x10\x86\x82\x87\x01a \xE4V[\x91PP\x92P\x92P\x92V[_\x80`@\x83\x85\x03\x12\x15a\"+W_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\"@W_\x80\xFD[a\"L\x85\x82\x86\x01a \xE4V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \xACW_\x80\xFD[_[\x83\x81\x10\x15a\"\x81W\x81\x81\x01Q\x83\x82\x01R` \x01a\"iV[PP_\x91\x01RV[_\x82`\x1F\x83\x01\x12a\"\x98W_\x80\xFD[\x81Qa\"\xA6a\x1C\xD0\x82a\x1C}V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\"\xC7W_\x80\xFD[` \x85\x01[\x83\x81\x10\x15a\x1D\x1CW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xE9W_\x80\xFD[\x86\x01`?\x81\x01\x88\x13a\"\xF9W_\x80\xFD[` \x81\x01Qa#\na\x1C\xD0\x82a\x1D\x8BV[\x81\x81R`@\x83\x83\x01\x01\x8A\x10\x15a#\x1EW_\x80\xFD[a#/\x82` \x83\x01`@\x86\x01a\"gV[\x85RPP` \x92\x83\x01\x92\x01a\"\xCCV[\x80Qa#J\x81a\x1D_V[\x91\x90PV[_\x80_``\x84\x86\x03\x12\x15a#aW_\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#vW_\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a#\x86W_\x80\xFD[\x80Qa#\x94a\x1C\xD0\x82a\x1C}V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a#\xB5W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a#\xE0W\x83Qa#\xCF\x81a\x1C\x9FV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a#\xBCV[\x80\x96PPPPP` \x84\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xFFW_\x80\xFD[a$\x0B\x86\x82\x87\x01a\"\x89V[\x92PPa$\x1A`@\x85\x01a#?V[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x80\x83\x01\x82\x90R\x83Q\x91\x83\x01\x82\x90R_\x91\x90\x84\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a$\x89W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a$bV[P\x90\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a$\xA5W_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xBAW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a$\xCAW_\x80\xFD[\x80Qa$\xD8a\x1C\xD0\x82a\x1C}V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a$\xF9W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a%\x1BW\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a%\0V[\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xA3Wa\x03\xA3a%%V[\x80\x82\x01\x80\x82\x11\x15a\x03\xA3Wa\x03\xA3a%%V[_\x82a%}WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a%\xA1Wa%\xA1a%%V[PP\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x03\xA3Wa\x03\xA3a%%V[_`\x01\x82\x01a%\xCDWa%\xCDa%%V[P`\x01\x01\x90V[_\x81Q\x80\x84Ra%\xEB\x81` \x86\x01` \x86\x01a\"gV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q```@\x84\x01Ra&(`\xA0\x84\x01\x82a%\xD4V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[_\x81a&WWa&Wa%%V[P_\x19\x01\x90V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_a&\xBB`@\x83\x01\x85a\x1EAV[\x82\x81\x03` \x84\x01Ra&\xCD\x81\x85a\x1EAV[\x95\x94PPPPPV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&\xF5Wa&\xF5a%%V[P\x92\x91PPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82\x81R`@` \x82\x01R_a\x1DW`@\x83\x01\x84a%\xD4V[_\x82Qa'9\x81\x84` \x87\x01a\"gV[\x91\x90\x91\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15a'SW_\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x05\rW_\x80\xFD\xFE\xA2dipfsX\"\x12 \"\n9\xBC\xCD\x90\xC5\xA2\x1D\xD4n\xAD[\n\x14j7\x8B$O\x13\x9E\xD4k!\x80\xA9h\x9D\xC8j\xC3dsolcC\0\x08\x1A\x003",
    );
    /**```solidity
    struct Quorum { StrategyParams[] strategies; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Quorum {
        pub strategies:
            alloy::sol_types::private::Vec<<StrategyParams as alloy::sol_types::SolType>::RustType>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Array<StrategyParams>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<<StrategyParams as alloy::sol_types::SolType>::RustType>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Quorum> for UnderlyingRustTuple<'_> {
            fn from(value: Quorum) -> Self {
                (value.strategies,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Quorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { strategies: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Quorum {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Quorum {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Quorum {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Quorum {
            const NAME: &'static str = "Quorum";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("Quorum(StrategyParams[] strategies)")
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(<StrategyParams as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<StrategyParams as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                <alloy::sol_types::sol_data::Array<
                    StrategyParams,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Quorum {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyParams,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    StrategyParams,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct StrategyParams { address strategy; uint96 multiplier; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyParams {
        pub strategy: alloy::sol_types::private::Address,
        pub multiplier: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> =
            (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<96>);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U96,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StrategyParams> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyParams) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { strategy: tuple.0, multiplier: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.multiplier,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StrategyParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StrategyParams {
            const NAME: &'static str = "StrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyParams(address strategy,uint96 multiplier)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.multiplier)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StrategyParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.multiplier,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategy,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.multiplier,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**Custom error with signature `InsufficientSignedStake()` and selector `0xe121632f`.
    ```solidity
    error InsufficientSignedStake();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientSignedStake {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientSignedStake> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientSignedStake) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientSignedStake {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientSignedStake {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientSignedStake()";
            const SELECTOR: [u8; 4] = [225u8, 33u8, 99u8, 47u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InsufficientWeight()` and selector `0xa8792fd1`.
    ```solidity
    error InsufficientWeight();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientWeight {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientWeight> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientWeight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientWeight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientWeight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientWeight()";
            const SELECTOR: [u8; 4] = [168u8, 121u8, 47u8, 209u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidLength()` and selector `0x947d5a84`.
    ```solidity
    error InvalidLength();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidLength {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidLength()";
            const SELECTOR: [u8; 4] = [148u8, 125u8, 90u8, 132u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidQuorum()` and selector `0xd1735779`.
    ```solidity
    error InvalidQuorum();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidQuorum {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidQuorum> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidQuorum) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidQuorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidQuorum {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidQuorum()";
            const SELECTOR: [u8; 4] = [209u8, 115u8, 87u8, 121u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidReferenceBlock()` and selector `0xe64f180f`.
    ```solidity
    error InvalidReferenceBlock();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidReferenceBlock {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidReferenceBlock> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidReferenceBlock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidReferenceBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidReferenceBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidReferenceBlock()";
            const SELECTOR: [u8; 4] = [230u8, 79u8, 24u8, 15u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
    ```solidity
    error InvalidSignature();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidSignedWeight()` and selector `0x960b41ee`.
    ```solidity
    error InvalidSignedWeight();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignedWeight {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSignedWeight> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignedWeight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignedWeight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignedWeight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignedWeight()";
            const SELECTOR: [u8; 4] = [150u8, 11u8, 65u8, 238u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidThreshold()` and selector `0xaabd5a09`.
    ```solidity
    error InvalidThreshold();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidThreshold {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidThreshold()";
            const SELECTOR: [u8; 4] = [170u8, 189u8, 90u8, 9u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `LengthMismatch()` and selector `0xff633a38`.
    ```solidity
    error LengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LengthMismatch {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<LengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: LengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LengthMismatch()";
            const SELECTOR: [u8; 4] = [255u8, 99u8, 58u8, 56u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `MustUpdateAllOperators()` and selector `0x2d3df6b6`.
    ```solidity
    error MustUpdateAllOperators();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MustUpdateAllOperators {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MustUpdateAllOperators> for UnderlyingRustTuple<'_> {
            fn from(value: MustUpdateAllOperators) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MustUpdateAllOperators {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MustUpdateAllOperators {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MustUpdateAllOperators()";
            const SELECTOR: [u8; 4] = [45u8, 61u8, 246u8, 182u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `NotSorted()` and selector `0xba50f911`.
    ```solidity
    error NotSorted();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotSorted {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotSorted> for UnderlyingRustTuple<'_> {
            fn from(value: NotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotSorted()";
            const SELECTOR: [u8; 4] = [186u8, 80u8, 249u8, 17u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `OperatorAlreadyRegistered()` and selector `0x42ee68b5`.
    ```solidity
    error OperatorAlreadyRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorAlreadyRegistered {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorAlreadyRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorAlreadyRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorAlreadyRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorAlreadyRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorAlreadyRegistered()";
            const SELECTOR: [u8; 4] = [66u8, 238u8, 104u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `OperatorNotRegistered()` and selector `0x25ec6c1f`.
    ```solidity
    error OperatorNotRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotRegistered {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorNotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotRegistered()";
            const SELECTOR: [u8; 4] = [37u8, 236u8, 108u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
    ```solidity
    event Initialized(uint8 version);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8,
                    56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8,
                    96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (<alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                    &self.version,
                ),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MinimumWeightUpdated(uint256,uint256)` and selector `0x713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f`.
    ```solidity
    event MinimumWeightUpdated(uint256 _old, uint256 _new);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct MinimumWeightUpdated {
        #[allow(missing_docs)]
        pub _old: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _new: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for MinimumWeightUpdated {
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::Uint<256>, alloy::sol_types::sol_data::Uint<256>);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MinimumWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    113u8, 60u8, 165u8, 59u8, 136u8, 214u8, 235u8, 99u8, 245u8, 177u8, 133u8, 76u8,
                    184u8, 203u8, 221u8, 115u8, 110u8, 197u8, 30u8, 218u8, 34u8, 94u8, 70u8, 121u8,
                    26u8, 169u8, 41u8, 139u8, 1u8, 96u8, 100u8, 143u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _old: data.0, _new: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._old,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._new,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MinimumWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MinimumWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MinimumWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorDeregistered(address,address)` and selector `0x31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed580`.
    ```solidity
    event OperatorDeregistered(address indexed _operator, address indexed _avs);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct OperatorDeregistered {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _avs: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorDeregistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorDeregistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 224u8, 173u8, 254u8, 199u8, 27u8, 204u8, 238u8, 55u8, 182u8, 232u8, 58u8,
                    144u8, 194u8, 254u8, 219u8, 23u8, 216u8, 241u8, 105u8, 63u8, 238u8, 134u8,
                    60u8, 71u8, 113u8, 231u8, 191u8, 226u8, 174u8, 213u8, 128u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _operator: topics.1, _avs: topics.2 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self._operator.clone(), self._avs.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorDeregistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorDeregistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorDeregistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRegistered(address,address)` and selector `0xa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c1`.
    ```solidity
    event OperatorRegistered(address indexed _operator, address indexed _avs);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct OperatorRegistered {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _avs: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorRegistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    164u8, 83u8, 219u8, 97u8, 42u8, 245u8, 158u8, 85u8, 33u8, 214u8, 171u8, 146u8,
                    132u8, 220u8, 62u8, 45u8, 6u8, 175u8, 40u8, 110u8, 177u8, 177u8, 183u8, 183u8,
                    113u8, 252u8, 228u8, 113u8, 108u8, 25u8, 242u8, 193u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _operator: topics.1, _avs: topics.2 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self._operator.clone(), self._avs.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorRegistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRegistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorRegistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorWeightUpdated(address,uint256,uint256)` and selector `0x88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594`.
    ```solidity
    event OperatorWeightUpdated(address indexed _operator, uint256 oldWeight, uint256 newWeight);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct OperatorWeightUpdated {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorWeightUpdated {
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::Uint<256>, alloy::sol_types::sol_data::Uint<256>);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList =
                (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            const SIGNATURE: &'static str = "OperatorWeightUpdated(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    136u8, 119u8, 13u8, 200u8, 98u8, 228u8, 122u8, 126u8, 213u8, 134u8, 144u8,
                    120u8, 87u8, 235u8, 27u8, 117u8, 228u8, 197u8, 255u8, 200u8, 183u8, 7u8, 199u8,
                    238u8, 16u8, 235u8, 116u8, 214u8, 136u8, 95u8, 229u8, 148u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _operator: topics.1, oldWeight: data.0, newWeight: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.oldWeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newWeight,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self._operator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
    ```solidity
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8,
                    208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8,
                    175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { previousOwner: topics.1, newOwner: topics.2 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.previousOwner.clone(), self.newOwner.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumUpdated(((address,uint96)[]),((address,uint96)[]))` and selector `0x23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e`.
    ```solidity
    event QuorumUpdated(Quorum _old, Quorum _new);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct QuorumUpdated {
        #[allow(missing_docs)]
        pub _old: <Quorum as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _new: <Quorum as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for QuorumUpdated {
            type DataTuple<'a> = (Quorum, Quorum);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "QuorumUpdated(((address,uint96)[]),((address,uint96)[]))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    35u8, 170u8, 212u8, 230u8, 23u8, 68u8, 236u8, 225u8, 100u8, 19u8, 10u8, 164u8,
                    21u8, 193u8, 97u8, 110u8, 128u8, 19u8, 107u8, 15u8, 7u8, 112u8, 229u8, 101u8,
                    137u8, 67u8, 139u8, 144u8, 178u8, 105u8, 38u8, 94u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _old: data.0, _new: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._old),
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._new),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SigningKeyUpdate(address,uint256,address,address)` and selector `0xd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea1315002`.
    ```solidity
    event SigningKeyUpdate(address indexed operator, uint256 indexed updateBlock, address indexed newSigningKey, address oldSigningKey);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct SigningKeyUpdate {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub updateBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newSigningKey: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldSigningKey: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SigningKeyUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SigningKeyUpdate(address,uint256,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    208u8, 97u8, 22u8, 130u8, 82u8, 244u8, 65u8, 115u8, 54u8, 88u8, 240u8, 158u8,
                    77u8, 143u8, 91u8, 45u8, 153u8, 142u8, 212u8, 239u8, 36u8, 162u8, 187u8, 253u8,
                    108u8, 236u8, 165u8, 46u8, 161u8, 49u8, 80u8, 2u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    updateBlock: topics.2,
                    newSigningKey: topics.3,
                    oldSigningKey: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.oldSigningKey,
                ),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.operator.clone(),
                    self.updateBlock.clone(),
                    self.newSigningKey.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.updateBlock);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newSigningKey,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SigningKeyUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SigningKeyUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SigningKeyUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ThresholdWeightUpdated(uint256)` and selector `0x9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b`.
    ```solidity
    event ThresholdWeightUpdated(uint256 _thresholdWeight);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct ThresholdWeightUpdated {
        #[allow(missing_docs)]
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ThresholdWeightUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ThresholdWeightUpdated(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    147u8, 36u8, 247u8, 229u8, 167u8, 192u8, 40u8, 136u8, 8u8, 166u8, 52u8, 204u8,
                    222u8, 68u8, 184u8, 233u8, 121u8, 103u8, 100u8, 116u8, 178u8, 46u8, 41u8,
                    238u8, 157u8, 213u8, 105u8, 181u8, 94u8, 121u8, 26u8, 75u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _thresholdWeight: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self._thresholdWeight,
                ),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ThresholdWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ThresholdWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ThresholdWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TotalWeightUpdated(uint256,uint256)` and selector `0x86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b`.
    ```solidity
    event TotalWeightUpdated(uint256 oldTotalWeight, uint256 newTotalWeight);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct TotalWeightUpdated {
        #[allow(missing_docs)]
        pub oldTotalWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newTotalWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TotalWeightUpdated {
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::Uint<256>, alloy::sol_types::sol_data::Uint<256>);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TotalWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    134u8, 220u8, 248u8, 107u8, 18u8, 223u8, 238u8, 222u8, 167u8, 74u8, 233u8,
                    48u8, 13u8, 189u8, 170u8, 25u8, 59u8, 204u8, 229u8, 128u8, 147u8, 105u8, 200u8,
                    23u8, 126u8, 162u8, 244u8, 234u8, 170u8, 101u8, 114u8, 155u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { oldTotalWeight: data.0, newTotalWeight: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.oldTotalWeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newTotalWeight,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TotalWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TotalWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TotalWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UpdateMinimumWeight(uint256,uint256)` and selector `0x1ea42186b305fa37310450d9fb87ea1e8f0c7f447e771479e3b27634bfe84dc1`.
    ```solidity
    event UpdateMinimumWeight(uint256 oldMinimumWeight, uint256 newMinimumWeight);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct UpdateMinimumWeight {
        #[allow(missing_docs)]
        pub oldMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UpdateMinimumWeight {
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::Uint<256>, alloy::sol_types::sol_data::Uint<256>);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateMinimumWeight(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    30u8, 164u8, 33u8, 134u8, 179u8, 5u8, 250u8, 55u8, 49u8, 4u8, 80u8, 217u8,
                    251u8, 135u8, 234u8, 30u8, 143u8, 12u8, 127u8, 68u8, 126u8, 119u8, 20u8, 121u8,
                    227u8, 178u8, 118u8, 52u8, 191u8, 232u8, 77u8, 193u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { oldMinimumWeight: data.0, newMinimumWeight: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.oldMinimumWeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newMinimumWeight,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UpdateMinimumWeight {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateMinimumWeight> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpdateMinimumWeight) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _delegationManager);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _delegationManager: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._delegationManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _delegationManager: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self._delegationManager,
                ),)
            }
        }
    };
    /**Function with signature `deregisterOperator()` and selector `0x857dc190`.
    ```solidity
    function deregisterOperator() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {}
    ///Container type for the return parameters of the [`deregisterOperator()`](deregisterOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator()";
            const SELECTOR: [u8; 4] = [133u8, 125u8, 193u8, 144u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointOperatorWeight(address)` and selector `0x3b242e4a`.
    ```solidity
    function getLastCheckpointOperatorWeight(address _operator) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointOperatorWeightCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getLastCheckpointOperatorWeight(address)`](getLastCheckpointOperatorWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointOperatorWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointOperatorWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointOperatorWeightCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointOperatorWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointOperatorWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointOperatorWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointOperatorWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointOperatorWeightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointOperatorWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointOperatorWeight(address)";
            const SELECTOR: [u8; 4] = [59u8, 36u8, 46u8, 74u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self._operator,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointThresholdWeight()` and selector `0xb933fa74`.
    ```solidity
    function getLastCheckpointThresholdWeight() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightCall {}
    ///Container type for the return parameters of the [`getLastCheckpointThresholdWeight()`](getLastCheckpointThresholdWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointThresholdWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointThresholdWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointThresholdWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointThresholdWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointThresholdWeight()";
            const SELECTOR: [u8; 4] = [185u8, 51u8, 250u8, 116u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointThresholdWeightAtBlock(uint32)` and selector `0x1e4cd85e`.
    ```solidity
    function getLastCheckpointThresholdWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightAtBlockCall {
        pub _blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getLastCheckpointThresholdWeightAtBlock(uint32)`](getLastCheckpointThresholdWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightAtBlockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightAtBlockCall>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: getLastCheckpointThresholdWeightAtBlockCall) -> Self {
                    (value._blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for getLastCheckpointThresholdWeightAtBlockCall
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _blockNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointThresholdWeightAtBlockReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: getLastCheckpointThresholdWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for getLastCheckpointThresholdWeightAtBlockReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointThresholdWeightAtBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointThresholdWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointThresholdWeightAtBlock(uint32)";
            const SELECTOR: [u8; 4] = [30u8, 76u8, 216u8, 94u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                    &self._blockNumber,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointTotalWeight()` and selector `0x314f3a49`.
    ```solidity
    function getLastCheckpointTotalWeight() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightCall {}
    ///Container type for the return parameters of the [`getLastCheckpointTotalWeight()`](getLastCheckpointTotalWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointTotalWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointTotalWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointTotalWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointTotalWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointTotalWeight()";
            const SELECTOR: [u8; 4] = [49u8, 79u8, 58u8, 73u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastCheckpointTotalWeightAtBlock(uint32)` and selector `0x0dba3394`.
    ```solidity
    function getLastCheckpointTotalWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightAtBlockCall {
        pub _blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getLastCheckpointTotalWeightAtBlock(uint32)`](getLastCheckpointTotalWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightAtBlockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightAtBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightAtBlockCall) -> Self {
                    (value._blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointTotalWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _blockNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastCheckpointTotalWeightAtBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastCheckpointTotalWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointTotalWeightAtBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointTotalWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointTotalWeightAtBlock(uint32)";
            const SELECTOR: [u8; 4] = [13u8, 186u8, 51u8, 148u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                    &self._blockNumber,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getLastestOperatorSigningKey(address)` and selector `0xcdcd3581`.
    ```solidity
    function getLastestOperatorSigningKey(address _operator) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastestOperatorSigningKeyCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getLastestOperatorSigningKey(address)`](getLastestOperatorSigningKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastestOperatorSigningKeyReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastestOperatorSigningKeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLastestOperatorSigningKeyCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastestOperatorSigningKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLastestOperatorSigningKeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLastestOperatorSigningKeyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLastestOperatorSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastestOperatorSigningKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastestOperatorSigningKeyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastestOperatorSigningKey(address)";
            const SELECTOR: [u8; 4] = [205u8, 205u8, 53u8, 129u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self._operator,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorSigningKeyAtBlock(address,uint256)` and selector `0x5e1042e8`.
    ```solidity
    function getOperatorSigningKeyAtBlock(address _operator, uint256 _blockNumber) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSigningKeyAtBlockCall {
        pub _operator: alloy::sol_types::private::Address,
        pub _blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getOperatorSigningKeyAtBlock(address,uint256)`](getOperatorSigningKeyAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSigningKeyAtBlockReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSigningKeyAtBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSigningKeyAtBlockCall) -> Self {
                    (value._operator, value._blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSigningKeyAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0, _blockNumber: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSigningKeyAtBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSigningKeyAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSigningKeyAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSigningKeyAtBlockCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSigningKeyAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSigningKeyAtBlock(address,uint256)";
            const SELECTOR: [u8; 4] = [94u8, 16u8, 66u8, 232u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._blockNumber,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorWeight(address)` and selector `0x98ec1ac9`.
    ```solidity
    function getOperatorWeight(address _operator) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorWeight(address)`](getOperatorWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorWeightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorWeight(address)";
            const SELECTOR: [u8; 4] = [152u8, 236u8, 26u8, 201u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self._operator,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorWeightAtBlock(address,uint32)` and selector `0x955f2d90`.
    ```solidity
    function getOperatorWeightAtBlock(address _operator, uint32 _blockNumber) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightAtBlockCall {
        pub _operator: alloy::sol_types::private::Address,
        pub _blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorWeightAtBlock(address,uint32)`](getOperatorWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightAtBlockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<32>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightAtBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightAtBlockCall) -> Self {
                    (value._operator, value._blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0, _blockNumber: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorWeightAtBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorWeightAtBlockCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<32>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorWeightAtBlock(address,uint32)";
            const SELECTOR: [u8; 4] = [149u8, 95u8, 45u8, 144u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._blockNumber,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,uint256,((address,uint96)[]))` and selector `0xab118995`.
    ```solidity
    function initialize(address _serviceManager, uint256 _thresholdWeight, Quorum memory _quorum) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _serviceManager: alloy::sol_types::private::Address,
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
        pub _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initialize(address,uint256,((address,uint96)[]))`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                Quorum,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <Quorum as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._serviceManager, value._thresholdWeight, value._quorum)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _serviceManager: tuple.0, _thresholdWeight: tuple.1, _quorum: tuple.2 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                Quorum,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,uint256,((address,uint96)[]))";
            const SELECTOR: [u8; 4] = [171u8, 17u8, 137u8, 149u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._serviceManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._thresholdWeight,
                    ),
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._quorum),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`.
    ```solidity
    function isValidSignature(bytes32 _dataHash, bytes memory _signatureData) external view returns (bytes4);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureCall {
        pub _dataHash: alloy::sol_types::private::FixedBytes<32>,
        pub _signatureData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`isValidSignature(bytes32,bytes)`](isValidSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureReturn {
        pub _0: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Bytes);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::FixedBytes<32>, alloy::sol_types::private::Bytes);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureCall> for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureCall) -> Self {
                    (value._dataHash, value._signatureData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _dataHash: tuple.0, _signatureData: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isValidSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidSignatureCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Bytes);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidSignatureReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidSignature(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [22u8, 38u8, 186u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._dataHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._signatureData,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `minimumWeight()` and selector `0x40bf2fb7`.
    ```solidity
    function minimumWeight() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumWeightCall {}
    ///Container type for the return parameters of the [`minimumWeight()`](minimumWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: minimumWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: minimumWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minimumWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = minimumWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minimumWeight()";
            const SELECTOR: [u8; 4] = [64u8, 191u8, 47u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `operatorRegistered(address)` and selector `0xec7fbb31`.
    ```solidity
    function operatorRegistered(address _operator) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorRegisteredCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorRegistered(address)`](operatorRegisteredCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorRegisteredReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorRegisteredCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorRegisteredCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorRegisteredCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorRegisteredReturn> for UnderlyingRustTuple<'_> {
                fn from(value: operatorRegisteredReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorRegisteredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorRegisteredCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorRegisteredReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorRegistered(address)";
            const SELECTOR: [u8; 4] = [236u8, 127u8, 187u8, 49u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self._operator,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
    ```solidity
    function owner() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `quorum()` and selector `0x1703a018`.
    ```solidity
    function quorum() external view returns (Quorum memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumCall {}
    ///Container type for the return parameters of the [`quorum()`](quorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumReturn {
        pub _0: <Quorum as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Quorum,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (<Quorum as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumReturn;
            type ReturnTuple<'a> = (Quorum,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorum()";
            const SELECTOR: [u8; 4] = [23u8, 3u8, 160u8, 24u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `registerOperatorWithSignature((bytes,bytes32,uint256),address)` and selector `0x3d5611f6`.
    ```solidity
    function registerOperatorWithSignature(ISignatureUtils.SignatureWithSaltAndExpiry memory _operatorSignature, address _signingKey) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithSignatureCall {
        pub _operatorSignature:
            <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        pub _signingKey: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`registerOperatorWithSignature((bytes,bytes32,uint256),address)`](registerOperatorWithSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithSignatureReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (ISignatureUtils::SignatureWithSaltAndExpiry, alloy::sol_types::sol_data::Address);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorWithSignatureCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithSignatureCall) -> Self {
                    (value._operatorSignature, value._signingKey)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorWithSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operatorSignature: tuple.0, _signingKey: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorWithSignatureReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithSignatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorWithSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorWithSignatureCall {
            type Parameters<'a> =
                (ISignatureUtils::SignatureWithSaltAndExpiry, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorWithSignatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "registerOperatorWithSignature((bytes,bytes32,uint256),address)";
            const SELECTOR: [u8; 4] = [61u8, 86u8, 17u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self._operatorSignature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._signingKey,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
    ```solidity
    function renounceOwnership() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
    ```solidity
    function transferOwnership(address newOwner) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.newOwner,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateMinimumWeight(uint256,address[])` and selector `0x696255be`.
    ```solidity
    function updateMinimumWeight(uint256 _newMinimumWeight, address[] memory _operators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateMinimumWeightCall {
        pub _newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
        pub _operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`updateMinimumWeight(uint256,address[])`](updateMinimumWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateMinimumWeightReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateMinimumWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateMinimumWeightCall) -> Self {
                    (value._newMinimumWeight, value._operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateMinimumWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _newMinimumWeight: tuple.0, _operators: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateMinimumWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateMinimumWeightReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateMinimumWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateMinimumWeightCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateMinimumWeightReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateMinimumWeight(uint256,address[])";
            const SELECTOR: [u8; 4] = [105u8, 98u8, 85u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._newMinimumWeight),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperatorSigningKey(address)` and selector `0x743c31f4`.
    ```solidity
    function updateOperatorSigningKey(address _newSigningKey) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorSigningKeyCall {
        pub _newSigningKey: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`updateOperatorSigningKey(address)`](updateOperatorSigningKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorSigningKeyReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorSigningKeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorSigningKeyCall) -> Self {
                    (value._newSigningKey,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorSigningKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _newSigningKey: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorSigningKeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorSigningKeyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorSigningKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorSigningKeyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorSigningKey(address)";
            const SELECTOR: [u8; 4] = [116u8, 60u8, 49u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self._newSigningKey,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperators(address[])` and selector `0x00cf2ab5`.
    ```solidity
    function updateOperators(address[] memory _operators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsCall {
        pub _operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`updateOperators(address[])`](updateOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsCall) -> Self {
                    (value._operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operators: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperators(address[])";
            const SELECTOR: [u8; 4] = [0u8, 207u8, 42u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperatorsForQuorum(address[][],bytes)` and selector `0x5140a548`.
    ```solidity
    function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumCall {
        pub operatorsPerQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        pub _1: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`updateOperatorsForQuorum(address[][],bytes)`](updateOperatorsForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                >,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumCall) -> Self {
                    (value.operatorsPerQuorum, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorsPerQuorum: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorsForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorsForQuorum(address[][],bytes)";
            const SELECTOR: [u8; 4] = [81u8, 64u8, 165u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.operatorsPerQuorum
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateQuorumConfig(((address,uint96)[]),address[])` and selector `0xdec5d1f6`.
    ```solidity
    function updateQuorumConfig(Quorum memory _quorum, address[] memory _operators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateQuorumConfigCall {
        pub _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
        pub _operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`updateQuorumConfig(((address,uint96)[]),address[])`](updateQuorumConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateQuorumConfigReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (Quorum, alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Quorum as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateQuorumConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumConfigCall) -> Self {
                    (value._quorum, value._operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateQuorumConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _quorum: tuple.0, _operators: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateQuorumConfigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateQuorumConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateQuorumConfigCall {
            type Parameters<'a> =
                (Quorum, alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateQuorumConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateQuorumConfig(((address,uint96)[]),address[])";
            const SELECTOR: [u8; 4] = [222u8, 197u8, 209u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._quorum),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._operators),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateStakeThreshold(uint256)` and selector `0x5ef53329`.
    ```solidity
    function updateStakeThreshold(uint256 _thresholdWeight) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateStakeThresholdCall {
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateStakeThreshold(uint256)`](updateStakeThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateStakeThresholdReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateStakeThresholdCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateStakeThresholdCall) -> Self {
                    (value._thresholdWeight,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateStakeThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _thresholdWeight: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateStakeThresholdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateStakeThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateStakeThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateStakeThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateStakeThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateStakeThreshold(uint256)";
            const SELECTOR: [u8; 4] = [94u8, 245u8, 51u8, 41u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self._thresholdWeight,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`ECDSAStakeRegistry`](self) function calls.
    pub enum ECDSAStakeRegistryCalls {
        deregisterOperator(deregisterOperatorCall),
        getLastCheckpointOperatorWeight(getLastCheckpointOperatorWeightCall),
        getLastCheckpointThresholdWeight(getLastCheckpointThresholdWeightCall),
        getLastCheckpointThresholdWeightAtBlock(getLastCheckpointThresholdWeightAtBlockCall),
        getLastCheckpointTotalWeight(getLastCheckpointTotalWeightCall),
        getLastCheckpointTotalWeightAtBlock(getLastCheckpointTotalWeightAtBlockCall),
        getLastestOperatorSigningKey(getLastestOperatorSigningKeyCall),
        getOperatorSigningKeyAtBlock(getOperatorSigningKeyAtBlockCall),
        getOperatorWeight(getOperatorWeightCall),
        getOperatorWeightAtBlock(getOperatorWeightAtBlockCall),
        initialize(initializeCall),
        isValidSignature(isValidSignatureCall),
        minimumWeight(minimumWeightCall),
        operatorRegistered(operatorRegisteredCall),
        owner(ownerCall),
        quorum(quorumCall),
        registerOperatorWithSignature(registerOperatorWithSignatureCall),
        renounceOwnership(renounceOwnershipCall),
        transferOwnership(transferOwnershipCall),
        updateMinimumWeight(updateMinimumWeightCall),
        updateOperatorSigningKey(updateOperatorSigningKeyCall),
        updateOperators(updateOperatorsCall),
        updateOperatorsForQuorum(updateOperatorsForQuorumCall),
        updateQuorumConfig(updateQuorumConfigCall),
        updateStakeThreshold(updateStakeThresholdCall),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 207u8, 42u8, 181u8],
            [13u8, 186u8, 51u8, 148u8],
            [22u8, 38u8, 186u8, 126u8],
            [23u8, 3u8, 160u8, 24u8],
            [30u8, 76u8, 216u8, 94u8],
            [49u8, 79u8, 58u8, 73u8],
            [59u8, 36u8, 46u8, 74u8],
            [61u8, 86u8, 17u8, 246u8],
            [64u8, 191u8, 47u8, 183u8],
            [81u8, 64u8, 165u8, 72u8],
            [94u8, 16u8, 66u8, 232u8],
            [94u8, 245u8, 51u8, 41u8],
            [105u8, 98u8, 85u8, 190u8],
            [113u8, 80u8, 24u8, 166u8],
            [116u8, 60u8, 49u8, 244u8],
            [133u8, 125u8, 193u8, 144u8],
            [141u8, 165u8, 203u8, 91u8],
            [149u8, 95u8, 45u8, 144u8],
            [152u8, 236u8, 26u8, 201u8],
            [171u8, 17u8, 137u8, 149u8],
            [185u8, 51u8, 250u8, 116u8],
            [205u8, 205u8, 53u8, 129u8],
            [222u8, 197u8, 209u8, 246u8],
            [236u8, 127u8, 187u8, 49u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ECDSAStakeRegistryCalls {
        const NAME: &'static str = "ECDSAStakeRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointOperatorWeight(_) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointThresholdWeight(_) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointThresholdWeightAtBlock(_) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointTotalWeight(_) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointTotalWeightAtBlock(_) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastestOperatorSigningKey(_) => {
                    <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSigningKeyAtBlock(_) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorWeight(_) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorWeightAtBlock(_) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidSignature(_) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minimumWeight(_) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorRegistered(_) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::quorum(_) => <quorumCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::registerOperatorWithSignature(_) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateMinimumWeight(_) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorSigningKey(_) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperators(_) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorsForQuorum(_) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateQuorumConfig(_) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateStakeThreshold(_) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            )
                -> alloy_sol_types::Result<ECDSAStakeRegistryCalls>] = &[
                {
                    fn updateOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::updateOperators)
                    }
                    updateOperators
                },
                {
                    fn getLastCheckpointTotalWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointTotalWeightAtBlock,
                            )
                    }
                    getLastCheckpointTotalWeightAtBlock
                },
                {
                    fn isValidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <isValidSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::isValidSignature)
                    }
                    isValidSignature
                },
                {
                    fn quorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <quorumCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryCalls::quorum)
                    }
                    quorum
                },
                {
                    fn getLastCheckpointThresholdWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointThresholdWeightAtBlock,
                            )
                    }
                    getLastCheckpointThresholdWeightAtBlock
                },
                {
                    fn getLastCheckpointTotalWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getLastCheckpointTotalWeight)
                    }
                    getLastCheckpointTotalWeight
                },
                {
                    fn getLastCheckpointOperatorWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointOperatorWeight,
                            )
                    }
                    getLastCheckpointOperatorWeight
                },
                {
                    fn registerOperatorWithSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::registerOperatorWithSignature)
                    }
                    registerOperatorWithSignature
                },
                {
                    fn minimumWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <minimumWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::minimumWeight)
                    }
                    minimumWeight
                },
                {
                    fn updateOperatorsForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::updateOperatorsForQuorum)
                    }
                    updateOperatorsForQuorum
                },
                {
                    fn getOperatorSigningKeyAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getOperatorSigningKeyAtBlock)
                    }
                    getOperatorSigningKeyAtBlock
                },
                {
                    fn updateStakeThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::updateStakeThreshold)
                    }
                    updateStakeThreshold
                },
                {
                    fn updateMinimumWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::updateMinimumWeight)
                    }
                    updateMinimumWeight
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn updateOperatorSigningKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::updateOperatorSigningKey)
                    }
                    updateOperatorSigningKey
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryCalls::owner)
                    }
                    owner
                },
                {
                    fn getOperatorWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::getOperatorWeightAtBlock)
                    }
                    getOperatorWeightAtBlock
                },
                {
                    fn getOperatorWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::getOperatorWeight)
                    }
                    getOperatorWeight
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getLastCheckpointThresholdWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointThresholdWeight,
                            )
                    }
                    getLastCheckpointThresholdWeight
                },
                {
                    fn getLastestOperatorSigningKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getLastestOperatorSigningKey)
                    }
                    getLastestOperatorSigningKey
                },
                {
                    fn updateQuorumConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::updateQuorumConfig)
                    }
                    updateQuorumConfig
                },
                {
                    fn operatorRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::operatorRegistered)
                    }
                    operatorRegistered
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryCalls::transferOwnership)
                    }
                    transferOwnership
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointOperatorWeight(inner) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointThresholdWeight(inner) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointThresholdWeightAtBlock(inner) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointTotalWeight(inner) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointTotalWeightAtBlock(inner) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastestOperatorSigningKey(inner) => {
                    <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSigningKeyAtBlock(inner) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorWeight(inner) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorWeightAtBlock(inner) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::minimumWeight(inner) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorRegistered(inner) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::quorum(inner) => {
                    <quorumCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerOperatorWithSignature(inner) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateMinimumWeight(inner) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorSigningKey(inner) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateQuorumConfig(inner) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateStakeThreshold(inner) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointOperatorWeight(inner) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointThresholdWeight(inner) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointThresholdWeightAtBlock(inner) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointTotalWeight(inner) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointTotalWeightAtBlock(inner) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastestOperatorSigningKey(inner) => {
                    <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorSigningKeyAtBlock(inner) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorWeight(inner) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorWeightAtBlock(inner) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minimumWeight(inner) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorRegistered(inner) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::quorum(inner) => {
                    <quorumCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerOperatorWithSignature(inner) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateMinimumWeight(inner) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorSigningKey(inner) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateQuorumConfig(inner) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateStakeThreshold(inner) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ECDSAStakeRegistry`](self) custom errors.
    pub enum ECDSAStakeRegistryErrors {
        InsufficientSignedStake(InsufficientSignedStake),
        InsufficientWeight(InsufficientWeight),
        InvalidLength(InvalidLength),
        InvalidQuorum(InvalidQuorum),
        InvalidReferenceBlock(InvalidReferenceBlock),
        InvalidSignature(InvalidSignature),
        InvalidSignedWeight(InvalidSignedWeight),
        InvalidThreshold(InvalidThreshold),
        LengthMismatch(LengthMismatch),
        MustUpdateAllOperators(MustUpdateAllOperators),
        NotSorted(NotSorted),
        OperatorAlreadyRegistered(OperatorAlreadyRegistered),
        OperatorNotRegistered(OperatorNotRegistered),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [37u8, 236u8, 108u8, 31u8],
            [45u8, 61u8, 246u8, 182u8],
            [66u8, 238u8, 104u8, 181u8],
            [139u8, 170u8, 87u8, 159u8],
            [148u8, 125u8, 90u8, 132u8],
            [150u8, 11u8, 65u8, 238u8],
            [168u8, 121u8, 47u8, 209u8],
            [170u8, 189u8, 90u8, 9u8],
            [186u8, 80u8, 249u8, 17u8],
            [209u8, 115u8, 87u8, 121u8],
            [225u8, 33u8, 99u8, 47u8],
            [230u8, 79u8, 24u8, 15u8],
            [255u8, 99u8, 58u8, 56u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ECDSAStakeRegistryErrors {
        const NAME: &'static str = "ECDSAStakeRegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InsufficientSignedStake(_) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientWeight(_) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidLength(_) => <InvalidLength as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidQuorum(_) => <InvalidQuorum as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidReferenceBlock(_) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignedWeight(_) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidThreshold(_) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LengthMismatch(_) => <LengthMismatch as alloy_sol_types::SolError>::SELECTOR,
                Self::MustUpdateAllOperators(_) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotSorted(_) => <NotSorted as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorAlreadyRegistered(_) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            )
                -> alloy_sol_types::Result<ECDSAStakeRegistryErrors>] = &[
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn MustUpdateAllOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::MustUpdateAllOperators)
                    }
                    MustUpdateAllOperators
                },
                {
                    fn OperatorAlreadyRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::OperatorAlreadyRegistered)
                    }
                    OperatorAlreadyRegistered
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn InvalidLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidLength as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryErrors::InvalidLength)
                    }
                    InvalidLength
                },
                {
                    fn InvalidSignedWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidSignedWeight as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::InvalidSignedWeight)
                    }
                    InvalidSignedWeight
                },
                {
                    fn InsufficientWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InsufficientWeight as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::InsufficientWeight)
                    }
                    InsufficientWeight
                },
                {
                    fn InvalidThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::InvalidThreshold)
                    }
                    InvalidThreshold
                },
                {
                    fn NotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <NotSorted as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryErrors::NotSorted)
                    }
                    NotSorted
                },
                {
                    fn InvalidQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidQuorum as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(ECDSAStakeRegistryErrors::InvalidQuorum)
                    }
                    InvalidQuorum
                },
                {
                    fn InsufficientSignedStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InsufficientSignedStake as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::InsufficientSignedStake)
                    }
                    InsufficientSignedStake
                },
                {
                    fn InvalidReferenceBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::InvalidReferenceBlock)
                    }
                    InvalidReferenceBlock
                },
                {
                    fn LengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <LengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(ECDSAStakeRegistryErrors::LengthMismatch)
                    }
                    LengthMismatch
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidReferenceBlock(inner) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidReferenceBlock(inner) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`ECDSAStakeRegistry`](self) events.
    pub enum ECDSAStakeRegistryEvents {
        Initialized(Initialized),
        MinimumWeightUpdated(MinimumWeightUpdated),
        OperatorDeregistered(OperatorDeregistered),
        OperatorRegistered(OperatorRegistered),
        OperatorWeightUpdated(OperatorWeightUpdated),
        OwnershipTransferred(OwnershipTransferred),
        QuorumUpdated(QuorumUpdated),
        SigningKeyUpdate(SigningKeyUpdate),
        ThresholdWeightUpdated(ThresholdWeightUpdated),
        TotalWeightUpdated(TotalWeightUpdated),
        UpdateMinimumWeight(UpdateMinimumWeight),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                30u8, 164u8, 33u8, 134u8, 179u8, 5u8, 250u8, 55u8, 49u8, 4u8, 80u8, 217u8, 251u8,
                135u8, 234u8, 30u8, 143u8, 12u8, 127u8, 68u8, 126u8, 119u8, 20u8, 121u8, 227u8,
                178u8, 118u8, 52u8, 191u8, 232u8, 77u8, 193u8,
            ],
            [
                35u8, 170u8, 212u8, 230u8, 23u8, 68u8, 236u8, 225u8, 100u8, 19u8, 10u8, 164u8,
                21u8, 193u8, 97u8, 110u8, 128u8, 19u8, 107u8, 15u8, 7u8, 112u8, 229u8, 101u8,
                137u8, 67u8, 139u8, 144u8, 178u8, 105u8, 38u8, 94u8,
            ],
            [
                49u8, 224u8, 173u8, 254u8, 199u8, 27u8, 204u8, 238u8, 55u8, 182u8, 232u8, 58u8,
                144u8, 194u8, 254u8, 219u8, 23u8, 216u8, 241u8, 105u8, 63u8, 238u8, 134u8, 60u8,
                71u8, 113u8, 231u8, 191u8, 226u8, 174u8, 213u8, 128u8,
            ],
            [
                113u8, 60u8, 165u8, 59u8, 136u8, 214u8, 235u8, 99u8, 245u8, 177u8, 133u8, 76u8,
                184u8, 203u8, 221u8, 115u8, 110u8, 197u8, 30u8, 218u8, 34u8, 94u8, 70u8, 121u8,
                26u8, 169u8, 41u8, 139u8, 1u8, 96u8, 100u8, 143u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                134u8, 220u8, 248u8, 107u8, 18u8, 223u8, 238u8, 222u8, 167u8, 74u8, 233u8, 48u8,
                13u8, 189u8, 170u8, 25u8, 59u8, 204u8, 229u8, 128u8, 147u8, 105u8, 200u8, 23u8,
                126u8, 162u8, 244u8, 234u8, 170u8, 101u8, 114u8, 155u8,
            ],
            [
                136u8, 119u8, 13u8, 200u8, 98u8, 228u8, 122u8, 126u8, 213u8, 134u8, 144u8, 120u8,
                87u8, 235u8, 27u8, 117u8, 228u8, 197u8, 255u8, 200u8, 183u8, 7u8, 199u8, 238u8,
                16u8, 235u8, 116u8, 214u8, 136u8, 95u8, 229u8, 148u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                147u8, 36u8, 247u8, 229u8, 167u8, 192u8, 40u8, 136u8, 8u8, 166u8, 52u8, 204u8,
                222u8, 68u8, 184u8, 233u8, 121u8, 103u8, 100u8, 116u8, 178u8, 46u8, 41u8, 238u8,
                157u8, 213u8, 105u8, 181u8, 94u8, 121u8, 26u8, 75u8,
            ],
            [
                164u8, 83u8, 219u8, 97u8, 42u8, 245u8, 158u8, 85u8, 33u8, 214u8, 171u8, 146u8,
                132u8, 220u8, 62u8, 45u8, 6u8, 175u8, 40u8, 110u8, 177u8, 177u8, 183u8, 183u8,
                113u8, 252u8, 228u8, 113u8, 108u8, 25u8, 242u8, 193u8,
            ],
            [
                208u8, 97u8, 22u8, 130u8, 82u8, 244u8, 65u8, 115u8, 54u8, 88u8, 240u8, 158u8, 77u8,
                143u8, 91u8, 45u8, 153u8, 142u8, 212u8, 239u8, 36u8, 162u8, 187u8, 253u8, 108u8,
                236u8, 165u8, 46u8, 161u8, 49u8, 80u8, 2u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ECDSAStakeRegistryEvents {
        const NAME: &'static str = "ECDSAStakeRegistryEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<MinimumWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MinimumWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::MinimumWeightUpdated)
                }
                Some(<OperatorDeregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorDeregistered)
                }
                Some(<OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorRegistered)
                }
                Some(<OperatorWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorWeightUpdated)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<QuorumUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumUpdated)
                }
                Some(<SigningKeyUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SigningKeyUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::SigningKeyUpdate)
                }
                Some(<ThresholdWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ThresholdWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ThresholdWeightUpdated)
                }
                Some(<TotalWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TotalWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TotalWeightUpdated)
                }
                Some(<UpdateMinimumWeight as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <UpdateMinimumWeight as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::UpdateMinimumWeight)
                }
                _ => alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                    name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                    log: alloy_sol_types::private::Box::new(
                        alloy_sol_types::private::LogData::new_unchecked(
                            topics.to_vec(),
                            data.to_vec().into(),
                        ),
                    ),
                }),
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for ECDSAStakeRegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MinimumWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SigningKeyUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ThresholdWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TotalWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateMinimumWeight(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MinimumWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SigningKeyUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ThresholdWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TotalWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateMinimumWeight(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ECDSAStakeRegistry`](self) contract instance.

    See the [wrapper's documentation](`ECDSAStakeRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ECDSAStakeRegistryInstance<T, P, N> {
        ECDSAStakeRegistryInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _delegationManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<ECDSAStakeRegistryInstance<T, P, N>>>
    {
        ECDSAStakeRegistryInstance::<T, P, N>::deploy(provider, _delegationManager)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
    and constructor arguments, if any.

    This is a simple wrapper around creating a `RawCallBuilder` with the data set to
    the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _delegationManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        ECDSAStakeRegistryInstance::<T, P, N>::deploy_builder(provider, _delegationManager)
    }
    /**A [`ECDSAStakeRegistry`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ECDSAStakeRegistry`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ECDSAStakeRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ECDSAStakeRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ECDSAStakeRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ECDSAStakeRegistryInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ECDSAStakeRegistry`](self) contract instance.

        See the [wrapper's documentation](`ECDSAStakeRegistryInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self { address, provider, _network_transport: ::core::marker::PhantomData }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<ECDSAStakeRegistryInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _delegationManager);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _delegationManager,
                    })[..],
                ]
                .concat()
                .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ECDSAStakeRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ECDSAStakeRegistryInstance<T, P, N> {
            ECDSAStakeRegistryInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ECDSAStakeRegistryInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(&deregisterOperatorCall {})
        }
        ///Creates a new call builder for the [`getLastCheckpointOperatorWeight`] function.
        pub fn getLastCheckpointOperatorWeight(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointOperatorWeightCall, N> {
            self.call_builder(&getLastCheckpointOperatorWeightCall { _operator })
        }
        ///Creates a new call builder for the [`getLastCheckpointThresholdWeight`] function.
        pub fn getLastCheckpointThresholdWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointThresholdWeightCall, N>
        {
            self.call_builder(&getLastCheckpointThresholdWeightCall {})
        }
        ///Creates a new call builder for the [`getLastCheckpointThresholdWeightAtBlock`] function.
        pub fn getLastCheckpointThresholdWeightAtBlock(
            &self,
            _blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointThresholdWeightAtBlockCall, N>
        {
            self.call_builder(&getLastCheckpointThresholdWeightAtBlockCall { _blockNumber })
        }
        ///Creates a new call builder for the [`getLastCheckpointTotalWeight`] function.
        pub fn getLastCheckpointTotalWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointTotalWeightCall, N> {
            self.call_builder(&getLastCheckpointTotalWeightCall {})
        }
        ///Creates a new call builder for the [`getLastCheckpointTotalWeightAtBlock`] function.
        pub fn getLastCheckpointTotalWeightAtBlock(
            &self,
            _blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointTotalWeightAtBlockCall, N>
        {
            self.call_builder(&getLastCheckpointTotalWeightAtBlockCall { _blockNumber })
        }
        ///Creates a new call builder for the [`getLastestOperatorSigningKey`] function.
        pub fn getLastestOperatorSigningKey(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastestOperatorSigningKeyCall, N> {
            self.call_builder(&getLastestOperatorSigningKeyCall { _operator })
        }
        ///Creates a new call builder for the [`getOperatorSigningKeyAtBlock`] function.
        pub fn getOperatorSigningKeyAtBlock(
            &self,
            _operator: alloy::sol_types::private::Address,
            _blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSigningKeyAtBlockCall, N> {
            self.call_builder(&getOperatorSigningKeyAtBlockCall { _operator, _blockNumber })
        }
        ///Creates a new call builder for the [`getOperatorWeight`] function.
        pub fn getOperatorWeight(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorWeightCall, N> {
            self.call_builder(&getOperatorWeightCall { _operator })
        }
        ///Creates a new call builder for the [`getOperatorWeightAtBlock`] function.
        pub fn getOperatorWeightAtBlock(
            &self,
            _operator: alloy::sol_types::private::Address,
            _blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorWeightAtBlockCall, N> {
            self.call_builder(&getOperatorWeightAtBlockCall { _operator, _blockNumber })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _serviceManager: alloy::sol_types::private::Address,
            _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
            _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall { _serviceManager, _thresholdWeight, _quorum })
        }
        ///Creates a new call builder for the [`isValidSignature`] function.
        pub fn isValidSignature(
            &self,
            _dataHash: alloy::sol_types::private::FixedBytes<32>,
            _signatureData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidSignatureCall, N> {
            self.call_builder(&isValidSignatureCall { _dataHash, _signatureData })
        }
        ///Creates a new call builder for the [`minimumWeight`] function.
        pub fn minimumWeight(&self) -> alloy_contract::SolCallBuilder<T, &P, minimumWeightCall, N> {
            self.call_builder(&minimumWeightCall {})
        }
        ///Creates a new call builder for the [`operatorRegistered`] function.
        pub fn operatorRegistered(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorRegisteredCall, N> {
            self.call_builder(&operatorRegisteredCall { _operator })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`quorum`] function.
        pub fn quorum(&self) -> alloy_contract::SolCallBuilder<T, &P, quorumCall, N> {
            self.call_builder(&quorumCall {})
        }
        ///Creates a new call builder for the [`registerOperatorWithSignature`] function.
        pub fn registerOperatorWithSignature(
            &self,
            _operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
            _signingKey: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorWithSignatureCall, N> {
            self.call_builder(&registerOperatorWithSignatureCall {
                _operatorSignature,
                _signingKey,
            })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`updateMinimumWeight`] function.
        pub fn updateMinimumWeight(
            &self,
            _newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
            _operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateMinimumWeightCall, N> {
            self.call_builder(&updateMinimumWeightCall { _newMinimumWeight, _operators })
        }
        ///Creates a new call builder for the [`updateOperatorSigningKey`] function.
        pub fn updateOperatorSigningKey(
            &self,
            _newSigningKey: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorSigningKeyCall, N> {
            self.call_builder(&updateOperatorSigningKeyCall { _newSigningKey })
        }
        ///Creates a new call builder for the [`updateOperators`] function.
        pub fn updateOperators(
            &self,
            _operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsCall, N> {
            self.call_builder(&updateOperatorsCall { _operators })
        }
        ///Creates a new call builder for the [`updateOperatorsForQuorum`] function.
        pub fn updateOperatorsForQuorum(
            &self,
            operatorsPerQuorum: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsForQuorumCall, N> {
            self.call_builder(&updateOperatorsForQuorumCall { operatorsPerQuorum, _1 })
        }
        ///Creates a new call builder for the [`updateQuorumConfig`] function.
        pub fn updateQuorumConfig(
            &self,
            _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
            _operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateQuorumConfigCall, N> {
            self.call_builder(&updateQuorumConfigCall { _quorum, _operators })
        }
        ///Creates a new call builder for the [`updateStakeThreshold`] function.
        pub fn updateStakeThreshold(
            &self,
            _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateStakeThresholdCall, N> {
            self.call_builder(&updateStakeThresholdCall { _thresholdWeight })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ECDSAStakeRegistryInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`MinimumWeightUpdated`] event.
        pub fn MinimumWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MinimumWeightUpdated, N> {
            self.event_filter::<MinimumWeightUpdated>()
        }
        ///Creates a new event filter for the [`OperatorDeregistered`] event.
        pub fn OperatorDeregistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorDeregistered, N> {
            self.event_filter::<OperatorDeregistered>()
        }
        ///Creates a new event filter for the [`OperatorRegistered`] event.
        pub fn OperatorRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRegistered, N> {
            self.event_filter::<OperatorRegistered>()
        }
        ///Creates a new event filter for the [`OperatorWeightUpdated`] event.
        pub fn OperatorWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorWeightUpdated, N> {
            self.event_filter::<OperatorWeightUpdated>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`QuorumUpdated`] event.
        pub fn QuorumUpdated_filter(&self) -> alloy_contract::Event<T, &P, QuorumUpdated, N> {
            self.event_filter::<QuorumUpdated>()
        }
        ///Creates a new event filter for the [`SigningKeyUpdate`] event.
        pub fn SigningKeyUpdate_filter(&self) -> alloy_contract::Event<T, &P, SigningKeyUpdate, N> {
            self.event_filter::<SigningKeyUpdate>()
        }
        ///Creates a new event filter for the [`ThresholdWeightUpdated`] event.
        pub fn ThresholdWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ThresholdWeightUpdated, N> {
            self.event_filter::<ThresholdWeightUpdated>()
        }
        ///Creates a new event filter for the [`TotalWeightUpdated`] event.
        pub fn TotalWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TotalWeightUpdated, N> {
            self.event_filter::<TotalWeightUpdated>()
        }
        ///Creates a new event filter for the [`UpdateMinimumWeight`] event.
        pub fn UpdateMinimumWeight_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UpdateMinimumWeight, N> {
            self.event_filter::<UpdateMinimumWeight>()
        }
    }
}
