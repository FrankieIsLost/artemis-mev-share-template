pub use counter::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod counter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("increment"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("increment"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("number"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("number"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setNumber"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setNumber"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newNumber"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COUNTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\xF7\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`\x046\x10`<W`\x005`\xE0\x1C\x80c?\xB5\xC1\xCB\x14`AW\x80c\x83\x81\xF5\x8A\x14`SW\x80c\xD0\x9D\xE0\x8A\x14`mW[`\0\x80\xFD[`Q`L6`\x04`\x83V[`\0UV[\0[`[`\0T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`Q`\0\x80T\x90\x80`|\x83`\x9BV[\x91\x90PUPV[`\0` \x82\x84\x03\x12\x15`\x94W`\0\x80\xFD[P5\x91\x90PV[`\0`\x01\x82\x01`\xBAWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xFA\xE0\xB1\xCE\xFC\x14\xF81g\x80q\xDA\xC5m|um\xBAJ~pWB\xBE?G<\x8C\x85\xE2v\x95dsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static COUNTER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`\x046\x10`<W`\x005`\xE0\x1C\x80c?\xB5\xC1\xCB\x14`AW\x80c\x83\x81\xF5\x8A\x14`SW\x80c\xD0\x9D\xE0\x8A\x14`mW[`\0\x80\xFD[`Q`L6`\x04`\x83V[`\0UV[\0[`[`\0T\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`Q`\0\x80T\x90\x80`|\x83`\x9BV[\x91\x90PUPV[`\0` \x82\x84\x03\x12\x15`\x94W`\0\x80\xFD[P5\x91\x90PV[`\0`\x01\x82\x01`\xBAWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \xFA\xE0\xB1\xCE\xFC\x14\xF81g\x80q\xDA\xC5m|um\xBAJ~pWB\xBE?G<\x8C\x85\xE2v\x95dsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static COUNTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Counter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Counter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Counter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Counter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Counter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Counter)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Counter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(address.into(), COUNTER_ABI.clone(), client))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                COUNTER_ABI.clone(),
                COUNTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `increment` (0xd09de08a) function
        pub fn increment(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 157, 224, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `number` (0x8381f58a) function
        pub fn number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([131, 129, 245, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNumber` (0x3fb5c1cb) function
        pub fn set_number(
            &self,
            new_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 181, 193, 203], new_number)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Counter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `increment` function with signature `increment()` and selector `0xd09de08a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "increment", abi = "increment()")]
    pub struct IncrementCall;
    ///Container type for all input parameters for the `number` function with signature `number()` and selector `0x8381f58a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "number", abi = "number()")]
    pub struct NumberCall;
    ///Container type for all input parameters for the `setNumber` function with signature `setNumber(uint256)` and selector `0x3fb5c1cb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setNumber", abi = "setNumber(uint256)")]
    pub struct SetNumberCall {
        pub new_number: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CounterCalls {
        Increment(IncrementCall),
        Number(NumberCall),
        SetNumber(SetNumberCall),
    }
    impl ::ethers::core::abi::AbiDecode for CounterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IncrementCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Increment(decoded));
            }
            if let Ok(decoded) = <NumberCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Number(decoded));
            }
            if let Ok(decoded) = <SetNumberCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetNumber(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CounterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Increment(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Number(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetNumber(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for CounterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Increment(element) => ::core::fmt::Display::fmt(element, f),
                Self::Number(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNumber(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IncrementCall> for CounterCalls {
        fn from(value: IncrementCall) -> Self {
            Self::Increment(value)
        }
    }
    impl ::core::convert::From<NumberCall> for CounterCalls {
        fn from(value: NumberCall) -> Self {
            Self::Number(value)
        }
    }
    impl ::core::convert::From<SetNumberCall> for CounterCalls {
        fn from(value: SetNumberCall) -> Self {
            Self::SetNumber(value)
        }
    }
    ///Container type for all return fields from the `number` function with signature `number()` and selector `0x8381f58a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NumberReturn(pub ::ethers::core::types::U256);
}
