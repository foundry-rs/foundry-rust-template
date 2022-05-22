pub use vm_mod::*;
#[allow(clippy::too_many_arguments)]
mod vm_mod {
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
    #[doc = "Vm was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static VM_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> = ethers::contract::Lazy::new(
        || {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"accesses\",\"outputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"reads\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"writes\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addr\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"assume\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"chainId\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"clearMockedCalls\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"coinbase\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"etch\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"expectCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"expectCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"expectEmit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"expectEmit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"expectRevert\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"expectRevert\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"expectRevert\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"fee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string[]\",\"name\":\"\",\"type\":\"string[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"ffi\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getCode\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getNonce\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"label\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"load\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mockCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mockCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"prank\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"prank\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"record\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"roll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setNonce\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sign\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startPrank\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startPrank\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stopPrank\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"store\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"warp\",\"outputs\":[]}]") . expect ("invalid abi")
        },
    );
    pub struct Vm<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Vm<M> {
        fn clone(&self) -> Self {
            Vm(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Vm<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Vm<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Vm))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Vm<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), VM_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `accesses` (0x65bc9481) function"]
        pub fn accesses(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<[u8; 32]>, ::std::vec::Vec<[u8; 32]>),
        > {
            self.0
                .method_hash([101, 188, 148, 129], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addr` (0xffa18649) function"]
        pub fn addr(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([255, 161, 134, 73], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assume` (0x4c63e562) function"]
        pub fn assume(&self, p0: bool) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 99, 229, 98], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `chainId` (0x4049ddd2) function"]
        pub fn chain_id(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 73, 221, 210], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `clearMockedCalls` (0x3fdf4e15) function"]
        pub fn clear_mocked_calls(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 223, 78, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `coinbase` (0xff483c54) function"]
        pub fn coinbase(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 72, 60, 84], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deal` (0xc88a5e6d) function"]
        pub fn deal(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 138, 94, 109], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `etch` (0xb4d6c782) function"]
        pub fn etch(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 214, 199, 130], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expectCall` (0xbd6af434) function"]
        pub fn expect_call_0(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 106, 244, 52], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expectCall` (0xf30c7ba3) function"]
        pub fn expect_call_1(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
            p2: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 12, 123, 163], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expectEmit` (0x491cc7c2) function"]
        pub fn expect_emit_0(
            &self,
            p0: bool,
            p1: bool,
            p2: bool,
            p3: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 28, 199, 194], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expectEmit` (0x81bad6f3) function"]
        pub fn expect_emit_1(
            &self,
            p0: bool,
            p1: bool,
            p2: bool,
            p3: bool,
            p4: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 186, 214, 243], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expectRevert` (0xc31eb0e0) function"]
        pub fn expect_revert_1(
            &self,
            p0: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 30, 176, 224], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expectRevert` (0xf28dceb3) function"]
        pub fn expect_revert_2(
            &self,
            p0: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 141, 206, 179], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expectRevert` (0xf4844814) function"]
        pub fn expect_revert_0(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 132, 72, 20], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fee` (0x39b37ab0) function"]
        pub fn fee(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 179, 122, 176], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ffi` (0x89160467) function"]
        pub fn ffi(
            &self,
            p0: ::std::vec::Vec<String>,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([137, 22, 4, 103], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCode` (0x8d1cc925) function"]
        pub fn get_code(
            &self,
            p0: String,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([141, 28, 201, 37], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNonce` (0x2d0335ab) function"]
        pub fn get_nonce(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([45, 3, 53, 171], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `label` (0xc657c718) function"]
        pub fn label(
            &self,
            p0: ethers::core::types::Address,
            p1: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 87, 199, 24], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `load` (0x667f9d70) function"]
        pub fn load(
            &self,
            p0: ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([102, 127, 157, 112], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mockCall` (0x81409b91) function"]
        pub fn mock_call_1(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
            p2: ethers::core::types::Bytes,
            p3: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 64, 155, 145], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mockCall` (0xb96213e4) function"]
        pub fn mock_call_0(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Bytes,
            p2: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 98, 19, 228], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `prank` (0x47e50cce) function"]
        pub fn prank_1(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 229, 12, 206], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `prank` (0xca669fa7) function"]
        pub fn prank_0(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 102, 159, 167], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `record` (0x266cf109) function"]
        pub fn record(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 108, 241, 9], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roll` (0x1f7b4f30) function"]
        pub fn roll(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 123, 79, 48], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setNonce` (0xf8e18b57) function"]
        pub fn set_nonce(
            &self,
            p0: ethers::core::types::Address,
            p1: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 225, 139, 87], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sign` (0xe341eaa4) function"]
        pub fn sign(
            &self,
            p0: ethers::core::types::U256,
            p1: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, (u8, [u8; 32], [u8; 32])> {
            self.0
                .method_hash([227, 65, 234, 164], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startPrank` (0x06447d56) function"]
        pub fn start_prank_0(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 68, 125, 86], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startPrank` (0x45b56078) function"]
        pub fn start_prank_1(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 181, 96, 120], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stopPrank` (0x90c5013b) function"]
        pub fn stop_prank(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 197, 1, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `store` (0x70ca10bb) function"]
        pub fn store(
            &self,
            p0: ethers::core::types::Address,
            p1: [u8; 32],
            p2: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 202, 16, 187], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `warp` (0xe5d6bf02) function"]
        pub fn warp(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 214, 191, 2], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Vm<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `accesses`function with signature `accesses(address)` and selector `[101, 188, 148, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "accesses", abi = "accesses(address)")]
    pub struct AccessesCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `addr`function with signature `addr(uint256)` and selector `[255, 161, 134, 73]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addr", abi = "addr(uint256)")]
    pub struct AddrCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `assume`function with signature `assume(bool)` and selector `[76, 99, 229, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "assume", abi = "assume(bool)")]
    pub struct AssumeCall(pub bool);
    #[doc = "Container type for all input parameters for the `chainId`function with signature `chainId(uint256)` and selector `[64, 73, 221, 210]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "chainId", abi = "chainId(uint256)")]
    pub struct ChainIdCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `clearMockedCalls`function with signature `clearMockedCalls()` and selector `[63, 223, 78, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "clearMockedCalls", abi = "clearMockedCalls()")]
    pub struct ClearMockedCallsCall;
    #[doc = "Container type for all input parameters for the `coinbase`function with signature `coinbase(address)` and selector `[255, 72, 60, 84]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "coinbase", abi = "coinbase(address)")]
    pub struct CoinbaseCall(pub ethers::core::types::Address);
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
    pub struct DealCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `etch`function with signature `etch(address,bytes)` and selector `[180, 214, 199, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "etch", abi = "etch(address,bytes)")]
    pub struct EtchCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `expectCall`function with signature `expectCall(address,bytes)` and selector `[189, 106, 244, 52]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "expectCall", abi = "expectCall(address,bytes)")]
    pub struct ExpectCall0Call(
        pub ethers::core::types::Address,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `expectCall`function with signature `expectCall(address,uint256,bytes)` and selector `[243, 12, 123, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "expectCall", abi = "expectCall(address,uint256,bytes)")]
    pub struct ExpectCall1Call(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `expectEmit`function with signature `expectEmit(bool,bool,bool,bool)` and selector `[73, 28, 199, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "expectEmit", abi = "expectEmit(bool,bool,bool,bool)")]
    pub struct ExpectEmit0Call(pub bool, pub bool, pub bool, pub bool);
    #[doc = "Container type for all input parameters for the `expectEmit`function with signature `expectEmit(bool,bool,bool,bool,address)` and selector `[129, 186, 214, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "expectEmit", abi = "expectEmit(bool,bool,bool,bool,address)")]
    pub struct ExpectEmit1Call(
        pub bool,
        pub bool,
        pub bool,
        pub bool,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `expectRevert`function with signature `expectRevert(bytes4)` and selector `[195, 30, 176, 224]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "expectRevert", abi = "expectRevert(bytes4)")]
    pub struct ExpectRevert1Call(pub [u8; 4]);
    #[doc = "Container type for all input parameters for the `expectRevert`function with signature `expectRevert(bytes)` and selector `[242, 141, 206, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "expectRevert", abi = "expectRevert(bytes)")]
    pub struct ExpectRevert2Call(pub ethers::core::types::Bytes);
    #[doc = "Container type for all input parameters for the `expectRevert`function with signature `expectRevert()` and selector `[244, 132, 72, 20]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "expectRevert", abi = "expectRevert()")]
    pub struct ExpectRevert0Call;
    #[doc = "Container type for all input parameters for the `fee`function with signature `fee(uint256)` and selector `[57, 179, 122, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fee", abi = "fee(uint256)")]
    pub struct FeeCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `ffi`function with signature `ffi(string[])` and selector `[137, 22, 4, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ffi", abi = "ffi(string[])")]
    pub struct FfiCall(pub ::std::vec::Vec<String>);
    #[doc = "Container type for all input parameters for the `getCode`function with signature `getCode(string)` and selector `[141, 28, 201, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCode", abi = "getCode(string)")]
    pub struct GetCodeCall(pub String);
    #[doc = "Container type for all input parameters for the `getNonce`function with signature `getNonce(address)` and selector `[45, 3, 53, 171]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNonce", abi = "getNonce(address)")]
    pub struct GetNonceCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `label`function with signature `label(address,string)` and selector `[198, 87, 199, 24]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "label", abi = "label(address,string)")]
    pub struct LabelCall(pub ethers::core::types::Address, pub String);
    #[doc = "Container type for all input parameters for the `load`function with signature `load(address,bytes32)` and selector `[102, 127, 157, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "load", abi = "load(address,bytes32)")]
    pub struct LoadCall(pub ethers::core::types::Address, pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `mockCall`function with signature `mockCall(address,uint256,bytes,bytes)` and selector `[129, 64, 155, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mockCall", abi = "mockCall(address,uint256,bytes,bytes)")]
    pub struct MockCall1Call(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::Bytes,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `mockCall`function with signature `mockCall(address,bytes,bytes)` and selector `[185, 98, 19, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mockCall", abi = "mockCall(address,bytes,bytes)")]
    pub struct MockCall0Call(
        pub ethers::core::types::Address,
        pub ethers::core::types::Bytes,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `prank`function with signature `prank(address,address)` and selector `[71, 229, 12, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "prank", abi = "prank(address,address)")]
    pub struct Prank1Call(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `prank`function with signature `prank(address)` and selector `[202, 102, 159, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "prank", abi = "prank(address)")]
    pub struct Prank0Call(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `record`function with signature `record()` and selector `[38, 108, 241, 9]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "record", abi = "record()")]
    pub struct RecordCall;
    #[doc = "Container type for all input parameters for the `roll`function with signature `roll(uint256)` and selector `[31, 123, 79, 48]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "roll", abi = "roll(uint256)")]
    pub struct RollCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `setNonce`function with signature `setNonce(address,uint64)` and selector `[248, 225, 139, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setNonce", abi = "setNonce(address,uint64)")]
    pub struct SetNonceCall(pub ethers::core::types::Address, pub u64);
    #[doc = "Container type for all input parameters for the `sign`function with signature `sign(uint256,bytes32)` and selector `[227, 65, 234, 164]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "sign", abi = "sign(uint256,bytes32)")]
    pub struct SignCall(pub ethers::core::types::U256, pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `startPrank`function with signature `startPrank(address)` and selector `[6, 68, 125, 86]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "startPrank", abi = "startPrank(address)")]
    pub struct StartPrank0Call(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `startPrank`function with signature `startPrank(address,address)` and selector `[69, 181, 96, 120]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "startPrank", abi = "startPrank(address,address)")]
    pub struct StartPrank1Call(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `stopPrank`function with signature `stopPrank()` and selector `[144, 197, 1, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "stopPrank", abi = "stopPrank()")]
    pub struct StopPrankCall;
    #[doc = "Container type for all input parameters for the `store`function with signature `store(address,bytes32,bytes32)` and selector `[112, 202, 16, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "store", abi = "store(address,bytes32,bytes32)")]
    pub struct StoreCall(pub ethers::core::types::Address, pub [u8; 32], pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `warp`function with signature `warp(uint256)` and selector `[229, 214, 191, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "warp", abi = "warp(uint256)")]
    pub struct WarpCall(pub ethers::core::types::U256);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum VmCalls {
        Accesses(AccessesCall),
        Addr(AddrCall),
        Assume(AssumeCall),
        ChainId(ChainIdCall),
        ClearMockedCalls(ClearMockedCallsCall),
        Coinbase(CoinbaseCall),
        Deal(DealCall),
        Etch(EtchCall),
        ExpectCall0(ExpectCall0Call),
        ExpectCall1(ExpectCall1Call),
        ExpectEmit0(ExpectEmit0Call),
        ExpectEmit1(ExpectEmit1Call),
        ExpectRevert1(ExpectRevert1Call),
        ExpectRevert2(ExpectRevert2Call),
        ExpectRevert0(ExpectRevert0Call),
        Fee(FeeCall),
        Ffi(FfiCall),
        GetCode(GetCodeCall),
        GetNonce(GetNonceCall),
        Label(LabelCall),
        Load(LoadCall),
        MockCall1(MockCall1Call),
        MockCall0(MockCall0Call),
        Prank1(Prank1Call),
        Prank0(Prank0Call),
        Record(RecordCall),
        Roll(RollCall),
        SetNonce(SetNonceCall),
        Sign(SignCall),
        StartPrank0(StartPrank0Call),
        StartPrank1(StartPrank1Call),
        StopPrank(StopPrankCall),
        Store(StoreCall),
        Warp(WarpCall),
    }
    impl ethers::core::abi::AbiDecode for VmCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AccessesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::Accesses(decoded));
            }
            if let Ok(decoded) = <AddrCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VmCalls::Addr(decoded));
            }
            if let Ok(decoded) = <AssumeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::Assume(decoded));
            }
            if let Ok(decoded) =
                <ChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::ChainId(decoded));
            }
            if let Ok(decoded) =
                <ClearMockedCallsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::ClearMockedCalls(decoded));
            }
            if let Ok(decoded) =
                <CoinbaseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::Coinbase(decoded));
            }
            if let Ok(decoded) = <DealCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VmCalls::Deal(decoded));
            }
            if let Ok(decoded) = <EtchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VmCalls::Etch(decoded));
            }
            if let Ok(decoded) =
                <ExpectCall0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::ExpectCall0(decoded));
            }
            if let Ok(decoded) =
                <ExpectCall1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::ExpectCall1(decoded));
            }
            if let Ok(decoded) =
                <ExpectEmit0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::ExpectEmit0(decoded));
            }
            if let Ok(decoded) =
                <ExpectEmit1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::ExpectEmit1(decoded));
            }
            if let Ok(decoded) =
                <ExpectRevert1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::ExpectRevert1(decoded));
            }
            if let Ok(decoded) =
                <ExpectRevert2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::ExpectRevert2(decoded));
            }
            if let Ok(decoded) =
                <ExpectRevert0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::ExpectRevert0(decoded));
            }
            if let Ok(decoded) = <FeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VmCalls::Fee(decoded));
            }
            if let Ok(decoded) = <FfiCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VmCalls::Ffi(decoded));
            }
            if let Ok(decoded) =
                <GetCodeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::GetCode(decoded));
            }
            if let Ok(decoded) =
                <GetNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::GetNonce(decoded));
            }
            if let Ok(decoded) = <LabelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::Label(decoded));
            }
            if let Ok(decoded) = <LoadCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VmCalls::Load(decoded));
            }
            if let Ok(decoded) =
                <MockCall1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::MockCall1(decoded));
            }
            if let Ok(decoded) =
                <MockCall0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::MockCall0(decoded));
            }
            if let Ok(decoded) = <Prank1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::Prank1(decoded));
            }
            if let Ok(decoded) = <Prank0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::Prank0(decoded));
            }
            if let Ok(decoded) = <RecordCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::Record(decoded));
            }
            if let Ok(decoded) = <RollCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VmCalls::Roll(decoded));
            }
            if let Ok(decoded) =
                <SetNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::SetNonce(decoded));
            }
            if let Ok(decoded) = <SignCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VmCalls::Sign(decoded));
            }
            if let Ok(decoded) =
                <StartPrank0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::StartPrank0(decoded));
            }
            if let Ok(decoded) =
                <StartPrank1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::StartPrank1(decoded));
            }
            if let Ok(decoded) =
                <StopPrankCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::StopPrank(decoded));
            }
            if let Ok(decoded) = <StoreCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VmCalls::Store(decoded));
            }
            if let Ok(decoded) = <WarpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VmCalls::Warp(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for VmCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                VmCalls::Accesses(element) => element.encode(),
                VmCalls::Addr(element) => element.encode(),
                VmCalls::Assume(element) => element.encode(),
                VmCalls::ChainId(element) => element.encode(),
                VmCalls::ClearMockedCalls(element) => element.encode(),
                VmCalls::Coinbase(element) => element.encode(),
                VmCalls::Deal(element) => element.encode(),
                VmCalls::Etch(element) => element.encode(),
                VmCalls::ExpectCall0(element) => element.encode(),
                VmCalls::ExpectCall1(element) => element.encode(),
                VmCalls::ExpectEmit0(element) => element.encode(),
                VmCalls::ExpectEmit1(element) => element.encode(),
                VmCalls::ExpectRevert1(element) => element.encode(),
                VmCalls::ExpectRevert2(element) => element.encode(),
                VmCalls::ExpectRevert0(element) => element.encode(),
                VmCalls::Fee(element) => element.encode(),
                VmCalls::Ffi(element) => element.encode(),
                VmCalls::GetCode(element) => element.encode(),
                VmCalls::GetNonce(element) => element.encode(),
                VmCalls::Label(element) => element.encode(),
                VmCalls::Load(element) => element.encode(),
                VmCalls::MockCall1(element) => element.encode(),
                VmCalls::MockCall0(element) => element.encode(),
                VmCalls::Prank1(element) => element.encode(),
                VmCalls::Prank0(element) => element.encode(),
                VmCalls::Record(element) => element.encode(),
                VmCalls::Roll(element) => element.encode(),
                VmCalls::SetNonce(element) => element.encode(),
                VmCalls::Sign(element) => element.encode(),
                VmCalls::StartPrank0(element) => element.encode(),
                VmCalls::StartPrank1(element) => element.encode(),
                VmCalls::StopPrank(element) => element.encode(),
                VmCalls::Store(element) => element.encode(),
                VmCalls::Warp(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for VmCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VmCalls::Accesses(element) => element.fmt(f),
                VmCalls::Addr(element) => element.fmt(f),
                VmCalls::Assume(element) => element.fmt(f),
                VmCalls::ChainId(element) => element.fmt(f),
                VmCalls::ClearMockedCalls(element) => element.fmt(f),
                VmCalls::Coinbase(element) => element.fmt(f),
                VmCalls::Deal(element) => element.fmt(f),
                VmCalls::Etch(element) => element.fmt(f),
                VmCalls::ExpectCall0(element) => element.fmt(f),
                VmCalls::ExpectCall1(element) => element.fmt(f),
                VmCalls::ExpectEmit0(element) => element.fmt(f),
                VmCalls::ExpectEmit1(element) => element.fmt(f),
                VmCalls::ExpectRevert1(element) => element.fmt(f),
                VmCalls::ExpectRevert2(element) => element.fmt(f),
                VmCalls::ExpectRevert0(element) => element.fmt(f),
                VmCalls::Fee(element) => element.fmt(f),
                VmCalls::Ffi(element) => element.fmt(f),
                VmCalls::GetCode(element) => element.fmt(f),
                VmCalls::GetNonce(element) => element.fmt(f),
                VmCalls::Label(element) => element.fmt(f),
                VmCalls::Load(element) => element.fmt(f),
                VmCalls::MockCall1(element) => element.fmt(f),
                VmCalls::MockCall0(element) => element.fmt(f),
                VmCalls::Prank1(element) => element.fmt(f),
                VmCalls::Prank0(element) => element.fmt(f),
                VmCalls::Record(element) => element.fmt(f),
                VmCalls::Roll(element) => element.fmt(f),
                VmCalls::SetNonce(element) => element.fmt(f),
                VmCalls::Sign(element) => element.fmt(f),
                VmCalls::StartPrank0(element) => element.fmt(f),
                VmCalls::StartPrank1(element) => element.fmt(f),
                VmCalls::StopPrank(element) => element.fmt(f),
                VmCalls::Store(element) => element.fmt(f),
                VmCalls::Warp(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AccessesCall> for VmCalls {
        fn from(var: AccessesCall) -> Self {
            VmCalls::Accesses(var)
        }
    }
    impl ::std::convert::From<AddrCall> for VmCalls {
        fn from(var: AddrCall) -> Self {
            VmCalls::Addr(var)
        }
    }
    impl ::std::convert::From<AssumeCall> for VmCalls {
        fn from(var: AssumeCall) -> Self {
            VmCalls::Assume(var)
        }
    }
    impl ::std::convert::From<ChainIdCall> for VmCalls {
        fn from(var: ChainIdCall) -> Self {
            VmCalls::ChainId(var)
        }
    }
    impl ::std::convert::From<ClearMockedCallsCall> for VmCalls {
        fn from(var: ClearMockedCallsCall) -> Self {
            VmCalls::ClearMockedCalls(var)
        }
    }
    impl ::std::convert::From<CoinbaseCall> for VmCalls {
        fn from(var: CoinbaseCall) -> Self {
            VmCalls::Coinbase(var)
        }
    }
    impl ::std::convert::From<DealCall> for VmCalls {
        fn from(var: DealCall) -> Self {
            VmCalls::Deal(var)
        }
    }
    impl ::std::convert::From<EtchCall> for VmCalls {
        fn from(var: EtchCall) -> Self {
            VmCalls::Etch(var)
        }
    }
    impl ::std::convert::From<ExpectCall0Call> for VmCalls {
        fn from(var: ExpectCall0Call) -> Self {
            VmCalls::ExpectCall0(var)
        }
    }
    impl ::std::convert::From<ExpectCall1Call> for VmCalls {
        fn from(var: ExpectCall1Call) -> Self {
            VmCalls::ExpectCall1(var)
        }
    }
    impl ::std::convert::From<ExpectEmit0Call> for VmCalls {
        fn from(var: ExpectEmit0Call) -> Self {
            VmCalls::ExpectEmit0(var)
        }
    }
    impl ::std::convert::From<ExpectEmit1Call> for VmCalls {
        fn from(var: ExpectEmit1Call) -> Self {
            VmCalls::ExpectEmit1(var)
        }
    }
    impl ::std::convert::From<ExpectRevert1Call> for VmCalls {
        fn from(var: ExpectRevert1Call) -> Self {
            VmCalls::ExpectRevert1(var)
        }
    }
    impl ::std::convert::From<ExpectRevert2Call> for VmCalls {
        fn from(var: ExpectRevert2Call) -> Self {
            VmCalls::ExpectRevert2(var)
        }
    }
    impl ::std::convert::From<ExpectRevert0Call> for VmCalls {
        fn from(var: ExpectRevert0Call) -> Self {
            VmCalls::ExpectRevert0(var)
        }
    }
    impl ::std::convert::From<FeeCall> for VmCalls {
        fn from(var: FeeCall) -> Self {
            VmCalls::Fee(var)
        }
    }
    impl ::std::convert::From<FfiCall> for VmCalls {
        fn from(var: FfiCall) -> Self {
            VmCalls::Ffi(var)
        }
    }
    impl ::std::convert::From<GetCodeCall> for VmCalls {
        fn from(var: GetCodeCall) -> Self {
            VmCalls::GetCode(var)
        }
    }
    impl ::std::convert::From<GetNonceCall> for VmCalls {
        fn from(var: GetNonceCall) -> Self {
            VmCalls::GetNonce(var)
        }
    }
    impl ::std::convert::From<LabelCall> for VmCalls {
        fn from(var: LabelCall) -> Self {
            VmCalls::Label(var)
        }
    }
    impl ::std::convert::From<LoadCall> for VmCalls {
        fn from(var: LoadCall) -> Self {
            VmCalls::Load(var)
        }
    }
    impl ::std::convert::From<MockCall1Call> for VmCalls {
        fn from(var: MockCall1Call) -> Self {
            VmCalls::MockCall1(var)
        }
    }
    impl ::std::convert::From<MockCall0Call> for VmCalls {
        fn from(var: MockCall0Call) -> Self {
            VmCalls::MockCall0(var)
        }
    }
    impl ::std::convert::From<Prank1Call> for VmCalls {
        fn from(var: Prank1Call) -> Self {
            VmCalls::Prank1(var)
        }
    }
    impl ::std::convert::From<Prank0Call> for VmCalls {
        fn from(var: Prank0Call) -> Self {
            VmCalls::Prank0(var)
        }
    }
    impl ::std::convert::From<RecordCall> for VmCalls {
        fn from(var: RecordCall) -> Self {
            VmCalls::Record(var)
        }
    }
    impl ::std::convert::From<RollCall> for VmCalls {
        fn from(var: RollCall) -> Self {
            VmCalls::Roll(var)
        }
    }
    impl ::std::convert::From<SetNonceCall> for VmCalls {
        fn from(var: SetNonceCall) -> Self {
            VmCalls::SetNonce(var)
        }
    }
    impl ::std::convert::From<SignCall> for VmCalls {
        fn from(var: SignCall) -> Self {
            VmCalls::Sign(var)
        }
    }
    impl ::std::convert::From<StartPrank0Call> for VmCalls {
        fn from(var: StartPrank0Call) -> Self {
            VmCalls::StartPrank0(var)
        }
    }
    impl ::std::convert::From<StartPrank1Call> for VmCalls {
        fn from(var: StartPrank1Call) -> Self {
            VmCalls::StartPrank1(var)
        }
    }
    impl ::std::convert::From<StopPrankCall> for VmCalls {
        fn from(var: StopPrankCall) -> Self {
            VmCalls::StopPrank(var)
        }
    }
    impl ::std::convert::From<StoreCall> for VmCalls {
        fn from(var: StoreCall) -> Self {
            VmCalls::Store(var)
        }
    }
    impl ::std::convert::From<WarpCall> for VmCalls {
        fn from(var: WarpCall) -> Self {
            VmCalls::Warp(var)
        }
    }
}
