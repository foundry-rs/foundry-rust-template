///Module containing a contract's types and functions.
/**

```solidity
library ILayerTrigger {
    type TriggerId is uint64;
    struct TriggerResponse { TriggerId triggerId; string serviceId; string workflowId; address creator; bytes data; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ILayerTrigger {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TriggerId(u64);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<TriggerId> for u64 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<64>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl TriggerId {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u64) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u64 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for TriggerId {
            type RustType = u64;
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::type_check(
                    token,
                )
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::detokenize(
                    token,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TriggerId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    /**```solidity
    struct TriggerResponse { TriggerId triggerId; string serviceId; string workflowId; address creator; bytes data; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TriggerResponse {
        pub triggerId: <TriggerId as alloy::sol_types::SolType>::RustType,
        pub serviceId: alloy::sol_types::private::String,
        pub workflowId: alloy::sol_types::private::String,
        pub creator: alloy::sol_types::private::Address,
        pub data: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            TriggerId,
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <TriggerId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::String,
            alloy::sol_types::private::String,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<TriggerResponse> for UnderlyingRustTuple<'_> {
            fn from(value: TriggerResponse) -> Self {
                (value.triggerId, value.serviceId, value.workflowId, value.creator, value.data)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TriggerResponse {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    triggerId: tuple.0,
                    serviceId: tuple.1,
                    workflowId: tuple.2,
                    creator: tuple.3,
                    data: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TriggerResponse {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TriggerResponse {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <TriggerId as alloy_sol_types::SolType>::tokenize(&self.triggerId),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.serviceId,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.workflowId,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.creator,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
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
        impl alloy_sol_types::SolType for TriggerResponse {
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
        impl alloy_sol_types::SolStruct for TriggerResponse {
            const NAME: &'static str = "TriggerResponse";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TriggerResponse(uint64 triggerId,string serviceId,string workflowId,address creator,bytes data)",
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
                    <TriggerId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.triggerId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.serviceId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.workflowId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.creator,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TriggerResponse {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <TriggerId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.triggerId,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.serviceId,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.workflowId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.creator,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <TriggerId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.triggerId,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.serviceId,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.workflowId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.creator,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
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
    /**Creates a new wrapper around an on-chain [`ILayerTrigger`](self) contract instance.

    See the [wrapper's documentation](`ILayerTriggerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ILayerTriggerInstance<T, P, N> {
        ILayerTriggerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ILayerTrigger`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ILayerTrigger`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ILayerTriggerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ILayerTriggerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ILayerTriggerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ILayerTriggerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ILayerTrigger`](self) contract instance.

        See the [wrapper's documentation](`ILayerTriggerInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ILayerTriggerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ILayerTriggerInstance<T, P, N> {
            ILayerTriggerInstance {
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
        > ILayerTriggerInstance<T, P, N>
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
        > ILayerTriggerInstance<T, P, N>
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
library ILayerTrigger {
    type TriggerId is uint64;
    struct TriggerResponse {
        TriggerId triggerId;
        string serviceId;
        string workflowId;
        address creator;
        bytes data;
    }
}

interface WavsTrigger {
    event NewTrigger(string serviceId, string workflowId, ILayerTrigger.TriggerId indexed triggerId);

    function addTrigger(string memory serviceId, string memory workflowId, bytes memory data) external;
    function getTrigger(ILayerTrigger.TriggerId triggerId) external view returns (ILayerTrigger.TriggerResponse memory);
    function nextTriggerId() external view returns (ILayerTrigger.TriggerId);
    function triggerIdsByCreator(address, uint256) external view returns (ILayerTrigger.TriggerId);
    function triggersById(ILayerTrigger.TriggerId) external view returns (string memory serviceId, string memory workflowId, address creator, bytes memory data);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "addTrigger",
    "inputs": [
      {
        "name": "serviceId",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "workflowId",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getTrigger",
    "inputs": [
      {
        "name": "triggerId",
        "type": "uint64",
        "internalType": "ILayerTrigger.TriggerId"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct ILayerTrigger.TriggerResponse",
        "components": [
          {
            "name": "triggerId",
            "type": "uint64",
            "internalType": "ILayerTrigger.TriggerId"
          },
          {
            "name": "serviceId",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "workflowId",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "creator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nextTriggerId",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "ILayerTrigger.TriggerId"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "triggerIdsByCreator",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "ILayerTrigger.TriggerId"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "triggersById",
    "inputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "ILayerTrigger.TriggerId"
      }
    ],
    "outputs": [
      {
        "name": "serviceId",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "workflowId",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "creator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "NewTrigger",
    "inputs": [
      {
        "name": "serviceId",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "workflowId",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "triggerId",
        "type": "uint64",
        "indexed": true,
        "internalType": "ILayerTrigger.TriggerId"
      }
    ],
    "anonymous": false
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
pub mod WavsTrigger {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f80fd5b50610afc8061001c5f395ff3fe608060405234801561000f575f80fd5b5060043610610055575f3560e01c806330628cc81461005957806342227fa41461006e578063913b1fbf1461009e578063ce289612146100b1578063e328ed77146100d4575b5f80fd5b61006c610067366004610721565b6100f4565b005b600254610081906001600160401b031681565b6040516001600160401b0390911681526020015b60405180910390f35b6100816100ac3660046107bf565b610247565b6100c46100bf3660046107f4565b61028e565b604051610095949392919061085d565b6100e76100e23660046107f4565b610456565b60405161009591906108af565b60025461010b906001600160401b03166001610933565b6002805467ffffffffffffffff19166001600160401b039290921691821790556040805160808101825285815260208082018690523382840152606082018590525f84815290819052919091208151829190819061016990826109e8565b506020820151600182019061017e90826109e8565b5060408201516002820180546001600160a01b0319166001600160a01b03909216919091179055606082015160038201906101b990826109e8565b5050335f9081526001602081815260408084208054938401815584529220600482040180546001600160401b0380881660086003909516949094026101000a84810291021990911617905590519091507f8a81be05064e2aef2ff874e33696c0ee4c4177c775d7fd263bdb2bd666ffffd9906102389088908890610aa2565b60405180910390a25050505050565b6001602052815f5260405f208181548110610260575f80fd5b905f5260205f209060049182820401919006600802915091509054906101000a90046001600160401b031681565b5f602081905290815260409020805481906102a890610964565b80601f01602080910402602001604051908101604052809291908181526020018280546102d490610964565b801561031f5780601f106102f65761010080835404028352916020019161031f565b820191905f5260205f20905b81548152906001019060200180831161030257829003601f168201915b50505050509080600101805461033490610964565b80601f016020809104026020016040519081016040528092919081815260200182805461036090610964565b80156103ab5780601f10610382576101008083540402835291602001916103ab565b820191905f5260205f20905b81548152906001019060200180831161038e57829003601f168201915b505050600284015460038501805494956001600160a01b039092169491935091506103d590610964565b80601f016020809104026020016040519081016040528092919081815260200182805461040190610964565b801561044c5780601f106104235761010080835404028352916020019161044c565b820191905f5260205f20905b81548152906001019060200180831161042f57829003601f168201915b5050505050905084565b6040805160a0810182525f8082526060602083018190529282018390528282015260808101919091526001600160401b0382165f8181526020818152604091829020825160a0810190935292825282549082019083906104b590610964565b80601f01602080910402602001604051908101604052809291908181526020018280546104e190610964565b801561052c5780601f106105035761010080835404028352916020019161052c565b820191905f5260205f20905b81548152906001019060200180831161050f57829003601f168201915b5050505050815260200182600101805461054590610964565b80601f016020809104026020016040519081016040528092919081815260200182805461057190610964565b80156105bc5780601f10610593576101008083540402835291602001916105bc565b820191905f5260205f20905b81548152906001019060200180831161059f57829003601f168201915b505050918352505060028301546001600160a01b031660208201526003830180546040909201916105ec90610964565b80601f016020809104026020016040519081016040528092919081815260200182805461061890610964565b80156106635780601f1061063a57610100808354040283529160200191610663565b820191905f5260205f20905b81548152906001019060200180831161064657829003601f168201915b5050505050815250915050919050565b634e487b7160e01b5f52604160045260245ffd5b5f806001600160401b038411156106a0576106a0610673565b50604051601f19601f85018116603f011681018181106001600160401b03821117156106ce576106ce610673565b6040528381529050808284018510156106e5575f80fd5b838360208301375f60208583010152509392505050565b5f82601f83011261070b575f80fd5b61071a83833560208501610687565b9392505050565b5f805f60608486031215610733575f80fd5b83356001600160401b03811115610748575f80fd5b610754868287016106fc565b93505060208401356001600160401b0381111561076f575f80fd5b61077b868287016106fc565b92505060408401356001600160401b03811115610796575f80fd5b8401601f810186136107a6575f80fd5b6107b586823560208401610687565b9150509250925092565b5f80604083850312156107d0575f80fd5b82356001600160a01b03811681146107e6575f80fd5b946020939093013593505050565b5f60208284031215610804575f80fd5b81356001600160401b038116811461071a575f80fd5b5f81518084525f5b8181101561083e57602081850181015186830182015201610822565b505f602082860101526020601f19601f83011685010191505092915050565b608081525f61086f608083018761081a565b8281036020840152610881818761081a565b6001600160a01b0386166040850152838103606085015290506108a4818561081a565b979650505050505050565b602081526001600160401b0382511660208201525f602083015160a060408401526108dd60c084018261081a565b90506040840151601f198483030160608501526108fa828261081a565b91505060018060a01b0360608501511660808401526080840151601f198483030160a085015261092a828261081a565b95945050505050565b6001600160401b03818116838216019081111561095e57634e487b7160e01b5f52601160045260245ffd5b92915050565b600181811c9082168061097857607f821691505b60208210810361099657634e487b7160e01b5f52602260045260245ffd5b50919050565b601f8211156109e357805f5260205f20601f840160051c810160208510156109c15750805b601f840160051c820191505b818110156109e0575f81556001016109cd565b50505b505050565b81516001600160401b03811115610a0157610a01610673565b610a1581610a0f8454610964565b8461099c565b6020601f821160018114610a47575f8315610a305750848201515b5f19600385901b1c1916600184901b1784556109e0565b5f84815260208120601f198516915b82811015610a765787850151825560209485019460019092019101610a56565b5084821015610a9357868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b604081525f610ab4604083018561081a565b828103602084015261092a818561081a56fea26469706673582212209aa02af4b9f720973037ec495f70231761f27d4b2d2fdbd66f1eaf9d4928e39964736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW_\x80\xFD[Pa\n\xFC\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80c0b\x8C\xC8\x14a\0YW\x80cB\"\x7F\xA4\x14a\0nW\x80c\x91;\x1F\xBF\x14a\0\x9EW\x80c\xCE(\x96\x12\x14a\0\xB1W\x80c\xE3(\xEDw\x14a\0\xD4W[_\x80\xFD[a\0la\0g6`\x04a\x07!V[a\0\xF4V[\0[`\x02Ta\0\x81\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x81a\0\xAC6`\x04a\x07\xBFV[a\x02GV[a\0\xC4a\0\xBF6`\x04a\x07\xF4V[a\x02\x8EV[`@Qa\0\x95\x94\x93\x92\x91\x90a\x08]V[a\0\xE7a\0\xE26`\x04a\x07\xF4V[a\x04VV[`@Qa\0\x95\x91\x90a\x08\xAFV[`\x02Ta\x01\x0B\x90`\x01`\x01`@\x1B\x03\x16`\x01a\t3V[`\x02\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@\x80Q`\x80\x81\x01\x82R\x85\x81R` \x80\x82\x01\x86\x90R3\x82\x84\x01R``\x82\x01\x85\x90R_\x84\x81R\x90\x81\x90R\x91\x90\x91 \x81Q\x82\x91\x90\x81\x90a\x01i\x90\x82a\t\xE8V[P` \x82\x01Q`\x01\x82\x01\x90a\x01~\x90\x82a\t\xE8V[P`@\x82\x01Q`\x02\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U``\x82\x01Q`\x03\x82\x01\x90a\x01\xB9\x90\x82a\t\xE8V[PP3_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x80T\x93\x84\x01\x81U\x84R\x92 `\x04\x82\x04\x01\x80T`\x01`\x01`@\x1B\x03\x80\x88\x16`\x08`\x03\x90\x95\x16\x94\x90\x94\x02a\x01\0\n\x84\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U\x90Q\x90\x91P\x7F\x8A\x81\xBE\x05\x06N*\xEF/\xF8t\xE36\x96\xC0\xEELAw\xC7u\xD7\xFD&;\xDB+\xD6f\xFF\xFF\xD9\x90a\x028\x90\x88\x90\x88\x90a\n\xA2V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\x01` R\x81_R`@_ \x81\x81T\x81\x10a\x02`W_\x80\xFD[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[_` \x81\x90R\x90\x81R`@\x90 \x80T\x81\x90a\x02\xA8\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xD4\x90a\tdV[\x80\x15a\x03\x1FW\x80`\x1F\x10a\x02\xF6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x1FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x02W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x034\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03`\x90a\tdV[\x80\x15a\x03\xABW\x80`\x1F\x10a\x03\x82Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xABV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x8EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP`\x02\x84\x01T`\x03\x85\x01\x80T\x94\x95`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x94\x91\x93P\x91Pa\x03\xD5\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\x01\x90a\tdV[\x80\x15a\x04LW\x80`\x1F\x10a\x04#Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04LV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04/W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x84V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R``` \x83\x01\x81\x90R\x92\x82\x01\x83\x90R\x82\x82\x01R`\x80\x81\x01\x91\x90\x91R`\x01`\x01`@\x1B\x03\x82\x16_\x81\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x90\x93R\x92\x82R\x82T\x90\x82\x01\x90\x83\x90a\x04\xB5\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xE1\x90a\tdV[\x80\x15a\x05,W\x80`\x1F\x10a\x05\x03Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05,V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x0FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x82`\x01\x01\x80Ta\x05E\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05q\x90a\tdV[\x80\x15a\x05\xBCW\x80`\x1F\x10a\x05\x93Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xBCV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x9FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x02\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`\x03\x83\x01\x80T`@\x90\x92\x01\x91a\x05\xEC\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x18\x90a\tdV[\x80\x15a\x06cW\x80`\x1F\x10a\x06:Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06cV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RP\x91PP\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a\x06\xA0Wa\x06\xA0a\x06sV[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x06\xCEWa\x06\xCEa\x06sV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x06\xE5W_\x80\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x07\x0BW_\x80\xFD[a\x07\x1A\x83\x835` \x85\x01a\x06\x87V[\x93\x92PPPV[_\x80_``\x84\x86\x03\x12\x15a\x073W_\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07HW_\x80\xFD[a\x07T\x86\x82\x87\x01a\x06\xFCV[\x93PP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07oW_\x80\xFD[a\x07{\x86\x82\x87\x01a\x06\xFCV[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x96W_\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x07\xA6W_\x80\xFD[a\x07\xB5\x86\x825` \x84\x01a\x06\x87V[\x91PP\x92P\x92P\x92V[_\x80`@\x83\x85\x03\x12\x15a\x07\xD0W_\x80\xFD[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xE6W_\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a\x08\x04W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x07\x1AW_\x80\xFD[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x08>W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x08\"V[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x80\x81R_a\x08o`\x80\x83\x01\x87a\x08\x1AV[\x82\x81\x03` \x84\x01Ra\x08\x81\x81\x87a\x08\x1AV[`\x01`\x01`\xA0\x1B\x03\x86\x16`@\x85\x01R\x83\x81\x03``\x85\x01R\x90Pa\x08\xA4\x81\x85a\x08\x1AV[\x97\x96PPPPPPPV[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R_` \x83\x01Q`\xA0`@\x84\x01Ra\x08\xDD`\xC0\x84\x01\x82a\x08\x1AV[\x90P`@\x84\x01Q`\x1F\x19\x84\x83\x03\x01``\x85\x01Ra\x08\xFA\x82\x82a\x08\x1AV[\x91PP`\x01\x80`\xA0\x1B\x03``\x85\x01Q\x16`\x80\x84\x01R`\x80\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\xA0\x85\x01Ra\t*\x82\x82a\x08\x1AV[\x95\x94PPPPPV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\t^WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\txW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\t\x96WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\t\xE3W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\t\xC1WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\t\xE0W_\x81U`\x01\x01a\t\xCDV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x01Wa\n\x01a\x06sV[a\n\x15\x81a\n\x0F\x84Ta\tdV[\x84a\t\x9CV[` `\x1F\x82\x11`\x01\x81\x14a\nGW_\x83\x15a\n0WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\t\xE0V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\nvW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\nVV[P\x84\x82\x10\x15a\n\x93W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x81R_a\n\xB4`@\x83\x01\x85a\x08\x1AV[\x82\x81\x03` \x84\x01Ra\t*\x81\x85a\x08\x1AV\xFE\xA2dipfsX\"\x12 \x9A\xA0*\xF4\xB9\xF7 \x9707\xECI_p#\x17a\xF2}K-/\xDB\xD6o\x1E\xAF\x9DI(\xE3\x99dsolcC\0\x08\x1A\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610055575f3560e01c806330628cc81461005957806342227fa41461006e578063913b1fbf1461009e578063ce289612146100b1578063e328ed77146100d4575b5f80fd5b61006c610067366004610721565b6100f4565b005b600254610081906001600160401b031681565b6040516001600160401b0390911681526020015b60405180910390f35b6100816100ac3660046107bf565b610247565b6100c46100bf3660046107f4565b61028e565b604051610095949392919061085d565b6100e76100e23660046107f4565b610456565b60405161009591906108af565b60025461010b906001600160401b03166001610933565b6002805467ffffffffffffffff19166001600160401b039290921691821790556040805160808101825285815260208082018690523382840152606082018590525f84815290819052919091208151829190819061016990826109e8565b506020820151600182019061017e90826109e8565b5060408201516002820180546001600160a01b0319166001600160a01b03909216919091179055606082015160038201906101b990826109e8565b5050335f9081526001602081815260408084208054938401815584529220600482040180546001600160401b0380881660086003909516949094026101000a84810291021990911617905590519091507f8a81be05064e2aef2ff874e33696c0ee4c4177c775d7fd263bdb2bd666ffffd9906102389088908890610aa2565b60405180910390a25050505050565b6001602052815f5260405f208181548110610260575f80fd5b905f5260205f209060049182820401919006600802915091509054906101000a90046001600160401b031681565b5f602081905290815260409020805481906102a890610964565b80601f01602080910402602001604051908101604052809291908181526020018280546102d490610964565b801561031f5780601f106102f65761010080835404028352916020019161031f565b820191905f5260205f20905b81548152906001019060200180831161030257829003601f168201915b50505050509080600101805461033490610964565b80601f016020809104026020016040519081016040528092919081815260200182805461036090610964565b80156103ab5780601f10610382576101008083540402835291602001916103ab565b820191905f5260205f20905b81548152906001019060200180831161038e57829003601f168201915b505050600284015460038501805494956001600160a01b039092169491935091506103d590610964565b80601f016020809104026020016040519081016040528092919081815260200182805461040190610964565b801561044c5780601f106104235761010080835404028352916020019161044c565b820191905f5260205f20905b81548152906001019060200180831161042f57829003601f168201915b5050505050905084565b6040805160a0810182525f8082526060602083018190529282018390528282015260808101919091526001600160401b0382165f8181526020818152604091829020825160a0810190935292825282549082019083906104b590610964565b80601f01602080910402602001604051908101604052809291908181526020018280546104e190610964565b801561052c5780601f106105035761010080835404028352916020019161052c565b820191905f5260205f20905b81548152906001019060200180831161050f57829003601f168201915b5050505050815260200182600101805461054590610964565b80601f016020809104026020016040519081016040528092919081815260200182805461057190610964565b80156105bc5780601f10610593576101008083540402835291602001916105bc565b820191905f5260205f20905b81548152906001019060200180831161059f57829003601f168201915b505050918352505060028301546001600160a01b031660208201526003830180546040909201916105ec90610964565b80601f016020809104026020016040519081016040528092919081815260200182805461061890610964565b80156106635780601f1061063a57610100808354040283529160200191610663565b820191905f5260205f20905b81548152906001019060200180831161064657829003601f168201915b5050505050815250915050919050565b634e487b7160e01b5f52604160045260245ffd5b5f806001600160401b038411156106a0576106a0610673565b50604051601f19601f85018116603f011681018181106001600160401b03821117156106ce576106ce610673565b6040528381529050808284018510156106e5575f80fd5b838360208301375f60208583010152509392505050565b5f82601f83011261070b575f80fd5b61071a83833560208501610687565b9392505050565b5f805f60608486031215610733575f80fd5b83356001600160401b03811115610748575f80fd5b610754868287016106fc565b93505060208401356001600160401b0381111561076f575f80fd5b61077b868287016106fc565b92505060408401356001600160401b03811115610796575f80fd5b8401601f810186136107a6575f80fd5b6107b586823560208401610687565b9150509250925092565b5f80604083850312156107d0575f80fd5b82356001600160a01b03811681146107e6575f80fd5b946020939093013593505050565b5f60208284031215610804575f80fd5b81356001600160401b038116811461071a575f80fd5b5f81518084525f5b8181101561083e57602081850181015186830182015201610822565b505f602082860101526020601f19601f83011685010191505092915050565b608081525f61086f608083018761081a565b8281036020840152610881818761081a565b6001600160a01b0386166040850152838103606085015290506108a4818561081a565b979650505050505050565b602081526001600160401b0382511660208201525f602083015160a060408401526108dd60c084018261081a565b90506040840151601f198483030160608501526108fa828261081a565b91505060018060a01b0360608501511660808401526080840151601f198483030160a085015261092a828261081a565b95945050505050565b6001600160401b03818116838216019081111561095e57634e487b7160e01b5f52601160045260245ffd5b92915050565b600181811c9082168061097857607f821691505b60208210810361099657634e487b7160e01b5f52602260045260245ffd5b50919050565b601f8211156109e357805f5260205f20601f840160051c810160208510156109c15750805b601f840160051c820191505b818110156109e0575f81556001016109cd565b50505b505050565b81516001600160401b03811115610a0157610a01610673565b610a1581610a0f8454610964565b8461099c565b6020601f821160018114610a47575f8315610a305750848201515b5f19600385901b1c1916600184901b1784556109e0565b5f84815260208120601f198516915b82811015610a765787850151825560209485019460019092019101610a56565b5084821015610a9357868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b604081525f610ab4604083018561081a565b828103602084015261092a818561081a56fea26469706673582212209aa02af4b9f720973037ec495f70231761f27d4b2d2fdbd66f1eaf9d4928e39964736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80c0b\x8C\xC8\x14a\0YW\x80cB\"\x7F\xA4\x14a\0nW\x80c\x91;\x1F\xBF\x14a\0\x9EW\x80c\xCE(\x96\x12\x14a\0\xB1W\x80c\xE3(\xEDw\x14a\0\xD4W[_\x80\xFD[a\0la\0g6`\x04a\x07!V[a\0\xF4V[\0[`\x02Ta\0\x81\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x81a\0\xAC6`\x04a\x07\xBFV[a\x02GV[a\0\xC4a\0\xBF6`\x04a\x07\xF4V[a\x02\x8EV[`@Qa\0\x95\x94\x93\x92\x91\x90a\x08]V[a\0\xE7a\0\xE26`\x04a\x07\xF4V[a\x04VV[`@Qa\0\x95\x91\x90a\x08\xAFV[`\x02Ta\x01\x0B\x90`\x01`\x01`@\x1B\x03\x16`\x01a\t3V[`\x02\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@\x80Q`\x80\x81\x01\x82R\x85\x81R` \x80\x82\x01\x86\x90R3\x82\x84\x01R``\x82\x01\x85\x90R_\x84\x81R\x90\x81\x90R\x91\x90\x91 \x81Q\x82\x91\x90\x81\x90a\x01i\x90\x82a\t\xE8V[P` \x82\x01Q`\x01\x82\x01\x90a\x01~\x90\x82a\t\xE8V[P`@\x82\x01Q`\x02\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U``\x82\x01Q`\x03\x82\x01\x90a\x01\xB9\x90\x82a\t\xE8V[PP3_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x80T\x93\x84\x01\x81U\x84R\x92 `\x04\x82\x04\x01\x80T`\x01`\x01`@\x1B\x03\x80\x88\x16`\x08`\x03\x90\x95\x16\x94\x90\x94\x02a\x01\0\n\x84\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U\x90Q\x90\x91P\x7F\x8A\x81\xBE\x05\x06N*\xEF/\xF8t\xE36\x96\xC0\xEELAw\xC7u\xD7\xFD&;\xDB+\xD6f\xFF\xFF\xD9\x90a\x028\x90\x88\x90\x88\x90a\n\xA2V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\x01` R\x81_R`@_ \x81\x81T\x81\x10a\x02`W_\x80\xFD[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[_` \x81\x90R\x90\x81R`@\x90 \x80T\x81\x90a\x02\xA8\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xD4\x90a\tdV[\x80\x15a\x03\x1FW\x80`\x1F\x10a\x02\xF6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x1FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x02W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x034\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03`\x90a\tdV[\x80\x15a\x03\xABW\x80`\x1F\x10a\x03\x82Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\xABV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x8EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP`\x02\x84\x01T`\x03\x85\x01\x80T\x94\x95`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x94\x91\x93P\x91Pa\x03\xD5\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\x01\x90a\tdV[\x80\x15a\x04LW\x80`\x1F\x10a\x04#Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04LV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04/W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x84V[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R``` \x83\x01\x81\x90R\x92\x82\x01\x83\x90R\x82\x82\x01R`\x80\x81\x01\x91\x90\x91R`\x01`\x01`@\x1B\x03\x82\x16_\x81\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x90\x93R\x92\x82R\x82T\x90\x82\x01\x90\x83\x90a\x04\xB5\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xE1\x90a\tdV[\x80\x15a\x05,W\x80`\x1F\x10a\x05\x03Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05,V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x0FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x82`\x01\x01\x80Ta\x05E\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05q\x90a\tdV[\x80\x15a\x05\xBCW\x80`\x1F\x10a\x05\x93Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xBCV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x9FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x02\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`\x03\x83\x01\x80T`@\x90\x92\x01\x91a\x05\xEC\x90a\tdV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x18\x90a\tdV[\x80\x15a\x06cW\x80`\x1F\x10a\x06:Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06cV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RP\x91PP\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a\x06\xA0Wa\x06\xA0a\x06sV[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x06\xCEWa\x06\xCEa\x06sV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x06\xE5W_\x80\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x07\x0BW_\x80\xFD[a\x07\x1A\x83\x835` \x85\x01a\x06\x87V[\x93\x92PPPV[_\x80_``\x84\x86\x03\x12\x15a\x073W_\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07HW_\x80\xFD[a\x07T\x86\x82\x87\x01a\x06\xFCV[\x93PP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07oW_\x80\xFD[a\x07{\x86\x82\x87\x01a\x06\xFCV[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x96W_\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x07\xA6W_\x80\xFD[a\x07\xB5\x86\x825` \x84\x01a\x06\x87V[\x91PP\x92P\x92P\x92V[_\x80`@\x83\x85\x03\x12\x15a\x07\xD0W_\x80\xFD[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xE6W_\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a\x08\x04W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x07\x1AW_\x80\xFD[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x08>W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x08\"V[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x80\x81R_a\x08o`\x80\x83\x01\x87a\x08\x1AV[\x82\x81\x03` \x84\x01Ra\x08\x81\x81\x87a\x08\x1AV[`\x01`\x01`\xA0\x1B\x03\x86\x16`@\x85\x01R\x83\x81\x03``\x85\x01R\x90Pa\x08\xA4\x81\x85a\x08\x1AV[\x97\x96PPPPPPPV[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R_` \x83\x01Q`\xA0`@\x84\x01Ra\x08\xDD`\xC0\x84\x01\x82a\x08\x1AV[\x90P`@\x84\x01Q`\x1F\x19\x84\x83\x03\x01``\x85\x01Ra\x08\xFA\x82\x82a\x08\x1AV[\x91PP`\x01\x80`\xA0\x1B\x03``\x85\x01Q\x16`\x80\x84\x01R`\x80\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\xA0\x85\x01Ra\t*\x82\x82a\x08\x1AV[\x95\x94PPPPPV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\t^WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\txW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\t\x96WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\t\xE3W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\t\xC1WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\t\xE0W_\x81U`\x01\x01a\t\xCDV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x01Wa\n\x01a\x06sV[a\n\x15\x81a\n\x0F\x84Ta\tdV[\x84a\t\x9CV[` `\x1F\x82\x11`\x01\x81\x14a\nGW_\x83\x15a\n0WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\t\xE0V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\nvW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\nVV[P\x84\x82\x10\x15a\n\x93W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x81R_a\n\xB4`@\x83\x01\x85a\x08\x1AV[\x82\x81\x03` \x84\x01Ra\t*\x81\x85a\x08\x1AV\xFE\xA2dipfsX\"\x12 \x9A\xA0*\xF4\xB9\xF7 \x9707\xECI_p#\x17a\xF2}K-/\xDB\xD6o\x1E\xAF\x9DI(\xE3\x99dsolcC\0\x08\x1A\x003",
    );
    /**Event with signature `NewTrigger(string,string,uint64)` and selector `0x8a81be05064e2aef2ff874e33696c0ee4c4177c775d7fd263bdb2bd666ffffd9`.
    ```solidity
    event NewTrigger(string serviceId, string workflowId, ILayerTrigger.TriggerId indexed triggerId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct NewTrigger {
        #[allow(missing_docs)]
        pub serviceId: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub workflowId: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub triggerId: <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NewTrigger {
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::String, alloy::sol_types::sol_data::String);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>, ILayerTrigger::TriggerId);
            const SIGNATURE: &'static str = "NewTrigger(string,string,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    138u8, 129u8, 190u8, 5u8, 6u8, 78u8, 42u8, 239u8, 47u8, 248u8, 116u8, 227u8,
                    54u8, 150u8, 192u8, 238u8, 76u8, 65u8, 119u8, 199u8, 117u8, 215u8, 253u8, 38u8,
                    59u8, 219u8, 43u8, 214u8, 102u8, 255u8, 255u8, 217u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { serviceId: data.0, workflowId: data.1, triggerId: topics.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.serviceId,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.workflowId,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.triggerId.clone())
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
                out[1usize] =
                    <ILayerTrigger::TriggerId as alloy_sol_types::EventTopic>::encode_topic(
                        &self.triggerId,
                    );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewTrigger {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewTrigger> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewTrigger) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `addTrigger(string,string,bytes)` and selector `0x30628cc8`.
    ```solidity
    function addTrigger(string memory serviceId, string memory workflowId, bytes memory data) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addTriggerCall {
        pub serviceId: alloy::sol_types::private::String,
        pub workflowId: alloy::sol_types::private::String,
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`addTrigger(string,string,bytes)`](addTriggerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addTriggerReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
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
            impl ::core::convert::From<addTriggerCall> for UnderlyingRustTuple<'_> {
                fn from(value: addTriggerCall) -> Self {
                    (value.serviceId, value.workflowId, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addTriggerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { serviceId: tuple.0, workflowId: tuple.1, data: tuple.2 }
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
            impl ::core::convert::From<addTriggerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addTriggerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addTriggerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addTriggerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addTriggerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addTrigger(string,string,bytes)";
            const SELECTOR: [u8; 4] = [48u8, 98u8, 140u8, 200u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.serviceId,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.workflowId,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
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
    /**Function with signature `getTrigger(uint64)` and selector `0xe328ed77`.
    ```solidity
    function getTrigger(ILayerTrigger.TriggerId triggerId) external view returns (ILayerTrigger.TriggerResponse memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTriggerCall {
        pub triggerId: <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getTrigger(uint64)`](getTriggerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTriggerReturn {
        pub _0: <ILayerTrigger::TriggerResponse as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ILayerTrigger::TriggerId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getTriggerCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTriggerCall) -> Self {
                    (value.triggerId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTriggerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { triggerId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ILayerTrigger::TriggerResponse,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ILayerTrigger::TriggerResponse as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getTriggerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTriggerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTriggerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTriggerCall {
            type Parameters<'a> = (ILayerTrigger::TriggerId,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTriggerReturn;
            type ReturnTuple<'a> = (ILayerTrigger::TriggerResponse,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTrigger(uint64)";
            const SELECTOR: [u8; 4] = [227u8, 40u8, 237u8, 119u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<ILayerTrigger::TriggerId as alloy_sol_types::SolType>::tokenize(&self.triggerId),)
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
    /**Function with signature `nextTriggerId()` and selector `0x42227fa4`.
    ```solidity
    function nextTriggerId() external view returns (ILayerTrigger.TriggerId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextTriggerIdCall {}
    ///Container type for the return parameters of the [`nextTriggerId()`](nextTriggerIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextTriggerIdReturn {
        pub _0: <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<nextTriggerIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: nextTriggerIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextTriggerIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ILayerTrigger::TriggerId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<nextTriggerIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nextTriggerIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextTriggerIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextTriggerIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = nextTriggerIdReturn;
            type ReturnTuple<'a> = (ILayerTrigger::TriggerId,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextTriggerId()";
            const SELECTOR: [u8; 4] = [66u8, 34u8, 127u8, 164u8];
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
    /**Function with signature `triggerIdsByCreator(address,uint256)` and selector `0x913b1fbf`.
    ```solidity
    function triggerIdsByCreator(address, uint256) external view returns (ILayerTrigger.TriggerId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggerIdsByCreatorCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`triggerIdsByCreator(address,uint256)`](triggerIdsByCreatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggerIdsByCreatorReturn {
        pub _0: <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<triggerIdsByCreatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: triggerIdsByCreatorCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggerIdsByCreatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ILayerTrigger::TriggerId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<triggerIdsByCreatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: triggerIdsByCreatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggerIdsByCreatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for triggerIdsByCreatorCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = triggerIdsByCreatorReturn;
            type ReturnTuple<'a> = (ILayerTrigger::TriggerId,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "triggerIdsByCreator(address,uint256)";
            const SELECTOR: [u8; 4] = [145u8, 59u8, 31u8, 191u8];
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `triggersById(uint64)` and selector `0xce289612`.
    ```solidity
    function triggersById(ILayerTrigger.TriggerId) external view returns (string memory serviceId, string memory workflowId, address creator, bytes memory data);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggersByIdCall {
        pub _0: <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`triggersById(uint64)`](triggersByIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggersByIdReturn {
        pub serviceId: alloy::sol_types::private::String,
        pub workflowId: alloy::sol_types::private::String,
        pub creator: alloy::sol_types::private::Address,
        pub data: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ILayerTrigger::TriggerId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<triggersByIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: triggersByIdCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggersByIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<triggersByIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: triggersByIdReturn) -> Self {
                    (value.serviceId, value.workflowId, value.creator, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggersByIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        serviceId: tuple.0,
                        workflowId: tuple.1,
                        creator: tuple.2,
                        data: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for triggersByIdCall {
            type Parameters<'a> = (ILayerTrigger::TriggerId,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = triggersByIdReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "triggersById(uint64)";
            const SELECTOR: [u8; 4] = [206u8, 40u8, 150u8, 18u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<ILayerTrigger::TriggerId as alloy_sol_types::SolType>::tokenize(&self._0),)
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
    ///Container for all the [`WavsTrigger`](self) function calls.
    pub enum WavsTriggerCalls {
        addTrigger(addTriggerCall),
        getTrigger(getTriggerCall),
        nextTriggerId(nextTriggerIdCall),
        triggerIdsByCreator(triggerIdsByCreatorCall),
        triggersById(triggersByIdCall),
    }
    #[automatically_derived]
    impl WavsTriggerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [48u8, 98u8, 140u8, 200u8],
            [66u8, 34u8, 127u8, 164u8],
            [145u8, 59u8, 31u8, 191u8],
            [206u8, 40u8, 150u8, 18u8],
            [227u8, 40u8, 237u8, 119u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for WavsTriggerCalls {
        const NAME: &'static str = "WavsTriggerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 5usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addTrigger(_) => <addTriggerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getTrigger(_) => <getTriggerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nextTriggerId(_) => <nextTriggerIdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::triggerIdsByCreator(_) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::triggersById(_) => <triggersByIdCall as alloy_sol_types::SolCall>::SELECTOR,
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
            static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<WavsTriggerCalls>] =
                &[
                    {
                        fn addTrigger(
                            data: &[u8],
                            validate: bool,
                        ) -> alloy_sol_types::Result<WavsTriggerCalls> {
                            <addTriggerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data, validate,
                            )
                            .map(WavsTriggerCalls::addTrigger)
                        }
                        addTrigger
                    },
                    {
                        fn nextTriggerId(
                            data: &[u8],
                            validate: bool,
                        ) -> alloy_sol_types::Result<WavsTriggerCalls> {
                            <nextTriggerIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data, validate,
                            )
                            .map(WavsTriggerCalls::nextTriggerId)
                        }
                        nextTriggerId
                    },
                    {
                        fn triggerIdsByCreator(
                            data: &[u8],
                            validate: bool,
                        ) -> alloy_sol_types::Result<WavsTriggerCalls> {
                            <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data, validate,
                            )
                            .map(WavsTriggerCalls::triggerIdsByCreator)
                        }
                        triggerIdsByCreator
                    },
                    {
                        fn triggersById(
                            data: &[u8],
                            validate: bool,
                        ) -> alloy_sol_types::Result<WavsTriggerCalls> {
                            <triggersByIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data, validate,
                            )
                            .map(WavsTriggerCalls::triggersById)
                        }
                        triggersById
                    },
                    {
                        fn getTrigger(
                            data: &[u8],
                            validate: bool,
                        ) -> alloy_sol_types::Result<WavsTriggerCalls> {
                            <getTriggerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data, validate,
                            )
                            .map(WavsTriggerCalls::getTrigger)
                        }
                        getTrigger
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
                Self::addTrigger(inner) => {
                    <addTriggerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getTrigger(inner) => {
                    <getTriggerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nextTriggerId(inner) => {
                    <nextTriggerIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::triggerIdsByCreator(inner) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::triggersById(inner) => {
                    <triggersByIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::addTrigger(inner) => {
                    <addTriggerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getTrigger(inner) => {
                    <getTriggerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nextTriggerId(inner) => {
                    <nextTriggerIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::triggerIdsByCreator(inner) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::triggersById(inner) => {
                    <triggersByIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`WavsTrigger`](self) events.
    pub enum WavsTriggerEvents {
        NewTrigger(NewTrigger),
    }
    #[automatically_derived]
    impl WavsTriggerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[[
            138u8, 129u8, 190u8, 5u8, 6u8, 78u8, 42u8, 239u8, 47u8, 248u8, 116u8, 227u8, 54u8,
            150u8, 192u8, 238u8, 76u8, 65u8, 119u8, 199u8, 117u8, 215u8, 253u8, 38u8, 59u8, 219u8,
            43u8, 214u8, 102u8, 255u8, 255u8, 217u8,
        ]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for WavsTriggerEvents {
        const NAME: &'static str = "WavsTriggerEvents";
        const COUNT: usize = 1usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<NewTrigger as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewTrigger as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::NewTrigger)
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
    impl alloy_sol_types::private::IntoLogData for WavsTriggerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::NewTrigger(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::NewTrigger(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`WavsTrigger`](self) contract instance.

    See the [wrapper's documentation](`WavsTriggerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> WavsTriggerInstance<T, P, N> {
        WavsTriggerInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<WavsTriggerInstance<T, P, N>>>
    {
        WavsTriggerInstance::<T, P, N>::deploy(provider)
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
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        WavsTriggerInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`WavsTrigger`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`WavsTrigger`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct WavsTriggerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for WavsTriggerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("WavsTriggerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > WavsTriggerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`WavsTrigger`](self) contract instance.

        See the [wrapper's documentation](`WavsTriggerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self { address, provider, _network_transport: ::core::marker::PhantomData }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(provider: P) -> alloy_contract::Result<WavsTriggerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<T, P: ::core::clone::Clone, N> WavsTriggerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> WavsTriggerInstance<T, P, N> {
            WavsTriggerInstance {
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
        > WavsTriggerInstance<T, P, N>
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
        ///Creates a new call builder for the [`addTrigger`] function.
        pub fn addTrigger(
            &self,
            serviceId: alloy::sol_types::private::String,
            workflowId: alloy::sol_types::private::String,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, addTriggerCall, N> {
            self.call_builder(&addTriggerCall { serviceId, workflowId, data })
        }
        ///Creates a new call builder for the [`getTrigger`] function.
        pub fn getTrigger(
            &self,
            triggerId: <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTriggerCall, N> {
            self.call_builder(&getTriggerCall { triggerId })
        }
        ///Creates a new call builder for the [`nextTriggerId`] function.
        pub fn nextTriggerId(&self) -> alloy_contract::SolCallBuilder<T, &P, nextTriggerIdCall, N> {
            self.call_builder(&nextTriggerIdCall {})
        }
        ///Creates a new call builder for the [`triggerIdsByCreator`] function.
        pub fn triggerIdsByCreator(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, triggerIdsByCreatorCall, N> {
            self.call_builder(&triggerIdsByCreatorCall { _0, _1 })
        }
        ///Creates a new call builder for the [`triggersById`] function.
        pub fn triggersById(
            &self,
            _0: <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, triggersByIdCall, N> {
            self.call_builder(&triggersByIdCall { _0 })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > WavsTriggerInstance<T, P, N>
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
        ///Creates a new event filter for the [`NewTrigger`] event.
        pub fn NewTrigger_filter(&self) -> alloy_contract::Event<T, &P, NewTrigger, N> {
            self.event_filter::<NewTrigger>()
        }
    }
}
