#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 13usize] = [
        "System",
        "Timestamp",
        "Aura",
        "Grandpa",
        "Balances",
        "TransactionPayment",
        "Sudo",
        "TemplateModule",
        "Contracts",
        "RandomnessCollectiveFlip",
        "Identity",
        "Multisig",
        "Dhatu",
    ];
    #[doc = r" The error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    #[derive(
        :: subxt :: ext :: codec :: Decode,
        :: subxt :: ext :: codec :: Encode,
        :: subxt :: ext :: scale_decode :: DecodeAsType,
        :: subxt :: ext :: scale_encode :: EncodeAsType,
        Debug,
    )]
    # [codec (crate = :: subxt :: ext :: codec)]
    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 3)]
        Grandpa(grandpa::Event),
        #[codec(index = 4)]
        Balances(balances::Event),
        #[codec(index = 5)]
        TransactionPayment(transaction_payment::Event),
        #[codec(index = 6)]
        Sudo(sudo::Event),
        #[codec(index = 7)]
        TemplateModule(template_module::Event),
        #[codec(index = 8)]
        Contracts(contracts::Event),
        #[codec(index = 10)]
        Identity(identity::Event),
        #[codec(index = 11)]
        Multisig(multisig::Event),
        #[codec(index = 12)]
        Dhatu(dhatu::Event),
    }
    impl ::subxt::events::RootEvent for Event {
        fn root_event(
            pallet_bytes: &[u8],
            pallet_name: &str,
            pallet_ty: u32,
            metadata: &::subxt::Metadata,
        ) -> Result<Self, ::subxt::Error> {
            use ::subxt::metadata::DecodeWithMetadata;
            if pallet_name == "System" {
                return Ok(Event::System(system::Event::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "Grandpa" {
                return Ok(Event::Grandpa(grandpa::Event::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "Balances" {
                return Ok(Event::Balances(balances::Event::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "TransactionPayment" {
                return Ok(Event::TransactionPayment(
                    transaction_payment::Event::decode_with_metadata(
                        &mut &*pallet_bytes,
                        pallet_ty,
                        metadata,
                    )?,
                ));
            }
            if pallet_name == "Sudo" {
                return Ok(Event::Sudo(sudo::Event::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "TemplateModule" {
                return Ok(Event::TemplateModule(
                    template_module::Event::decode_with_metadata(
                        &mut &*pallet_bytes,
                        pallet_ty,
                        metadata,
                    )?,
                ));
            }
            if pallet_name == "Contracts" {
                return Ok(Event::Contracts(contracts::Event::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "Identity" {
                return Ok(Event::Identity(identity::Event::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "Multisig" {
                return Ok(Event::Multisig(multisig::Event::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "Dhatu" {
                return Ok(Event::Dhatu(dhatu::Event::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            Err(::subxt::ext::scale_decode::Error::custom(format!(
                "Pallet name '{}' not found in root Event enum",
                pallet_name
            ))
            .into())
        }
    }
    #[derive(
        :: subxt :: ext :: codec :: Decode,
        :: subxt :: ext :: codec :: Encode,
        :: subxt :: ext :: scale_decode :: DecodeAsType,
        :: subxt :: ext :: scale_encode :: EncodeAsType,
        Debug,
    )]
    # [codec (crate = :: subxt :: ext :: codec)]
    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
    pub enum Call {
        #[codec(index = 0)]
        System(system::Call),
        #[codec(index = 1)]
        Timestamp(timestamp::Call),
        #[codec(index = 3)]
        Grandpa(grandpa::Call),
        #[codec(index = 4)]
        Balances(balances::Call),
        #[codec(index = 6)]
        Sudo(sudo::Call),
        #[codec(index = 7)]
        TemplateModule(template_module::Call),
        #[codec(index = 8)]
        Contracts(contracts::Call),
        #[codec(index = 10)]
        Identity(identity::Call),
        #[codec(index = 11)]
        Multisig(multisig::Call),
        #[codec(index = 12)]
        Dhatu(dhatu::Call),
    }
    impl ::subxt::blocks::RootExtrinsic for Call {
        fn root_extrinsic(
            pallet_bytes: &[u8],
            pallet_name: &str,
            pallet_ty: u32,
            metadata: &::subxt::Metadata,
        ) -> Result<Self, ::subxt::Error> {
            use ::subxt::metadata::DecodeWithMetadata;
            if pallet_name == "System" {
                return Ok(Call::System(system::Call::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "Timestamp" {
                return Ok(Call::Timestamp(timestamp::Call::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "Grandpa" {
                return Ok(Call::Grandpa(grandpa::Call::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "Balances" {
                return Ok(Call::Balances(balances::Call::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "Sudo" {
                return Ok(Call::Sudo(sudo::Call::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "TemplateModule" {
                return Ok(Call::TemplateModule(
                    template_module::Call::decode_with_metadata(
                        &mut &*pallet_bytes,
                        pallet_ty,
                        metadata,
                    )?,
                ));
            }
            if pallet_name == "Contracts" {
                return Ok(Call::Contracts(contracts::Call::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "Identity" {
                return Ok(Call::Identity(identity::Call::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "Multisig" {
                return Ok(Call::Multisig(multisig::Call::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            if pallet_name == "Dhatu" {
                return Ok(Call::Dhatu(dhatu::Call::decode_with_metadata(
                    &mut &*pallet_bytes,
                    pallet_ty,
                    metadata,
                )?));
            }
            Err(::subxt::ext::scale_decode::Error::custom(format!(
                "Pallet name '{}' not found in root Call enum",
                pallet_name
            ))
            .into())
        }
    }
    #[derive(
        :: subxt :: ext :: codec :: Decode,
        :: subxt :: ext :: codec :: Encode,
        :: subxt :: ext :: scale_decode :: DecodeAsType,
        :: subxt :: ext :: scale_encode :: EncodeAsType,
        Debug,
    )]
    # [codec (crate = :: subxt :: ext :: codec)]
    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
    pub enum Error {
        #[codec(index = 0)]
        System(system::Error),
        #[codec(index = 3)]
        Grandpa(grandpa::Error),
        #[codec(index = 4)]
        Balances(balances::Error),
        #[codec(index = 6)]
        Sudo(sudo::Error),
        #[codec(index = 7)]
        TemplateModule(template_module::Error),
        #[codec(index = 8)]
        Contracts(contracts::Error),
        #[codec(index = 10)]
        Identity(identity::Error),
        #[codec(index = 11)]
        Multisig(multisig::Error),
    }
    impl ::subxt::error::RootError for Error {
        fn root_error(
            pallet_bytes: &[u8],
            pallet_name: &str,
            metadata: &::subxt::Metadata,
        ) -> Result<Self, ::subxt::Error> {
            use ::subxt::metadata::DecodeWithMetadata;
            let cursor = &mut &pallet_bytes[..];
            if pallet_name == "System" {
                let variant_error = system::Error::decode_with_metadata(cursor, 73u32, metadata)?;
                return Ok(Error::System(variant_error));
            }
            if pallet_name == "Grandpa" {
                let variant_error = grandpa::Error::decode_with_metadata(cursor, 97u32, metadata)?;
                return Ok(Error::Grandpa(variant_error));
            }
            if pallet_name == "Balances" {
                let variant_error =
                    balances::Error::decode_with_metadata(cursor, 118u32, metadata)?;
                return Ok(Error::Balances(variant_error));
            }
            if pallet_name == "Sudo" {
                let variant_error = sudo::Error::decode_with_metadata(cursor, 179u32, metadata)?;
                return Ok(Error::Sudo(variant_error));
            }
            if pallet_name == "TemplateModule" {
                let variant_error =
                    template_module::Error::decode_with_metadata(cursor, 180u32, metadata)?;
                return Ok(Error::TemplateModule(variant_error));
            }
            if pallet_name == "Contracts" {
                let variant_error =
                    contracts::Error::decode_with_metadata(cursor, 193u32, metadata)?;
                return Ok(Error::Contracts(variant_error));
            }
            if pallet_name == "Identity" {
                let variant_error =
                    identity::Error::decode_with_metadata(cursor, 205u32, metadata)?;
                return Ok(Error::Identity(variant_error));
            }
            if pallet_name == "Multisig" {
                let variant_error =
                    multisig::Error::decode_with_metadata(cursor, 209u32, metadata)?;
                return Ok(Error::Multisig(variant_error));
            }
            Err(::subxt::ext::scale_decode::Error::custom(format!(
                "Pallet name '{}' not found in root Error enum",
                pallet_name
            ))
            .into())
        }
    }
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use ::subxt::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {}
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn contracts(&self) -> contracts::constants::ConstantsApi {
            contracts::constants::ConstantsApi
        }
        pub fn identity(&self) -> identity::constants::ConstantsApi {
            identity::constants::ConstantsApi
        }
        pub fn multisig(&self) -> multisig::constants::ConstantsApi {
            multisig::constants::ConstantsApi
        }
        pub fn dhatu(&self) -> dhatu::constants::ConstantsApi {
            dhatu::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn aura(&self) -> aura::storage::StorageApi {
            aura::storage::StorageApi
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi {
            grandpa::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }
        pub fn template_module(&self) -> template_module::storage::StorageApi {
            template_module::storage::StorageApi
        }
        pub fn contracts(&self) -> contracts::storage::StorageApi {
            contracts::storage::StorageApi
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi {
            randomness_collective_flip::storage::StorageApi
        }
        pub fn identity(&self) -> identity::storage::StorageApi {
            identity::storage::StorageApi
        }
        pub fn multisig(&self) -> multisig::storage::StorageApi {
            multisig::storage::StorageApi
        }
        pub fn dhatu(&self) -> dhatu::storage::StorageApi {
            dhatu::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
            grandpa::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi {
            sudo::calls::TransactionApi
        }
        pub fn template_module(&self) -> template_module::calls::TransactionApi {
            template_module::calls::TransactionApi
        }
        pub fn contracts(&self) -> contracts::calls::TransactionApi {
            contracts::calls::TransactionApi
        }
        pub fn identity(&self) -> identity::calls::TransactionApi {
            identity::calls::TransactionApi
        }
        pub fn multisig(&self) -> multisig::calls::TransactionApi {
            multisig::calls::TransactionApi
        }
        pub fn dhatu(&self) -> dhatu::calls::TransactionApi {
            dhatu::calls::TransactionApi
        }
    }
    #[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
    pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
        client: &C,
    ) -> Result<(), ::subxt::error::MetadataError> {
        let runtime_metadata_hash = client
            .metadata()
            .hasher()
            .only_these_pallets(&PALLETS)
            .hash();
        if runtime_metadata_hash
            != [
                162u8, 135u8, 185u8, 189u8, 184u8, 134u8, 199u8, 133u8, 97u8, 230u8, 203u8, 224u8,
                144u8, 116u8, 100u8, 10u8, 183u8, 159u8, 106u8, 18u8, 102u8, 69u8, 43u8, 100u8,
                87u8, 205u8, 214u8, 137u8, 165u8, 44u8, 112u8, 110u8,
            ]
        {
            Err(::subxt::error::MetadataError::IncompatibleCodegen)
        } else {
            Ok(())
        }
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the System pallet"]
        pub type Error = runtime_types::frame_system::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::frame_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Remark {
                    pub remark: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for Remark {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetHeapPages {
                    pub pages: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetHeapPages {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_heap_pages";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetCode {
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCode {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetCodeWithoutChecks {
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCodeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code_without_checks";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetStorage {
                    pub items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_storage";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KillStorage {
                    pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                }
                impl ::subxt::blocks::StaticExtrinsic for KillStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_storage";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KillPrefix {
                    pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                    pub subkeys: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for KillPrefix {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_prefix";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemarkWithEvent {
                    pub remark: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemarkWithEvent {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark_with_event";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(1)`"]
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::Remark> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "remark",
                        types::Remark { remark },
                        [
                            43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
                            216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
                            250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
                            13u8,
                        ],
                    )
                }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::tx::Payload<types::SetHeapPages> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_heap_pages",
                        types::SetHeapPages { pages },
                        [
                            188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
                            215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
                            134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
                            57u8, 147u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::SetCode> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_code",
                        types::SetCode { code },
                        [
                            233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
                            203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
                            27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
                        ],
                    )
                }
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(C)` where `C` length of `code`"]
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::SetCodeWithoutChecks> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_code_without_checks",
                        types::SetCodeWithoutChecks { code },
                        [
                            82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
                            157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
                            147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
                            115u8,
                        ],
                    )
                }
                #[doc = "Set some items of storage."]
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::tx::Payload<types::SetStorage> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_storage",
                        types::SetStorage { items },
                        [
                            184u8, 169u8, 248u8, 68u8, 40u8, 193u8, 190u8, 151u8, 96u8, 159u8,
                            19u8, 237u8, 241u8, 156u8, 5u8, 158u8, 191u8, 237u8, 9u8, 13u8, 86u8,
                            213u8, 77u8, 58u8, 48u8, 139u8, 1u8, 85u8, 220u8, 233u8, 139u8, 164u8,
                        ],
                    )
                }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::tx::Payload<types::KillStorage> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "kill_storage",
                        types::KillStorage { keys },
                        [
                            73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
                            234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
                            156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
                            35u8,
                        ],
                    )
                }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::KillPrefix> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "kill_prefix",
                        types::KillPrefix { prefix, subkeys },
                        [
                            184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
                            175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
                            67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
                            85u8,
                        ],
                    )
                }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::RemarkWithEvent> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "remark_with_event",
                        types::RemarkWithEvent { remark },
                        [
                            120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
                            228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
                            147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Event for the System pallet."]
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
            }
            impl ::subxt::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: ::subxt::utils::AccountId32,
                pub hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The full account information for a particular account ID."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            234u8, 12u8, 167u8, 96u8, 2u8, 244u8, 235u8, 62u8, 153u8, 200u8, 96u8,
                            74u8, 135u8, 8u8, 35u8, 188u8, 146u8, 249u8, 246u8, 40u8, 224u8, 22u8,
                            15u8, 99u8, 150u8, 222u8, 82u8, 85u8, 123u8, 123u8, 19u8, 110u8,
                        ],
                    )
                }
                #[doc = " The full account information for a particular account ID."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Account",
                        Vec::new(),
                        [
                            234u8, 12u8, 167u8, 96u8, 2u8, 244u8, 235u8, 62u8, 153u8, 200u8, 96u8,
                            74u8, 135u8, 8u8, 35u8, 188u8, 146u8, 249u8, 246u8, 40u8, 224u8, 22u8,
                            15u8, 99u8, 150u8, 222u8, 82u8, 85u8, 123u8, 123u8, 19u8, 110u8,
                        ],
                    )
                }
                #[doc = " Total extrinsics count for the current block."]
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicCount",
                        vec![],
                        [
                            102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
                            153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
                            120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
                            159u8, 79u8,
                        ],
                    )
                }
                #[doc = " The current weight for the block."]
                pub fn block_weight(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockWeight",
                        vec![],
                        [
                            52u8, 191u8, 212u8, 137u8, 26u8, 39u8, 239u8, 35u8, 182u8, 32u8, 39u8,
                            103u8, 56u8, 184u8, 60u8, 159u8, 167u8, 232u8, 193u8, 116u8, 105u8,
                            56u8, 98u8, 127u8, 124u8, 188u8, 214u8, 154u8, 160u8, 41u8, 20u8,
                            162u8,
                        ],
                    )
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "AllExtrinsicsLen",
                        vec![],
                        [
                            117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
                            243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
                            101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
                            242u8, 65u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockHash",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                #[doc = " Map of block numbers to block hashes."]
                pub fn block_hash_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockHash",
                        Vec::new(),
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicData",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub fn extrinsic_data_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<::core::primitive::u8>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicData",
                        Vec::new(),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub fn number(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Number",
                        vec![],
                        [
                            30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
                            9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
                            200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
                        ],
                    )
                }
                #[doc = " Hash of the previous block."]
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::H256,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ParentHash",
                        vec![],
                        [
                            26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
                            192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
                            71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
                        ],
                    )
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub fn digest(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_runtime::generic::digest::Digest,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Digest",
                        vec![],
                        [
                            70u8, 156u8, 127u8, 89u8, 115u8, 250u8, 103u8, 62u8, 185u8, 153u8,
                            26u8, 72u8, 39u8, 226u8, 181u8, 97u8, 137u8, 225u8, 45u8, 158u8, 212u8,
                            254u8, 142u8, 136u8, 90u8, 22u8, 243u8, 125u8, 226u8, 49u8, 235u8,
                            215u8,
                        ],
                    )
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub fn events(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::local_runtime::RuntimeEvent,
                            ::subxt::utils::H256,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Events",
                        vec![],
                        [
                            212u8, 107u8, 173u8, 78u8, 5u8, 151u8, 110u8, 234u8, 173u8, 8u8, 139u8,
                            74u8, 10u8, 59u8, 221u8, 189u8, 17u8, 48u8, 231u8, 210u8, 198u8, 114u8,
                            201u8, 132u8, 64u8, 177u8, 136u8, 63u8, 85u8, 250u8, 170u8, 174u8,
                        ],
                    )
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub fn event_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventCount",
                        vec![],
                        [
                            175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
                            151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
                            254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
                            133u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventTopics",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            154u8, 29u8, 31u8, 148u8, 254u8, 7u8, 124u8, 251u8, 241u8, 77u8, 24u8,
                            37u8, 28u8, 75u8, 205u8, 17u8, 159u8, 79u8, 239u8, 62u8, 67u8, 60u8,
                            252u8, 112u8, 215u8, 145u8, 103u8, 170u8, 110u8, 186u8, 221u8, 76u8,
                        ],
                    )
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub fn event_topics_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventTopics",
                        Vec::new(),
                        [
                            154u8, 29u8, 31u8, 148u8, 254u8, 7u8, 124u8, 251u8, 241u8, 77u8, 24u8,
                            37u8, 28u8, 75u8, 205u8, 17u8, 159u8, 79u8, 239u8, 62u8, 67u8, 60u8,
                            252u8, 112u8, 215u8, 145u8, 103u8, 170u8, 110u8, 186u8, 221u8, 76u8,
                        ],
                    )
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::LastRuntimeUpgradeInfo,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "LastRuntimeUpgrade",
                        vec![],
                        [
                            137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
                            148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
                            194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "UpgradedToU32RefCount",
                        vec![],
                        [
                            229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
                            130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
                            107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
                        ],
                    )
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "UpgradedToTripleRefCount",
                        vec![],
                        [
                            97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
                            101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
                            167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
                            151u8,
                        ],
                    )
                }
                #[doc = " The execution phase of the block."]
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::frame_system::Phase,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExecutionPhase",
                        vec![],
                        [
                            191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
                            0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
                            35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockWeights>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockWeights",
                        [
                            238u8, 20u8, 221u8, 11u8, 146u8, 236u8, 47u8, 103u8, 8u8, 239u8, 13u8,
                            176u8, 202u8, 10u8, 151u8, 68u8, 110u8, 162u8, 99u8, 40u8, 211u8,
                            136u8, 71u8, 82u8, 50u8, 80u8, 244u8, 211u8, 231u8, 198u8, 36u8, 152u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockLength>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockLength",
                        [
                            117u8, 144u8, 154u8, 125u8, 106u8, 34u8, 224u8, 228u8, 80u8, 76u8,
                            126u8, 0u8, 177u8, 223u8, 116u8, 244u8, 167u8, 23u8, 253u8, 44u8,
                            128u8, 116u8, 155u8, 245u8, 163u8, 20u8, 21u8, 222u8, 174u8, 237u8,
                            162u8, 240u8,
                        ],
                    )
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::sp_weights::RuntimeDbWeight>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "DbWeight",
                        [
                            206u8, 53u8, 134u8, 247u8, 42u8, 38u8, 197u8, 59u8, 191u8, 83u8, 160u8,
                            9u8, 207u8, 133u8, 108u8, 152u8, 150u8, 103u8, 109u8, 228u8, 218u8,
                            24u8, 27u8, 210u8, 106u8, 252u8, 74u8, 93u8, 27u8, 63u8, 109u8, 252u8,
                        ],
                    )
                }
                #[doc = " Get the chain's current version."]
                pub fn version(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::sp_version::RuntimeVersion>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "Version",
                        [
                            134u8, 0u8, 23u8, 0u8, 199u8, 213u8, 89u8, 240u8, 194u8, 186u8, 239u8,
                            157u8, 168u8, 211u8, 223u8, 156u8, 138u8, 140u8, 194u8, 23u8, 167u8,
                            158u8, 195u8, 233u8, 25u8, 165u8, 27u8, 237u8, 198u8, 206u8, 233u8,
                            28u8,
                        ],
                    )
                }
                #[doc = " The designated SS58 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_timestamp::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Set {
                    #[codec(compact)]
                    pub now: ::core::primitive::u64,
                }
                impl ::subxt::blocks::StaticExtrinsic for Set {
                    const PALLET: &'static str = "Timestamp";
                    const CALL: &'static str = "set";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "`MinimumPeriod`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Inherent`."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                pub fn set(&self, now: ::core::primitive::u64) -> ::subxt::tx::Payload<types::Set> {
                    ::subxt::tx::Payload::new_static(
                        "Timestamp",
                        "set",
                        types::Set { now },
                        [
                            37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
                            199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
                            200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Current time for the current block."]
                pub fn now(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Timestamp",
                        "Now",
                        vec![],
                        [
                            44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
                            92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
                            141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
                        ],
                    )
                }
                #[doc = " Did the timestamp get updated in this block?"]
                pub fn did_update(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::bool,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Timestamp",
                        "DidUpdate",
                        vec![],
                        [
                            229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
                            205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
                            248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
                            214u8, 140u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
                #[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
                #[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
                #[doc = " double this period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u64> {
                    ::subxt::constants::Address::new_static(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The current authority set."]
                pub fn authorities(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Aura",
                        "Authorities",
                        vec![],
                        [
                            232u8, 129u8, 167u8, 104u8, 47u8, 188u8, 238u8, 164u8, 6u8, 29u8,
                            129u8, 45u8, 64u8, 182u8, 194u8, 47u8, 0u8, 73u8, 63u8, 102u8, 204u8,
                            94u8, 111u8, 96u8, 137u8, 7u8, 141u8, 110u8, 180u8, 80u8, 228u8, 16u8,
                        ],
                    )
                }
                #[doc = " The current slot of this block."]
                #[doc = ""]
                #[doc = " This will be set in `on_initialize`."]
                pub fn current_slot(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Aura",
                        "CurrentSlot",
                        vec![],
                        [
                            112u8, 199u8, 115u8, 248u8, 217u8, 242u8, 45u8, 231u8, 178u8, 53u8,
                            236u8, 167u8, 219u8, 238u8, 81u8, 243u8, 39u8, 140u8, 68u8, 19u8,
                            201u8, 169u8, 211u8, 133u8, 135u8, 213u8, 150u8, 105u8, 60u8, 252u8,
                            43u8, 57u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::pallet_grandpa::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_grandpa::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReportEquivocation {
                    pub equivocation_proof: ::std::boxed::Box<
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::utils::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    pub key_owner_proof: runtime_types::sp_core::Void,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReportEquivocation {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "report_equivocation";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReportEquivocationUnsigned {
                    pub equivocation_proof: ::std::boxed::Box<
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::utils::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    pub key_owner_proof: runtime_types::sp_core::Void,
                }
                impl ::subxt::blocks::StaticExtrinsic for ReportEquivocationUnsigned {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "report_equivocation_unsigned";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct NoteStalled {
                    pub delay: ::core::primitive::u32,
                    pub best_finalized_block_number: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for NoteStalled {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "note_stalled";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::tx::Payload<types::ReportEquivocation> {
                    ::subxt::tx::Payload::new_static(
                        "Grandpa",
                        "report_equivocation",
                        types::ReportEquivocation {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        },
                        [
                            73u8, 240u8, 131u8, 56u8, 54u8, 150u8, 13u8, 201u8, 230u8, 52u8, 71u8,
                            43u8, 28u8, 181u8, 156u8, 145u8, 127u8, 42u8, 13u8, 57u8, 54u8, 163u8,
                            117u8, 191u8, 174u8, 64u8, 229u8, 156u8, 28u8, 181u8, 230u8, 42u8,
                        ],
                    )
                }
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                #[doc = ""]
                #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                #[doc = "if the block author is defined it will be defined as the equivocation"]
                #[doc = "reporter."]
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
                        ::subxt::utils::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::tx::Payload<types::ReportEquivocationUnsigned> {
                    ::subxt::tx::Payload::new_static(
                        "Grandpa",
                        "report_equivocation_unsigned",
                        types::ReportEquivocationUnsigned {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        },
                        [
                            164u8, 101u8, 158u8, 143u8, 211u8, 35u8, 126u8, 243u8, 179u8, 133u8,
                            231u8, 44u8, 17u8, 234u8, 224u8, 191u8, 242u8, 206u8, 8u8, 205u8,
                            191u8, 103u8, 140u8, 184u8, 236u8, 195u8, 162u8, 148u8, 246u8, 147u8,
                            87u8, 247u8,
                        ],
                    )
                }
                #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                #[doc = ""]
                #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                #[doc = "block of all validators of the new authority set."]
                #[doc = ""]
                #[doc = "Only callable by root."]
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::NoteStalled> {
                    ::subxt::tx::Payload::new_static(
                        "Grandpa",
                        "note_stalled",
                        types::NoteStalled {
                            delay,
                            best_finalized_block_number,
                        },
                        [
                            232u8, 162u8, 42u8, 199u8, 101u8, 116u8, 38u8, 27u8, 147u8, 15u8,
                            224u8, 76u8, 229u8, 244u8, 13u8, 49u8, 218u8, 232u8, 253u8, 37u8, 7u8,
                            222u8, 97u8, 158u8, 201u8, 199u8, 169u8, 218u8, 201u8, 136u8, 192u8,
                            128u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "New authority set has been applied."]
            pub struct NewAuthorities {
                pub authority_set: ::std::vec::Vec<(
                    runtime_types::sp_consensus_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            }
            impl ::subxt::events::StaticEvent for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Current authority set has been paused."]
            pub struct Paused;
            impl ::subxt::events::StaticEvent for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Current authority set has been resumed."]
            pub struct Resumed;
            impl ::subxt::events::StaticEvent for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " State of the current authority set."]
                pub fn state(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "State",
                        vec![],
                        [
                            254u8, 81u8, 54u8, 203u8, 26u8, 74u8, 162u8, 215u8, 165u8, 247u8,
                            143u8, 139u8, 242u8, 164u8, 67u8, 27u8, 97u8, 172u8, 66u8, 98u8, 28u8,
                            151u8, 32u8, 38u8, 209u8, 82u8, 41u8, 209u8, 72u8, 3u8, 167u8, 42u8,
                        ],
                    )
                }
                #[doc = " Pending change: (signaled at, scheduled change)."]
                pub fn pending_change(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "PendingChange",
                        vec![],
                        [
                            207u8, 134u8, 15u8, 77u8, 9u8, 253u8, 20u8, 132u8, 226u8, 115u8, 150u8,
                            184u8, 18u8, 15u8, 143u8, 172u8, 71u8, 114u8, 221u8, 162u8, 174u8,
                            205u8, 46u8, 144u8, 70u8, 116u8, 18u8, 105u8, 250u8, 44u8, 75u8, 27u8,
                        ],
                    )
                }
                #[doc = " next block number where we can force a change."]
                pub fn next_forced(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "NextForced",
                        vec![],
                        [
                            3u8, 231u8, 56u8, 18u8, 87u8, 112u8, 227u8, 126u8, 180u8, 131u8, 255u8,
                            141u8, 82u8, 34u8, 61u8, 47u8, 234u8, 37u8, 95u8, 62u8, 33u8, 235u8,
                            231u8, 122u8, 125u8, 8u8, 223u8, 95u8, 255u8, 204u8, 40u8, 97u8,
                        ],
                    )
                }
                #[doc = " `true` if we are currently stalled."]
                pub fn stalled(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "Stalled",
                        vec![],
                        [
                            146u8, 18u8, 59u8, 59u8, 21u8, 246u8, 5u8, 167u8, 221u8, 8u8, 230u8,
                            74u8, 81u8, 217u8, 67u8, 158u8, 136u8, 36u8, 23u8, 106u8, 136u8, 89u8,
                            110u8, 217u8, 31u8, 138u8, 107u8, 251u8, 164u8, 10u8, 119u8, 18u8,
                        ],
                    )
                }
                #[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
                #[doc = " in the \"set\" of Grandpa validators from genesis."]
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "CurrentSetId",
                        vec![],
                        [
                            234u8, 215u8, 218u8, 42u8, 30u8, 76u8, 129u8, 40u8, 125u8, 137u8,
                            207u8, 47u8, 46u8, 213u8, 159u8, 50u8, 175u8, 81u8, 155u8, 123u8,
                            246u8, 175u8, 156u8, 68u8, 22u8, 113u8, 135u8, 137u8, 163u8, 18u8,
                            115u8, 73u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
                #[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
                #[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
                #[doc = " was the owner of a given key on a given session, and what the active set ID was"]
                #[doc = " during that session."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u64>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "SetIdSession",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
                            65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
                            238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
                        ],
                    )
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " This is only used for validating equivocation proofs. An equivocation proof must"]
                #[doc = " contains a key-ownership proof for a given session, therefore we need a way to tie"]
                #[doc = " together sessions and GRANDPA set ids, i.e. we need to validate that a validator"]
                #[doc = " was the owner of a given key on a given session, and what the active set ID was"]
                #[doc = " during that session."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub fn set_id_session_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "SetIdSession",
                        Vec::new(),
                        [
                            47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
                            65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
                            238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Max Authorities in use"]
                pub fn max_authorities(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Grandpa",
                        "MaxAuthorities",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of entries to keep in the set id to session index mapping."]
                #[doc = ""]
                #[doc = " Since the `SetIdSession` map is only used for validating equivocations this"]
                #[doc = " value should relate to the bonding duration of whatever staking system is"]
                #[doc = " being used (if any). If equivocation handling is not enabled then this value"]
                #[doc = " can be zero."]
                pub fn max_set_id_session_entries(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u64> {
                    ::subxt::constants::Address::new_static(
                        "Grandpa",
                        "MaxSetIdSessionEntries",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::pallet_balances::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_balances::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferAllowDeath {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferAllowDeath {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_allow_death";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetBalanceDeprecated {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub new_free: ::core::primitive::u128,
                    #[codec(compact)]
                    pub old_reserved: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetBalanceDeprecated {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "set_balance_deprecated";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceTransfer {
                    pub source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_transfer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferKeepAlive {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_keep_alive";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransferAll {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub keep_alive: ::core::primitive::bool,
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferAll {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_all";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceUnreserve {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub amount: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceUnreserve {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_unreserve";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UpgradeAccounts {
                    pub who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                }
                impl ::subxt::blocks::StaticExtrinsic for UpgradeAccounts {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "upgrade_accounts";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Transfer {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for Transfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ForceSetBalance {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub new_free: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceSetBalance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_set_balance";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                pub fn transfer_allow_death(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::TransferAllowDeath> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_allow_death",
                        types::TransferAllowDeath { dest, value },
                        [
                            113u8, 146u8, 7u8, 108u8, 132u8, 222u8, 204u8, 107u8, 142u8, 47u8,
                            46u8, 72u8, 140u8, 22u8, 128u8, 135u8, 132u8, 10u8, 116u8, 41u8, 253u8,
                            65u8, 140u8, 1u8, 50u8, 153u8, 47u8, 243u8, 210u8, 62u8, 23u8, 181u8,
                        ],
                    )
                }
                #[doc = "Set the regular balance of a given account; it also takes a reserved balance but this"]
                #[doc = "must be the same as the account's current reserved balance."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                #[doc = ""]
                #[doc = "WARNING: This call is DEPRECATED! Use `force_set_balance` instead."]
                pub fn set_balance_deprecated(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    new_free: ::core::primitive::u128,
                    old_reserved: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::SetBalanceDeprecated> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "set_balance_deprecated",
                        types::SetBalanceDeprecated {
                            who,
                            new_free,
                            old_reserved,
                        },
                        [
                            43u8, 86u8, 149u8, 26u8, 133u8, 41u8, 43u8, 16u8, 98u8, 65u8, 244u8,
                            123u8, 233u8, 146u8, 76u8, 116u8, 48u8, 164u8, 23u8, 211u8, 42u8,
                            111u8, 245u8, 16u8, 54u8, 92u8, 163u8, 66u8, 193u8, 58u8, 58u8, 185u8,
                        ],
                    )
                }
                #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                #[doc = "may be specified."]
                pub fn force_transfer(
                    &self,
                    source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::ForceTransfer> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_transfer",
                        types::ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            105u8, 201u8, 253u8, 250u8, 151u8, 193u8, 29u8, 188u8, 132u8, 138u8,
                            84u8, 243u8, 170u8, 22u8, 169u8, 161u8, 202u8, 233u8, 79u8, 122u8,
                            77u8, 198u8, 84u8, 149u8, 151u8, 238u8, 174u8, 179u8, 201u8, 150u8,
                            184u8, 20u8,
                        ],
                    )
                }
                #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                #[doc = "kill the origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                #[doc = ""]
                #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_keep_alive",
                        types::TransferKeepAlive { dest, value },
                        [
                            31u8, 197u8, 106u8, 219u8, 164u8, 234u8, 37u8, 214u8, 112u8, 211u8,
                            149u8, 238u8, 162u8, 234u8, 226u8, 11u8, 182u8, 178u8, 182u8, 19u8,
                            43u8, 172u8, 122u8, 175u8, 16u8, 253u8, 193u8, 116u8, 199u8, 158u8,
                            255u8, 188u8,
                        ],
                    )
                }
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true)."]
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::tx::Payload<types::TransferAll> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_all",
                        types::TransferAll { dest, keep_alive },
                        [
                            167u8, 120u8, 58u8, 200u8, 65u8, 248u8, 240u8, 141u8, 208u8, 240u8,
                            89u8, 13u8, 62u8, 169u8, 228u8, 29u8, 219u8, 56u8, 137u8, 224u8, 16u8,
                            111u8, 13u8, 65u8, 65u8, 84u8, 123u8, 173u8, 12u8, 82u8, 101u8, 226u8,
                        ],
                    )
                }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::ForceUnreserve> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_unreserve",
                        types::ForceUnreserve { who, amount },
                        [
                            244u8, 207u8, 86u8, 147u8, 199u8, 152u8, 130u8, 38u8, 248u8, 32u8,
                            97u8, 0u8, 243u8, 58u8, 74u8, 238u8, 68u8, 144u8, 213u8, 168u8, 21u8,
                            151u8, 62u8, 78u8, 8u8, 159u8, 89u8, 1u8, 255u8, 23u8, 83u8, 18u8,
                        ],
                    )
                }
                #[doc = "Upgrade a specified account."]
                #[doc = ""]
                #[doc = "- `origin`: Must be `Signed`."]
                #[doc = "- `who`: The account to be upgraded."]
                #[doc = ""]
                #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                #[doc = "possibililty of churn)."]
                pub fn upgrade_accounts(
                    &self,
                    who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                ) -> ::subxt::tx::Payload<types::UpgradeAccounts> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "upgrade_accounts",
                        types::UpgradeAccounts { who },
                        [
                            66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
                            233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
                            214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
                        ],
                    )
                }
                #[doc = "Alias for `transfer_allow_death`, provided only for name-wise compatibility."]
                #[doc = ""]
                #[doc = "WARNING: DEPRECATED! Will be released in approximately 3 months."]
                pub fn transfer(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::Transfer> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer",
                        types::Transfer { dest, value },
                        [
                            155u8, 139u8, 211u8, 134u8, 163u8, 226u8, 249u8, 125u8, 15u8, 236u8,
                            254u8, 100u8, 49u8, 104u8, 68u8, 71u8, 121u8, 120u8, 66u8, 159u8, 49u8,
                            1u8, 125u8, 223u8, 43u8, 174u8, 19u8, 186u8, 110u8, 190u8, 87u8, 6u8,
                        ],
                    )
                }
                #[doc = "Set the regular balance of a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn force_set_balance(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    new_free: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::ForceSetBalance> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_set_balance",
                        types::ForceSetBalance { who, new_free },
                        [
                            22u8, 6u8, 147u8, 252u8, 116u8, 39u8, 228u8, 198u8, 229u8, 92u8, 141u8,
                            236u8, 192u8, 108u8, 45u8, 242u8, 100u8, 148u8, 47u8, 5u8, 249u8, 49u8,
                            81u8, 156u8, 81u8, 79u8, 216u8, 6u8, 15u8, 192u8, 97u8, 65u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: ::subxt::utils::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: ::subxt::utils::AccountId32,
                pub to: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: ::subxt::utils::AccountId32,
                pub free: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: ::subxt::utils::AccountId32,
                pub to: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was minted into an account."]
            pub struct Minted {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Minted {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Minted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was burned from an account."]
            pub struct Burned {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Burned {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was suspended from an account (it can be restored later)."]
            pub struct Suspended {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Suspended {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Suspended";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some amount was restored into an account."]
            pub struct Restored {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Restored {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Restored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "An account was upgraded."]
            pub struct Upgraded {
                pub who: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Upgraded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Upgraded";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
            pub struct Issued {
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Issued {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
            pub struct Rescinded {
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Rescinded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Rescinded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was locked."]
            pub struct Locked {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Locked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Locked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was unlocked."]
            pub struct Unlocked {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Unlocked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was frozen."]
            pub struct Frozen {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Frozen {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Some balance was thawed."]
            pub struct Thawed {
                pub who: ::subxt::utils::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for Thawed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Thawed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The total units issued in the system."]
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "TotalIssuance",
                        vec![],
                        [
                            116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
                            171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
                            255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
                            185u8,
                        ],
                    )
                }
                #[doc = " The total units of outstanding deactivated balance in the system."]
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "InactiveIssuance",
                        vec![],
                        [
                            212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
                            249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
                            30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            47u8, 253u8, 83u8, 165u8, 18u8, 176u8, 62u8, 239u8, 78u8, 85u8, 231u8,
                            235u8, 157u8, 145u8, 251u8, 35u8, 225u8, 171u8, 82u8, 167u8, 68u8,
                            206u8, 28u8, 169u8, 8u8, 93u8, 169u8, 101u8, 180u8, 206u8, 231u8,
                            143u8,
                        ],
                    )
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub fn account_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Account",
                        Vec::new(),
                        [
                            47u8, 253u8, 83u8, 165u8, 18u8, 176u8, 62u8, 239u8, 78u8, 85u8, 231u8,
                            235u8, 157u8, 145u8, 251u8, 35u8, 225u8, 171u8, 82u8, 167u8, 68u8,
                            206u8, 28u8, 169u8, 8u8, 93u8, 169u8, 101u8, 180u8, 206u8, 231u8,
                            143u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Locks",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            44u8, 44u8, 48u8, 20u8, 121u8, 168u8, 200u8, 87u8, 205u8, 172u8, 111u8,
                            208u8, 62u8, 243u8, 225u8, 223u8, 181u8, 36u8, 197u8, 9u8, 52u8, 182u8,
                            113u8, 55u8, 126u8, 164u8, 82u8, 209u8, 151u8, 126u8, 186u8, 85u8,
                        ],
                    )
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub fn locks_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Locks",
                        Vec::new(),
                        [
                            44u8, 44u8, 48u8, 20u8, 121u8, 168u8, 200u8, 87u8, 205u8, 172u8, 111u8,
                            208u8, 62u8, 243u8, 225u8, 223u8, 181u8, 36u8, 197u8, 9u8, 52u8, 182u8,
                            113u8, 55u8, 126u8, 164u8, 82u8, 209u8, 151u8, 126u8, 186u8, 85u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Reserves",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            192u8, 99u8, 91u8, 129u8, 195u8, 73u8, 153u8, 126u8, 82u8, 52u8, 56u8,
                            85u8, 105u8, 178u8, 113u8, 101u8, 229u8, 37u8, 242u8, 174u8, 166u8,
                            244u8, 68u8, 173u8, 14u8, 225u8, 172u8, 70u8, 181u8, 211u8, 165u8,
                            134u8,
                        ],
                    )
                }
                #[doc = " Named reserves on some account balances."]
                pub fn reserves_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Reserves",
                        Vec::new(),
                        [
                            192u8, 99u8, 91u8, 129u8, 195u8, 73u8, 153u8, 126u8, 82u8, 52u8, 56u8,
                            85u8, 105u8, 178u8, 113u8, 101u8, 229u8, 37u8, 242u8, 174u8, 166u8,
                            244u8, 68u8, 173u8, 14u8, 225u8, 172u8, 70u8, 181u8, 211u8, 165u8,
                            134u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            runtime_types::local_runtime::HoldReason,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Holds",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            153u8, 100u8, 207u8, 189u8, 31u8, 85u8, 211u8, 112u8, 89u8, 160u8,
                            255u8, 191u8, 166u8, 233u8, 241u8, 24u8, 54u8, 100u8, 89u8, 187u8,
                            130u8, 214u8, 253u8, 236u8, 137u8, 27u8, 169u8, 244u8, 108u8, 68u8,
                            9u8, 250u8,
                        ],
                    )
                }
                #[doc = " Holds on account balances."]
                pub fn holds_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            runtime_types::local_runtime::HoldReason,
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Holds",
                        Vec::new(),
                        [
                            153u8, 100u8, 207u8, 189u8, 31u8, 85u8, 211u8, 112u8, 89u8, 160u8,
                            255u8, 191u8, 166u8, 233u8, 241u8, 24u8, 54u8, 100u8, 89u8, 187u8,
                            130u8, 214u8, 253u8, 236u8, 137u8, 27u8, 169u8, 244u8, 108u8, 68u8,
                            9u8, 250u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Freezes",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
                #[doc = " Freeze locks on account balances."]
                pub fn freezes_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            (),
                            ::core::primitive::u128,
                        >,
                    >,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Freezes",
                        Vec::new(),
                        [
                            69u8, 49u8, 165u8, 76u8, 135u8, 142u8, 179u8, 118u8, 50u8, 109u8, 53u8,
                            112u8, 110u8, 94u8, 30u8, 93u8, 173u8, 38u8, 27u8, 142u8, 19u8, 5u8,
                            163u8, 4u8, 68u8, 218u8, 179u8, 224u8, 118u8, 218u8, 115u8, 64u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The minimum amount required to keep an account open. MUST BE GREATER THAN ZERO!"]
                #[doc = ""]
                #[doc = " If you *really* need it to be zero, you can enable the feature `insecure_zero_ed` for"]
                #[doc = " this pallet. However, you do so at your own risk: this will open up a major DoS vector."]
                #[doc = " In case you have multiple sources of provider references, you may also get unexpected"]
                #[doc = " behaviour if you set this to zero."]
                #[doc = ""]
                #[doc = " Bottom line: Do yourself a favour and make it at least one!"]
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of holds that can exist on an account at any time."]
                pub fn max_holds(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxHolds",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum number of individual freeze locks that can exist on an account at any time."]
                pub fn max_freezes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxFreezes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
            #[doc = "has been paid by `who`."]
            pub struct TransactionFeePaid {
                pub who: ::subxt::utils::AccountId32,
                pub actual_fee: ::core::primitive::u128,
                pub tip: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for TransactionFeePaid {
                const PALLET: &'static str = "TransactionPayment";
                const EVENT: &'static str = "TransactionFeePaid";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        vec![],
                        [
                            247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
                            147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
                            159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
                            197u8,
                        ],
                    )
                }
                pub fn storage_version(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TransactionPayment",
                        "StorageVersion",
                        vec![],
                        [
                            105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
                            178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
                            251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
                            144u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u8> {
                    ::subxt::constants::Address::new_static(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "Error for the Sudo pallet"]
        pub type Error = runtime_types::pallet_sudo::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_sudo::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Sudo {
                    pub call: ::std::boxed::Box<runtime_types::local_runtime::RuntimeCall>,
                }
                impl ::subxt::blocks::StaticExtrinsic for Sudo {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SudoUncheckedWeight {
                    pub call: ::std::boxed::Box<runtime_types::local_runtime::RuntimeCall>,
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for SudoUncheckedWeight {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_unchecked_weight";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetKey {
                    pub new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "set_key";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SudoAs {
                    pub who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub call: ::std::boxed::Box<runtime_types::local_runtime::RuntimeCall>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SudoAs {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_as";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- O(1)."]
                pub fn sudo(
                    &self,
                    call: runtime_types::local_runtime::RuntimeCall,
                ) -> ::subxt::tx::Payload<types::Sudo> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "sudo",
                        types::Sudo {
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            237u8, 184u8, 180u8, 39u8, 11u8, 177u8, 75u8, 238u8, 248u8, 97u8, 93u8,
                            100u8, 218u8, 216u8, 3u8, 24u8, 245u8, 38u8, 21u8, 16u8, 199u8, 220u8,
                            119u8, 158u8, 55u8, 59u8, 43u8, 25u8, 88u8, 189u8, 211u8, 145u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- O(1)."]
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::local_runtime::RuntimeCall,
                    weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::SudoUncheckedWeight> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "sudo_unchecked_weight",
                        types::SudoUncheckedWeight {
                            call: ::std::boxed::Box::new(call),
                            weight,
                        },
                        [
                            179u8, 31u8, 45u8, 24u8, 215u8, 211u8, 249u8, 73u8, 114u8, 168u8,
                            128u8, 220u8, 121u8, 4u8, 133u8, 206u8, 68u8, 49u8, 194u8, 242u8, 54u8,
                            47u8, 172u8, 53u8, 198u8, 75u8, 174u8, 117u8, 154u8, 230u8, 254u8,
                            44u8,
                        ],
                    )
                }
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- O(1)."]
                pub fn set_key(
                    &self,
                    new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::Payload<types::SetKey> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "set_key",
                        types::SetKey { new },
                        [
                            196u8, 87u8, 177u8, 124u8, 216u8, 66u8, 124u8, 56u8, 67u8, 36u8, 3u8,
                            124u8, 2u8, 5u8, 175u8, 110u8, 34u8, 76u8, 108u8, 144u8, 19u8, 65u8,
                            84u8, 173u8, 73u8, 179u8, 64u8, 70u8, 190u8, 141u8, 173u8, 195u8,
                        ],
                    )
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- O(1)."]
                pub fn sudo_as(
                    &self,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    call: runtime_types::local_runtime::RuntimeCall,
                ) -> ::subxt::tx::Payload<types::SudoAs> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "sudo_as",
                        types::SudoAs {
                            who,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            252u8, 10u8, 87u8, 171u8, 13u8, 197u8, 29u8, 119u8, 102u8, 209u8,
                            195u8, 6u8, 202u8, 145u8, 31u8, 209u8, 222u8, 136u8, 134u8, 29u8,
                            132u8, 119u8, 3u8, 22u8, 120u8, 137u8, 241u8, 220u8, 114u8, 235u8,
                            200u8, 159u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
            }
            impl ::subxt::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct SudoAsDone {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The `AccountId` of the sudo key."]
                pub fn key(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::subxt::utils::AccountId32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Sudo",
                        "Key",
                        vec![],
                        [
                            72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
                            31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
                            36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod template_module {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::pallet_template::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_template::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DoSomething {
                    pub something: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for DoSomething {
                    const PALLET: &'static str = "TemplateModule";
                    const CALL: &'static str = "do_something";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CauseError;
                impl ::subxt::blocks::StaticExtrinsic for CauseError {
                    const PALLET: &'static str = "TemplateModule";
                    const CALL: &'static str = "cause_error";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "An example dispatchable that takes a singles value as a parameter, writes the value to"]
                #[doc = "storage and emits an event. This function must be dispatched by a signed extrinsic."]
                pub fn do_something(
                    &self,
                    something: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::DoSomething> {
                    ::subxt::tx::Payload::new_static(
                        "TemplateModule",
                        "do_something",
                        types::DoSomething { something },
                        [
                            236u8, 120u8, 161u8, 55u8, 61u8, 88u8, 224u8, 28u8, 174u8, 255u8, 69u8,
                            156u8, 44u8, 11u8, 255u8, 87u8, 183u8, 239u8, 120u8, 48u8, 187u8,
                            157u8, 151u8, 49u8, 105u8, 193u8, 12u8, 1u8, 183u8, 58u8, 73u8, 4u8,
                        ],
                    )
                }
                #[doc = "An example dispatchable that may throw a custom error."]
                pub fn cause_error(&self) -> ::subxt::tx::Payload<types::CauseError> {
                    ::subxt::tx::Payload::new_static(
                        "TemplateModule",
                        "cause_error",
                        types::CauseError {},
                        [
                            29u8, 131u8, 81u8, 134u8, 218u8, 173u8, 12u8, 104u8, 19u8, 10u8, 117u8,
                            192u8, 155u8, 3u8, 171u8, 11u8, 177u8, 248u8, 43u8, 252u8, 4u8, 46u8,
                            244u8, 69u8, 220u8, 51u8, 188u8, 75u8, 165u8, 41u8, 16u8, 188u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_template::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Event documentation should end with an array that provides descriptive names for event"]
            #[doc = "parameters. [something, who]"]
            pub struct SomethingStored {
                pub something: ::core::primitive::u32,
                pub who: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for SomethingStored {
                const PALLET: &'static str = "TemplateModule";
                const EVENT: &'static str = "SomethingStored";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                pub fn something(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TemplateModule",
                        "Something",
                        vec![],
                        [
                            6u8, 80u8, 50u8, 15u8, 9u8, 111u8, 229u8, 130u8, 206u8, 91u8, 123u8,
                            1u8, 210u8, 243u8, 237u8, 67u8, 231u8, 115u8, 161u8, 63u8, 208u8,
                            123u8, 139u8, 55u8, 186u8, 125u8, 167u8, 229u8, 229u8, 107u8, 235u8,
                            56u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod contracts {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::pallet_contracts::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_contracts::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CallOldWeight {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                    #[codec(compact)]
                    pub gas_limit: ::core::primitive::u64,
                    pub storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    pub data: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for CallOldWeight {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "call_old_weight";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InstantiateWithCodeOldWeight {
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                    #[codec(compact)]
                    pub gas_limit: ::core::primitive::u64,
                    pub storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                    pub data: ::std::vec::Vec<::core::primitive::u8>,
                    pub salt: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for InstantiateWithCodeOldWeight {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "instantiate_with_code_old_weight";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InstantiateOldWeight {
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                    #[codec(compact)]
                    pub gas_limit: ::core::primitive::u64,
                    pub storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    pub code_hash: ::subxt::utils::H256,
                    pub data: ::std::vec::Vec<::core::primitive::u8>,
                    pub salt: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for InstantiateOldWeight {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "instantiate_old_weight";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UploadCode {
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                    pub storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    pub determinism: runtime_types::pallet_contracts::wasm::Determinism,
                }
                impl ::subxt::blocks::StaticExtrinsic for UploadCode {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "upload_code";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoveCode {
                    pub code_hash: ::subxt::utils::H256,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveCode {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "remove_code";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetCode {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub code_hash: ::subxt::utils::H256,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCode {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "set_code";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Call {
                    pub dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                    pub gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                    pub storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    pub data: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for Call {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "call";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InstantiateWithCode {
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                    pub gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                    pub storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    pub code: ::std::vec::Vec<::core::primitive::u8>,
                    pub data: ::std::vec::Vec<::core::primitive::u8>,
                    pub salt: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for InstantiateWithCode {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "instantiate_with_code";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Instantiate {
                    #[codec(compact)]
                    pub value: ::core::primitive::u128,
                    pub gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                    pub storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    pub code_hash: ::subxt::utils::H256,
                    pub data: ::std::vec::Vec<::core::primitive::u8>,
                    pub salt: ::std::vec::Vec<::core::primitive::u8>,
                }
                impl ::subxt::blocks::StaticExtrinsic for Instantiate {
                    const PALLET: &'static str = "Contracts";
                    const CALL: &'static str = "instantiate";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Deprecated version if [`Self::call`] for use in an in-storage `Call`."]
                pub fn call_old_weight(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                    gas_limit: ::core::primitive::u64,
                    storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::CallOldWeight> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "call_old_weight",
                        types::CallOldWeight {
                            dest,
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            data,
                        },
                        [
                            162u8, 248u8, 137u8, 129u8, 107u8, 81u8, 255u8, 215u8, 143u8, 105u8,
                            202u8, 248u8, 251u8, 113u8, 168u8, 214u8, 16u8, 161u8, 248u8, 75u8,
                            169u8, 45u8, 84u8, 252u8, 121u8, 221u8, 84u8, 221u8, 218u8, 253u8,
                            146u8, 94u8,
                        ],
                    )
                }
                #[doc = "Deprecated version if [`Self::instantiate_with_code`] for use in an in-storage `Call`."]
                pub fn instantiate_with_code_old_weight(
                    &self,
                    value: ::core::primitive::u128,
                    gas_limit: ::core::primitive::u64,
                    storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                    salt: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::InstantiateWithCodeOldWeight> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "instantiate_with_code_old_weight",
                        types::InstantiateWithCodeOldWeight {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code,
                            data,
                            salt,
                        },
                        [
                            136u8, 86u8, 215u8, 54u8, 92u8, 118u8, 161u8, 244u8, 32u8, 166u8,
                            167u8, 69u8, 231u8, 11u8, 252u8, 63u8, 91u8, 210u8, 252u8, 161u8, 60u8,
                            77u8, 54u8, 69u8, 115u8, 132u8, 146u8, 215u8, 93u8, 239u8, 36u8, 15u8,
                        ],
                    )
                }
                #[doc = "Deprecated version if [`Self::instantiate`] for use in an in-storage `Call`."]
                pub fn instantiate_old_weight(
                    &self,
                    value: ::core::primitive::u128,
                    gas_limit: ::core::primitive::u64,
                    storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    code_hash: ::subxt::utils::H256,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                    salt: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::InstantiateOldWeight> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "instantiate_old_weight",
                        types::InstantiateOldWeight {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code_hash,
                            data,
                            salt,
                        },
                        [
                            10u8, 91u8, 153u8, 191u8, 165u8, 31u8, 75u8, 96u8, 149u8, 28u8, 196u8,
                            95u8, 11u8, 88u8, 227u8, 158u8, 254u8, 202u8, 189u8, 181u8, 224u8,
                            148u8, 204u8, 121u8, 141u8, 133u8, 19u8, 56u8, 54u8, 61u8, 3u8, 108u8,
                        ],
                    )
                }
                #[doc = "Upload new `code` without instantiating a contract from it."]
                #[doc = ""]
                #[doc = "If the code does not already exist a deposit is reserved from the caller"]
                #[doc = "and unreserved only when [`Self::remove_code`] is called. The size of the reserve"]
                #[doc = "depends on the instrumented size of the the supplied `code`."]
                #[doc = ""]
                #[doc = "If the code already exists in storage it will still return `Ok` and upgrades"]
                #[doc = "the in storage version to the current"]
                #[doc = "[`InstructionWeights::version`](InstructionWeights)."]
                #[doc = ""]
                #[doc = "- `determinism`: If this is set to any other value but [`Determinism::Enforced`] then"]
                #[doc = "  the only way to use this code is to delegate call into it from an offchain execution."]
                #[doc = "  Set to [`Determinism::Enforced`] if in doubt."]
                #[doc = ""]
                #[doc = "# Note"]
                #[doc = ""]
                #[doc = "Anyone can instantiate a contract from any uploaded code and thus prevent its removal."]
                #[doc = "To avoid this situation a constructor could employ access control so that it can"]
                #[doc = "only be instantiated by permissioned entities. The same is true when uploading"]
                #[doc = "through [`Self::instantiate_with_code`]."]
                pub fn upload_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                    storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    determinism: runtime_types::pallet_contracts::wasm::Determinism,
                ) -> ::subxt::tx::Payload<types::UploadCode> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "upload_code",
                        types::UploadCode {
                            code,
                            storage_deposit_limit,
                            determinism,
                        },
                        [
                            159u8, 17u8, 234u8, 83u8, 162u8, 68u8, 117u8, 80u8, 64u8, 251u8, 31u8,
                            38u8, 214u8, 227u8, 235u8, 74u8, 97u8, 72u8, 83u8, 197u8, 7u8, 57u8,
                            212u8, 217u8, 219u8, 139u8, 182u8, 248u8, 92u8, 91u8, 56u8, 2u8,
                        ],
                    )
                }
                #[doc = "Remove the code stored under `code_hash` and refund the deposit to its owner."]
                #[doc = ""]
                #[doc = "A code can only be removed by its original uploader (its owner) and only if it is"]
                #[doc = "not used by any contract."]
                pub fn remove_code(
                    &self,
                    code_hash: ::subxt::utils::H256,
                ) -> ::subxt::tx::Payload<types::RemoveCode> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "remove_code",
                        types::RemoveCode { code_hash },
                        [
                            99u8, 184u8, 12u8, 208u8, 123u8, 158u8, 140u8, 21u8, 190u8, 152u8,
                            95u8, 79u8, 217u8, 131u8, 161u8, 160u8, 21u8, 56u8, 167u8, 27u8, 90u8,
                            255u8, 75u8, 0u8, 133u8, 111u8, 119u8, 217u8, 157u8, 67u8, 238u8, 69u8,
                        ],
                    )
                }
                #[doc = "Privileged function that changes the code of an existing contract."]
                #[doc = ""]
                #[doc = "This takes care of updating refcounts and all other necessary operations. Returns"]
                #[doc = "an error if either the `code_hash` or `dest` do not exist."]
                #[doc = ""]
                #[doc = "# Note"]
                #[doc = ""]
                #[doc = "This does **not** change the address of the contract in question. This means"]
                #[doc = "that the contract address is no longer derived from its code hash after calling"]
                #[doc = "this dispatchable."]
                pub fn set_code(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    code_hash: ::subxt::utils::H256,
                ) -> ::subxt::tx::Payload<types::SetCode> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "set_code",
                        types::SetCode { dest, code_hash },
                        [
                            224u8, 107u8, 50u8, 5u8, 53u8, 18u8, 63u8, 203u8, 203u8, 200u8, 136u8,
                            31u8, 242u8, 6u8, 250u8, 31u8, 194u8, 53u8, 185u8, 82u8, 72u8, 26u8,
                            72u8, 47u8, 98u8, 138u8, 43u8, 131u8, 87u8, 214u8, 151u8, 88u8,
                        ],
                    )
                }
                #[doc = "Makes a call to an account, optionally transferring some balance."]
                #[doc = ""]
                #[doc = "# Parameters"]
                #[doc = ""]
                #[doc = "* `dest`: Address of the contract to call."]
                #[doc = "* `value`: The balance to transfer from the `origin` to `dest`."]
                #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged from the"]
                #[doc = "  caller to pay for the storage consumed."]
                #[doc = "* `data`: The input data to pass to the contract."]
                #[doc = ""]
                #[doc = "* If the account is a smart-contract account, the associated code will be"]
                #[doc = "executed and any value will be transferred."]
                #[doc = "* If the account is a regular account, any value will be transferred."]
                #[doc = "* If no account exists and the call value is not less than `existential_deposit`,"]
                #[doc = "a regular account will be created and any value will be transferred."]
                pub fn call(
                    &self,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    value: ::core::primitive::u128,
                    gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                    storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::Call> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "call",
                        types::Call {
                            dest,
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            data,
                        },
                        [
                            254u8, 50u8, 72u8, 79u8, 175u8, 249u8, 26u8, 84u8, 103u8, 167u8, 127u8,
                            48u8, 133u8, 11u8, 102u8, 69u8, 101u8, 192u8, 39u8, 59u8, 97u8, 173u8,
                            166u8, 21u8, 63u8, 202u8, 127u8, 201u8, 159u8, 148u8, 49u8, 133u8,
                        ],
                    )
                }
                #[doc = "Instantiates a new contract from the supplied `code` optionally transferring"]
                #[doc = "some balance."]
                #[doc = ""]
                #[doc = "This dispatchable has the same effect as calling [`Self::upload_code`] +"]
                #[doc = "[`Self::instantiate`]. Bundling them together provides efficiency gains. Please"]
                #[doc = "also check the documentation of [`Self::upload_code`]."]
                #[doc = ""]
                #[doc = "# Parameters"]
                #[doc = ""]
                #[doc = "* `value`: The balance to transfer from the `origin` to the newly created contract."]
                #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged/reserved"]
                #[doc = "  from the caller to pay for the storage consumed."]
                #[doc = "* `code`: The contract code to deploy in raw bytes."]
                #[doc = "* `data`: The input data to pass to the contract constructor."]
                #[doc = "* `salt`: Used for the address derivation. See [`Pallet::contract_address`]."]
                #[doc = ""]
                #[doc = "Instantiation is executed as follows:"]
                #[doc = ""]
                #[doc = "- The supplied `code` is instrumented, deployed, and a `code_hash` is created for that"]
                #[doc = "  code."]
                #[doc = "- If the `code_hash` already exists on the chain the underlying `code` will be shared."]
                #[doc = "- The destination address is computed based on the sender, code_hash and the salt."]
                #[doc = "- The smart-contract account is created at the computed address."]
                #[doc = "- The `value` is transferred to the new account."]
                #[doc = "- The `deploy` function is executed in the context of the newly-created account."]
                pub fn instantiate_with_code(
                    &self,
                    value: ::core::primitive::u128,
                    gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                    storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                    salt: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::InstantiateWithCode> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "instantiate_with_code",
                        types::InstantiateWithCode {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code,
                            data,
                            salt,
                        },
                        [
                            56u8, 224u8, 6u8, 35u8, 3u8, 79u8, 211u8, 229u8, 187u8, 87u8, 43u8,
                            213u8, 168u8, 55u8, 39u8, 41u8, 142u8, 132u8, 16u8, 161u8, 52u8, 22u8,
                            88u8, 114u8, 208u8, 80u8, 99u8, 221u8, 131u8, 156u8, 61u8, 163u8,
                        ],
                    )
                }
                #[doc = "Instantiates a contract from a previously deployed wasm binary."]
                #[doc = ""]
                #[doc = "This function is identical to [`Self::instantiate_with_code`] but without the"]
                #[doc = "code deployment step. Instead, the `code_hash` of an on-chain deployed wasm binary"]
                #[doc = "must be supplied."]
                pub fn instantiate(
                    &self,
                    value: ::core::primitive::u128,
                    gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                    storage_deposit_limit: ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >,
                    code_hash: ::subxt::utils::H256,
                    data: ::std::vec::Vec<::core::primitive::u8>,
                    salt: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::tx::Payload<types::Instantiate> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "instantiate",
                        types::Instantiate {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code_hash,
                            data,
                            salt,
                        },
                        [
                            185u8, 177u8, 11u8, 254u8, 229u8, 161u8, 133u8, 235u8, 255u8, 228u8,
                            121u8, 19u8, 79u8, 20u8, 50u8, 101u8, 203u8, 69u8, 92u8, 48u8, 93u8,
                            152u8, 61u8, 60u8, 168u8, 68u8, 177u8, 187u8, 156u8, 102u8, 210u8,
                            160u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_contracts::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contract deployed by address at the specified address."]
            pub struct Instantiated {
                pub deployer: ::subxt::utils::AccountId32,
                pub contract: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Instantiated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Instantiated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contract has been removed."]
            #[doc = ""]
            #[doc = "# Note"]
            #[doc = ""]
            #[doc = "The only way for a contract to be removed and emitting this event is by calling"]
            #[doc = "`seal_terminate`."]
            pub struct Terminated {
                pub contract: ::subxt::utils::AccountId32,
                pub beneficiary: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Terminated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Terminated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Code with the specified hash has been stored."]
            pub struct CodeStored {
                pub code_hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for CodeStored {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "CodeStored";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A custom event emitted by the contract."]
            pub struct ContractEmitted {
                pub contract: ::subxt::utils::AccountId32,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::events::StaticEvent for ContractEmitted {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "ContractEmitted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A code with the specified hash was removed."]
            pub struct CodeRemoved {
                pub code_hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for CodeRemoved {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "CodeRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A contract's code was updated."]
            pub struct ContractCodeUpdated {
                pub contract: ::subxt::utils::AccountId32,
                pub new_code_hash: ::subxt::utils::H256,
                pub old_code_hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for ContractCodeUpdated {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "ContractCodeUpdated";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A contract was called either by a plain account or another contract."]
            #[doc = ""]
            #[doc = "# Note"]
            #[doc = ""]
            #[doc = "Please keep in mind that like all events this is only emitted for successful"]
            #[doc = "calls. This is because on failure all storage changes including events are"]
            #[doc = "rolled back."]
            pub struct Called {
                pub caller: ::subxt::utils::AccountId32,
                pub contract: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for Called {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "Called";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A contract delegate called a code hash."]
            #[doc = ""]
            #[doc = "# Note"]
            #[doc = ""]
            #[doc = "Please keep in mind that like all events this is only emitted for successful"]
            #[doc = "calls. This is because on failure all storage changes including events are"]
            #[doc = "rolled back."]
            pub struct DelegateCalled {
                pub contract: ::subxt::utils::AccountId32,
                pub code_hash: ::subxt::utils::H256,
            }
            impl ::subxt::events::StaticEvent for DelegateCalled {
                const PALLET: &'static str = "Contracts";
                const EVENT: &'static str = "DelegateCalled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " A mapping from an original code hash to the original code, untouched by instrumentation."]
                pub fn pristine_code(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "PristineCode",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            57u8, 79u8, 159u8, 232u8, 131u8, 156u8, 213u8, 30u8, 6u8, 255u8, 44u8,
                            49u8, 149u8, 175u8, 120u8, 125u8, 44u8, 238u8, 23u8, 117u8, 1u8, 125u8,
                            199u8, 29u8, 72u8, 97u8, 94u8, 163u8, 202u8, 120u8, 207u8, 123u8,
                        ],
                    )
                }
                #[doc = " A mapping from an original code hash to the original code, untouched by instrumentation."]
                pub fn pristine_code_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "PristineCode",
                        Vec::new(),
                        [
                            57u8, 79u8, 159u8, 232u8, 131u8, 156u8, 213u8, 30u8, 6u8, 255u8, 44u8,
                            49u8, 149u8, 175u8, 120u8, 125u8, 44u8, 238u8, 23u8, 117u8, 1u8, 125u8,
                            199u8, 29u8, 72u8, 97u8, 94u8, 163u8, 202u8, 120u8, 207u8, 123u8,
                        ],
                    )
                }
                #[doc = " A mapping between an original code hash and instrumented wasm code, ready for execution."]
                pub fn code_storage(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_contracts::wasm::PrefabWasmModule,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "CodeStorage",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            84u8, 245u8, 130u8, 92u8, 142u8, 214u8, 246u8, 22u8, 10u8, 27u8, 171u8,
                            126u8, 20u8, 40u8, 23u8, 91u8, 90u8, 203u8, 148u8, 105u8, 111u8, 12u8,
                            57u8, 102u8, 183u8, 182u8, 186u8, 147u8, 127u8, 230u8, 110u8, 180u8,
                        ],
                    )
                }
                #[doc = " A mapping between an original code hash and instrumented wasm code, ready for execution."]
                pub fn code_storage_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_contracts::wasm::PrefabWasmModule,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "CodeStorage",
                        Vec::new(),
                        [
                            84u8, 245u8, 130u8, 92u8, 142u8, 214u8, 246u8, 22u8, 10u8, 27u8, 171u8,
                            126u8, 20u8, 40u8, 23u8, 91u8, 90u8, 203u8, 148u8, 105u8, 111u8, 12u8,
                            57u8, 102u8, 183u8, 182u8, 186u8, 147u8, 127u8, 230u8, 110u8, 180u8,
                        ],
                    )
                }
                #[doc = " A mapping between an original code hash and its owner information."]
                pub fn owner_info_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::H256>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_contracts::wasm::OwnerInfo,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "OwnerInfoOf",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            2u8, 159u8, 98u8, 144u8, 186u8, 85u8, 85u8, 97u8, 63u8, 233u8, 251u8,
                            171u8, 202u8, 194u8, 246u8, 211u8, 136u8, 234u8, 138u8, 208u8, 25u8,
                            14u8, 63u8, 211u8, 229u8, 54u8, 33u8, 57u8, 69u8, 87u8, 57u8, 39u8,
                        ],
                    )
                }
                #[doc = " A mapping between an original code hash and its owner information."]
                pub fn owner_info_of_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_contracts::wasm::OwnerInfo,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "OwnerInfoOf",
                        Vec::new(),
                        [
                            2u8, 159u8, 98u8, 144u8, 186u8, 85u8, 85u8, 97u8, 63u8, 233u8, 251u8,
                            171u8, 202u8, 194u8, 246u8, 211u8, 136u8, 234u8, 138u8, 208u8, 25u8,
                            14u8, 63u8, 211u8, 229u8, 54u8, 33u8, 57u8, 69u8, 87u8, 57u8, 39u8,
                        ],
                    )
                }
                #[doc = " This is a **monotonic** counter incremented on contract instantiation."]
                #[doc = ""]
                #[doc = " This is used in order to generate unique trie ids for contracts."]
                #[doc = " The trie id of a new contract is calculated from hash(account_id, nonce)."]
                #[doc = " The nonce is required because otherwise the following sequence would lead to"]
                #[doc = " a possible collision of storage:"]
                #[doc = ""]
                #[doc = " 1. Create a new contract."]
                #[doc = " 2. Terminate the contract."]
                #[doc = " 3. Immediately recreate the contract with the same account_id."]
                #[doc = ""]
                #[doc = " This is bad because the contents of a trie are deleted lazily and there might be"]
                #[doc = " storage of the old instantiation still in it when the new contract is created. Please"]
                #[doc = " note that we can't replace the counter by the block number because the sequence above"]
                #[doc = " can happen in the same block. We also can't keep the account counter in memory only"]
                #[doc = " because storage is the only way to communicate across different extrinsics in the"]
                #[doc = " same block."]
                #[doc = ""]
                #[doc = " # Note"]
                #[doc = ""]
                #[doc = " Do not use it to determine the number of contracts. It won't be decremented if"]
                #[doc = " a contract is destroyed."]
                pub fn nonce(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u64,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "Nonce",
                        vec![],
                        [
                            47u8, 101u8, 89u8, 252u8, 98u8, 25u8, 178u8, 154u8, 17u8, 57u8, 185u8,
                            10u8, 133u8, 94u8, 73u8, 160u8, 137u8, 150u8, 97u8, 119u8, 8u8, 146u8,
                            149u8, 146u8, 212u8, 60u8, 141u8, 24u8, 124u8, 28u8, 57u8, 19u8,
                        ],
                    )
                }
                #[doc = " The code associated with a given account."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn contract_info_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_contracts::storage::ContractInfo,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "ContractInfoOf",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            127u8, 10u8, 55u8, 188u8, 167u8, 57u8, 228u8, 152u8, 70u8, 86u8, 231u8,
                            205u8, 114u8, 129u8, 17u8, 156u8, 245u8, 213u8, 186u8, 159u8, 146u8,
                            81u8, 7u8, 62u8, 167u8, 134u8, 131u8, 33u8, 42u8, 149u8, 47u8, 231u8,
                        ],
                    )
                }
                #[doc = " The code associated with a given account."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: SAFE since `AccountId` is a secure hash."]
                pub fn contract_info_of_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_contracts::storage::ContractInfo,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "ContractInfoOf",
                        Vec::new(),
                        [
                            127u8, 10u8, 55u8, 188u8, 167u8, 57u8, 228u8, 152u8, 70u8, 86u8, 231u8,
                            205u8, 114u8, 129u8, 17u8, 156u8, 245u8, 213u8, 186u8, 159u8, 146u8,
                            81u8, 7u8, 62u8, 167u8, 134u8, 131u8, 33u8, 42u8, 149u8, 47u8, 231u8,
                        ],
                    )
                }
                #[doc = " Evicted contracts that await child trie deletion."]
                #[doc = ""]
                #[doc = " Child trie deletion is a heavy operation depending on the amount of storage items"]
                #[doc = " stored in said trie. Therefore this operation is performed lazily in `on_idle`."]
                pub fn deletion_queue(
                    &self,
                    _0: impl ::std::borrow::Borrow<::core::primitive::u32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "DeletionQueue",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            233u8, 193u8, 191u8, 44u8, 151u8, 46u8, 124u8, 188u8, 132u8, 227u8,
                            107u8, 210u8, 37u8, 110u8, 172u8, 95u8, 12u8, 114u8, 63u8, 83u8, 60u8,
                            163u8, 58u8, 174u8, 160u8, 47u8, 198u8, 156u8, 216u8, 182u8, 65u8,
                            229u8,
                        ],
                    )
                }
                #[doc = " Evicted contracts that await child trie deletion."]
                #[doc = ""]
                #[doc = " Child trie deletion is a heavy operation depending on the amount of storage items"]
                #[doc = " stored in said trie. Therefore this operation is performed lazily in `on_idle`."]
                pub fn deletion_queue_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "DeletionQueue",
                        Vec::new(),
                        [
                            233u8, 193u8, 191u8, 44u8, 151u8, 46u8, 124u8, 188u8, 132u8, 227u8,
                            107u8, 210u8, 37u8, 110u8, 172u8, 95u8, 12u8, 114u8, 63u8, 83u8, 60u8,
                            163u8, 58u8, 174u8, 160u8, 47u8, 198u8, 156u8, 216u8, 182u8, 65u8,
                            229u8,
                        ],
                    )
                }
                #[doc = " A pair of monotonic counters used to track the latest contract marked for deletion"]
                #[doc = " and the latest deleted contract in queue."]
                pub fn deletion_queue_counter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_contracts::storage::DeletionQueueManager,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "DeletionQueueCounter",
                        vec![],
                        [
                            110u8, 62u8, 69u8, 98u8, 41u8, 1u8, 189u8, 178u8, 149u8, 34u8, 42u8,
                            136u8, 127u8, 110u8, 206u8, 2u8, 74u8, 96u8, 188u8, 210u8, 78u8, 37u8,
                            70u8, 251u8, 37u8, 233u8, 26u8, 71u8, 224u8, 116u8, 101u8, 128u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " Cost schedule and limits."]
                pub fn schedule(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::pallet_contracts::schedule::Schedule>
                {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "Schedule",
                        [
                            139u8, 5u8, 62u8, 200u8, 171u8, 157u8, 124u8, 28u8, 16u8, 17u8, 103u8,
                            180u8, 59u8, 98u8, 158u8, 106u8, 43u8, 52u8, 111u8, 249u8, 122u8,
                            137u8, 34u8, 69u8, 142u8, 174u8, 142u8, 223u8, 32u8, 102u8, 189u8,
                            75u8,
                        ],
                    )
                }
                #[doc = " The amount of balance a caller has to pay for each byte of storage."]
                #[doc = ""]
                #[doc = " # Note"]
                #[doc = ""]
                #[doc = " Changing this value for an existing chain might need a storage migration."]
                pub fn deposit_per_byte(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "DepositPerByte",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " Fallback value to limit the storage deposit if it's not being set by the caller."]
                pub fn default_deposit_limit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "DefaultDepositLimit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount of balance a caller has to pay for each storage item."]
                #[doc = ""]
                #[doc = " # Note"]
                #[doc = ""]
                #[doc = " Changing this value for an existing chain might need a storage migration."]
                pub fn deposit_per_item(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "DepositPerItem",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum length of a contract code in bytes. This limit applies to the instrumented"]
                #[doc = " version of the code. Therefore `instantiate_with_code` can fail even when supplying"]
                #[doc = " a wasm binary below this maximum size."]
                #[doc = ""]
                #[doc = " The value should be chosen carefully taking into the account the overall memory limit"]
                #[doc = " your runtime has, as well as the [maximum allowed callstack"]
                #[doc = " depth](#associatedtype.CallStack). Look into the `integrity_test()` for some insights."]
                pub fn max_code_len(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "MaxCodeLen",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " The maximum allowable length in bytes for storage keys."]
                pub fn max_storage_key_len(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "MaxStorageKeyLen",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Make contract callable functions marked as `#[unstable]` available."]
                #[doc = ""]
                #[doc = " Contracts that use `#[unstable]` functions won't be able to be uploaded unless"]
                #[doc = " this is set to `true`. This is only meant for testnets and dev nodes in order to"]
                #[doc = " experiment with new features."]
                #[doc = ""]
                #[doc = " # Warning"]
                #[doc = ""]
                #[doc = " Do **not** set to `true` on productions chains."]
                pub fn unsafe_unstable_interface(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::bool> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "UnsafeUnstableInterface",
                        [
                            165u8, 28u8, 112u8, 190u8, 18u8, 129u8, 182u8, 206u8, 237u8, 1u8, 68u8,
                            252u8, 125u8, 234u8, 185u8, 50u8, 149u8, 164u8, 47u8, 126u8, 134u8,
                            100u8, 14u8, 86u8, 209u8, 39u8, 20u8, 4u8, 233u8, 115u8, 102u8, 131u8,
                        ],
                    )
                }
                #[doc = " The maximum length of the debug buffer in bytes."]
                pub fn max_debug_buffer_len(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "MaxDebugBufferLen",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Series of block headers from the last 81 blocks that acts as random seed material. This"]
                #[doc = " is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of"]
                #[doc = " the oldest hash."]
                pub fn random_material(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::subxt::utils::H256,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "RandomnessCollectiveFlip",
                        "RandomMaterial",
                        vec![],
                        [
                            195u8, 232u8, 244u8, 162u8, 110u8, 137u8, 66u8, 57u8, 51u8, 221u8,
                            143u8, 38u8, 51u8, 183u8, 105u8, 245u8, 175u8, 13u8, 33u8, 192u8, 53u8,
                            16u8, 161u8, 76u8, 219u8, 177u8, 144u8, 192u8, 96u8, 166u8, 117u8,
                            247u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod identity {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::pallet_identity::pallet::Error;
        #[doc = "Identity pallet declaration."]
        pub type Call = runtime_types::pallet_identity::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddRegistrar {
                    pub account: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                }
                impl ::subxt::blocks::StaticExtrinsic for AddRegistrar {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "add_registrar";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetIdentity {
                    pub info:
                        ::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetIdentity {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "set_identity";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetSubs {
                    pub subs: ::std::vec::Vec<(
                        ::subxt::utils::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    )>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetSubs {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "set_subs";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ClearIdentity;
                impl ::subxt::blocks::StaticExtrinsic for ClearIdentity {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "clear_identity";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RequestJudgement {
                    #[codec(compact)]
                    pub reg_index: ::core::primitive::u32,
                    #[codec(compact)]
                    pub max_fee: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for RequestJudgement {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "request_judgement";
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CancelRequest {
                    pub reg_index: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for CancelRequest {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "cancel_request";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetFee {
                    #[codec(compact)]
                    pub index: ::core::primitive::u32,
                    #[codec(compact)]
                    pub fee: ::core::primitive::u128,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetFee {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "set_fee";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetAccountId {
                    #[codec(compact)]
                    pub index: ::core::primitive::u32,
                    pub new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetAccountId {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "set_account_id";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SetFields {
                    #[codec(compact)]
                    pub index: ::core::primitive::u32,
                    pub fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                }
                impl ::subxt::blocks::StaticExtrinsic for SetFields {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "set_fields";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ProvideJudgement {
                    #[codec(compact)]
                    pub reg_index: ::core::primitive::u32,
                    pub target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub judgement:
                        runtime_types::pallet_identity::types::Judgement<::core::primitive::u128>,
                    pub identity: ::subxt::utils::H256,
                }
                impl ::subxt::blocks::StaticExtrinsic for ProvideJudgement {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "provide_judgement";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KillIdentity {
                    pub target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                }
                impl ::subxt::blocks::StaticExtrinsic for KillIdentity {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "kill_identity";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AddSub {
                    pub sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub data: runtime_types::pallet_identity::types::Data,
                }
                impl ::subxt::blocks::StaticExtrinsic for AddSub {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "add_sub";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RenameSub {
                    pub sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pub data: runtime_types::pallet_identity::types::Data,
                }
                impl ::subxt::blocks::StaticExtrinsic for RenameSub {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "rename_sub";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoveSub {
                    pub sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveSub {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "remove_sub";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QuitSub;
                impl ::subxt::blocks::StaticExtrinsic for QuitSub {
                    const PALLET: &'static str = "Identity";
                    const CALL: &'static str = "quit_sub";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Add a registrar to the system."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `T::RegistrarOrigin`."]
                #[doc = ""]
                #[doc = "- `account`: the account of the registrar."]
                #[doc = ""]
                #[doc = "Emits `RegistrarAdded` if successful."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(R)` where `R` registrar-count (governance-bounded and code-bounded)."]
                pub fn add_registrar(
                    &self,
                    account: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::Payload<types::AddRegistrar> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "add_registrar",
                        types::AddRegistrar { account },
                        [
                            151u8, 254u8, 116u8, 168u8, 124u8, 236u8, 0u8, 15u8, 49u8, 117u8,
                            191u8, 181u8, 254u8, 152u8, 163u8, 69u8, 139u8, 157u8, 38u8, 231u8,
                            87u8, 4u8, 227u8, 0u8, 79u8, 188u8, 41u8, 31u8, 120u8, 28u8, 214u8,
                            31u8,
                        ],
                    )
                }
                #[doc = "Set an account's identity information and reserve the appropriate deposit."]
                #[doc = ""]
                #[doc = "If the account already has identity information, the deposit is taken as part payment"]
                #[doc = "for the new deposit."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `info`: The identity information."]
                #[doc = ""]
                #[doc = "Emits `IdentitySet` if successful."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(X + X' + R)`"]
                #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)"]
                #[doc = "  - where `R` judgements-count (registrar-count-bounded)"]
                pub fn set_identity(
                    &self,
                    info: runtime_types::pallet_identity::types::IdentityInfo,
                ) -> ::subxt::tx::Payload<types::SetIdentity> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "set_identity",
                        types::SetIdentity {
                            info: ::std::boxed::Box::new(info),
                        },
                        [
                            205u8, 7u8, 54u8, 226u8, 123u8, 160u8, 173u8, 25u8, 179u8, 93u8, 172u8,
                            37u8, 222u8, 143u8, 209u8, 1u8, 230u8, 32u8, 84u8, 80u8, 110u8, 195u8,
                            87u8, 185u8, 27u8, 31u8, 185u8, 161u8, 154u8, 166u8, 177u8, 190u8,
                        ],
                    )
                }
                #[doc = "Set the sub-accounts of the sender."]
                #[doc = ""]
                #[doc = "Payment: Any aggregate balance reserved by previous `set_subs` calls will be returned"]
                #[doc = "and an amount `SubAccountDeposit` will be reserved for each item in `subs`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                #[doc = "identity."]
                #[doc = ""]
                #[doc = "- `subs`: The identity's (new) sub-accounts."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(P + S)`"]
                #[doc = "  - where `P` old-subs-count (hard- and deposit-bounded)."]
                #[doc = "  - where `S` subs-count (hard- and deposit-bounded)."]
                pub fn set_subs(
                    &self,
                    subs: ::std::vec::Vec<(
                        ::subxt::utils::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    )>,
                ) -> ::subxt::tx::Payload<types::SetSubs> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "set_subs",
                        types::SetSubs { subs },
                        [
                            76u8, 193u8, 92u8, 120u8, 9u8, 99u8, 102u8, 220u8, 177u8, 29u8, 65u8,
                            14u8, 250u8, 101u8, 118u8, 59u8, 251u8, 153u8, 136u8, 141u8, 89u8,
                            250u8, 74u8, 254u8, 111u8, 220u8, 132u8, 228u8, 248u8, 132u8, 177u8,
                            128u8,
                        ],
                    )
                }
                #[doc = "Clear an account's identity info and all sub-accounts and return all deposits."]
                #[doc = ""]
                #[doc = "Payment: All reserved balances on the account are returned."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                #[doc = "identity."]
                #[doc = ""]
                #[doc = "Emits `IdentityCleared` if successful."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(R + S + X)`"]
                #[doc = "  - where `R` registrar-count (governance-bounded)."]
                #[doc = "  - where `S` subs-count (hard- and deposit-bounded)."]
                #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                pub fn clear_identity(&self) -> ::subxt::tx::Payload<types::ClearIdentity> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "clear_identity",
                        types::ClearIdentity {},
                        [
                            43u8, 115u8, 205u8, 44u8, 24u8, 130u8, 220u8, 69u8, 247u8, 176u8,
                            200u8, 175u8, 67u8, 183u8, 36u8, 200u8, 162u8, 132u8, 242u8, 25u8,
                            21u8, 106u8, 197u8, 219u8, 141u8, 51u8, 204u8, 13u8, 191u8, 201u8,
                            31u8, 31u8,
                        ],
                    )
                }
                #[doc = "Request a judgement from a registrar."]
                #[doc = ""]
                #[doc = "Payment: At most `max_fee` will be reserved for payment to the registrar if judgement"]
                #[doc = "given."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a"]
                #[doc = "registered identity."]
                #[doc = ""]
                #[doc = "- `reg_index`: The index of the registrar whose judgement is requested."]
                #[doc = "- `max_fee`: The maximum fee that may be paid. This should just be auto-populated as:"]
                #[doc = ""]
                #[doc = "```nocompile"]
                #[doc = "Self::registrars().get(reg_index).unwrap().fee"]
                #[doc = "```"]
                #[doc = ""]
                #[doc = "Emits `JudgementRequested` if successful."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(R + X)`."]
                #[doc = "  - where `R` registrar-count (governance-bounded)."]
                #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                pub fn request_judgement(
                    &self,
                    reg_index: ::core::primitive::u32,
                    max_fee: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::RequestJudgement> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "request_judgement",
                        types::RequestJudgement { reg_index, max_fee },
                        [
                            83u8, 85u8, 55u8, 184u8, 14u8, 54u8, 49u8, 212u8, 26u8, 148u8, 33u8,
                            147u8, 182u8, 54u8, 180u8, 12u8, 61u8, 179u8, 216u8, 157u8, 103u8,
                            52u8, 120u8, 252u8, 83u8, 203u8, 144u8, 65u8, 15u8, 3u8, 21u8, 33u8,
                        ],
                    )
                }
                #[doc = "Cancel a previous request."]
                #[doc = ""]
                #[doc = "Payment: A previously reserved deposit is returned on success."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a"]
                #[doc = "registered identity."]
                #[doc = ""]
                #[doc = "- `reg_index`: The index of the registrar whose judgement is no longer requested."]
                #[doc = ""]
                #[doc = "Emits `JudgementUnrequested` if successful."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(R + X)`."]
                #[doc = "  - where `R` registrar-count (governance-bounded)."]
                #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                pub fn cancel_request(
                    &self,
                    reg_index: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::CancelRequest> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "cancel_request",
                        types::CancelRequest { reg_index },
                        [
                            81u8, 14u8, 133u8, 219u8, 43u8, 84u8, 163u8, 208u8, 21u8, 185u8, 75u8,
                            117u8, 126u8, 33u8, 210u8, 106u8, 122u8, 210u8, 35u8, 207u8, 104u8,
                            206u8, 41u8, 117u8, 247u8, 108u8, 56u8, 23u8, 123u8, 169u8, 169u8,
                            61u8,
                        ],
                    )
                }
                #[doc = "Set the fee required for a judgement to be requested from a registrar."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                #[doc = "of the registrar whose index is `index`."]
                #[doc = ""]
                #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                #[doc = "- `fee`: the new fee."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(R)`."]
                #[doc = "  - where `R` registrar-count (governance-bounded)."]
                pub fn set_fee(
                    &self,
                    index: ::core::primitive::u32,
                    fee: ::core::primitive::u128,
                ) -> ::subxt::tx::Payload<types::SetFee> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "set_fee",
                        types::SetFee { index, fee },
                        [
                            131u8, 20u8, 17u8, 127u8, 180u8, 65u8, 225u8, 144u8, 193u8, 60u8,
                            131u8, 241u8, 30u8, 149u8, 8u8, 76u8, 29u8, 52u8, 102u8, 108u8, 127u8,
                            130u8, 70u8, 18u8, 94u8, 145u8, 179u8, 109u8, 252u8, 219u8, 58u8,
                            163u8,
                        ],
                    )
                }
                #[doc = "Change the account associated with a registrar."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                #[doc = "of the registrar whose index is `index`."]
                #[doc = ""]
                #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                #[doc = "- `new`: the new account ID."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(R)`."]
                #[doc = "  - where `R` registrar-count (governance-bounded)."]
                pub fn set_account_id(
                    &self,
                    index: ::core::primitive::u32,
                    new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::Payload<types::SetAccountId> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "set_account_id",
                        types::SetAccountId { index, new },
                        [
                            7u8, 72u8, 255u8, 4u8, 182u8, 226u8, 22u8, 255u8, 181u8, 64u8, 165u8,
                            247u8, 102u8, 190u8, 42u8, 236u8, 196u8, 201u8, 26u8, 6u8, 91u8, 113u8,
                            223u8, 237u8, 250u8, 182u8, 40u8, 184u8, 45u8, 255u8, 64u8, 137u8,
                        ],
                    )
                }
                #[doc = "Set the field information for a registrar."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                #[doc = "of the registrar whose index is `index`."]
                #[doc = ""]
                #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                #[doc = "- `fields`: the fields that the registrar concerns themselves with."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(R)`."]
                #[doc = "  - where `R` registrar-count (governance-bounded)."]
                pub fn set_fields(
                    &self,
                    index: ::core::primitive::u32,
                    fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                ) -> ::subxt::tx::Payload<types::SetFields> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "set_fields",
                        types::SetFields { index, fields },
                        [
                            25u8, 129u8, 119u8, 232u8, 18u8, 32u8, 77u8, 23u8, 185u8, 56u8, 32u8,
                            199u8, 74u8, 174u8, 104u8, 203u8, 171u8, 253u8, 19u8, 225u8, 101u8,
                            239u8, 14u8, 242u8, 157u8, 51u8, 203u8, 74u8, 1u8, 65u8, 165u8, 205u8,
                        ],
                    )
                }
                #[doc = "Provide a judgement for an account's identity."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                #[doc = "of the registrar whose index is `reg_index`."]
                #[doc = ""]
                #[doc = "- `reg_index`: the index of the registrar whose judgement is being made."]
                #[doc = "- `target`: the account whose identity the judgement is upon. This must be an account"]
                #[doc = "  with a registered identity."]
                #[doc = "- `judgement`: the judgement of the registrar of index `reg_index` about `target`."]
                #[doc = "- `identity`: The hash of the [`IdentityInfo`] for that the judgement is provided."]
                #[doc = ""]
                #[doc = "Emits `JudgementGiven` if successful."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(R + X)`."]
                #[doc = "  - where `R` registrar-count (governance-bounded)."]
                #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                pub fn provide_judgement(
                    &self,
                    reg_index: ::core::primitive::u32,
                    target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    judgement: runtime_types::pallet_identity::types::Judgement<
                        ::core::primitive::u128,
                    >,
                    identity: ::subxt::utils::H256,
                ) -> ::subxt::tx::Payload<types::ProvideJudgement> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "provide_judgement",
                        types::ProvideJudgement {
                            reg_index,
                            target,
                            judgement,
                            identity,
                        },
                        [
                            84u8, 65u8, 119u8, 246u8, 52u8, 71u8, 44u8, 213u8, 173u8, 21u8, 198u8,
                            91u8, 234u8, 186u8, 176u8, 168u8, 158u8, 232u8, 10u8, 230u8, 15u8,
                            34u8, 27u8, 241u8, 200u8, 58u8, 17u8, 112u8, 211u8, 88u8, 162u8, 228u8,
                        ],
                    )
                }
                #[doc = "Remove an account's identity and sub-account information and slash the deposits."]
                #[doc = ""]
                #[doc = "Payment: Reserved balances from `set_subs` and `set_identity` are slashed and handled by"]
                #[doc = "`Slash`. Verification request deposits are not returned; they should be cancelled"]
                #[doc = "manually using `cancel_request`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must match `T::ForceOrigin`."]
                #[doc = ""]
                #[doc = "- `target`: the account whose identity the judgement is upon. This must be an account"]
                #[doc = "  with a registered identity."]
                #[doc = ""]
                #[doc = "Emits `IdentityKilled` if successful."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(R + S + X)`"]
                #[doc = "  - where `R` registrar-count (governance-bounded)."]
                #[doc = "  - where `S` subs-count (hard- and deposit-bounded)."]
                #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                pub fn kill_identity(
                    &self,
                    target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::Payload<types::KillIdentity> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "kill_identity",
                        types::KillIdentity { target },
                        [
                            154u8, 203u8, 22u8, 126u8, 241u8, 51u8, 109u8, 221u8, 79u8, 203u8,
                            12u8, 113u8, 234u8, 162u8, 125u8, 39u8, 69u8, 105u8, 194u8, 7u8, 254u8,
                            196u8, 66u8, 104u8, 15u8, 41u8, 164u8, 11u8, 55u8, 98u8, 160u8, 175u8,
                        ],
                    )
                }
                #[doc = "Add the given account to the sender's subs."]
                #[doc = ""]
                #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                #[doc = "to the sender."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                #[doc = "sub identity of `sub`."]
                pub fn add_sub(
                    &self,
                    sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    data: runtime_types::pallet_identity::types::Data,
                ) -> ::subxt::tx::Payload<types::AddSub> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "add_sub",
                        types::AddSub { sub, data },
                        [
                            157u8, 159u8, 194u8, 77u8, 105u8, 29u8, 238u8, 209u8, 145u8, 64u8,
                            24u8, 130u8, 102u8, 41u8, 124u8, 74u8, 190u8, 29u8, 142u8, 123u8,
                            131u8, 126u8, 58u8, 129u8, 46u8, 54u8, 160u8, 17u8, 178u8, 72u8, 15u8,
                            145u8,
                        ],
                    )
                }
                #[doc = "Alter the associated name of the given sub-account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                #[doc = "sub identity of `sub`."]
                pub fn rename_sub(
                    &self,
                    sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    data: runtime_types::pallet_identity::types::Data,
                ) -> ::subxt::tx::Payload<types::RenameSub> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "rename_sub",
                        types::RenameSub { sub, data },
                        [
                            100u8, 82u8, 99u8, 104u8, 4u8, 129u8, 166u8, 128u8, 187u8, 80u8, 73u8,
                            57u8, 167u8, 74u8, 155u8, 125u8, 171u8, 229u8, 201u8, 246u8, 145u8,
                            190u8, 222u8, 1u8, 129u8, 139u8, 205u8, 96u8, 186u8, 221u8, 15u8, 35u8,
                        ],
                    )
                }
                #[doc = "Remove the given account from the sender's subs."]
                #[doc = ""]
                #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                #[doc = "to the sender."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                #[doc = "sub identity of `sub`."]
                pub fn remove_sub(
                    &self,
                    sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                ) -> ::subxt::tx::Payload<types::RemoveSub> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "remove_sub",
                        types::RemoveSub { sub },
                        [
                            209u8, 217u8, 16u8, 99u8, 152u8, 250u8, 236u8, 172u8, 139u8, 229u8,
                            246u8, 165u8, 188u8, 59u8, 66u8, 31u8, 105u8, 237u8, 51u8, 218u8,
                            177u8, 108u8, 101u8, 115u8, 190u8, 105u8, 208u8, 241u8, 133u8, 224u8,
                            2u8, 38u8,
                        ],
                    )
                }
                #[doc = "Remove the sender as a sub-account."]
                #[doc = ""]
                #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                #[doc = "to the sender (*not* the original depositor)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                #[doc = "super-identity."]
                #[doc = ""]
                #[doc = "NOTE: This should not normally be used, but is provided in the case that the non-"]
                #[doc = "controller of an account is maliciously registered as a sub-account."]
                pub fn quit_sub(&self) -> ::subxt::tx::Payload<types::QuitSub> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "quit_sub",
                        types::QuitSub {},
                        [
                            147u8, 131u8, 175u8, 171u8, 187u8, 201u8, 240u8, 26u8, 146u8, 224u8,
                            74u8, 166u8, 242u8, 193u8, 204u8, 247u8, 168u8, 93u8, 18u8, 32u8, 27u8,
                            208u8, 149u8, 146u8, 179u8, 172u8, 75u8, 112u8, 84u8, 141u8, 233u8,
                            223u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_identity::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A name was set or reset (which will remove all judgements)."]
            pub struct IdentitySet {
                pub who: ::subxt::utils::AccountId32,
            }
            impl ::subxt::events::StaticEvent for IdentitySet {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentitySet";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A name was cleared, and the given balance returned."]
            pub struct IdentityCleared {
                pub who: ::subxt::utils::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for IdentityCleared {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentityCleared";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A name was removed and the given balance slashed."]
            pub struct IdentityKilled {
                pub who: ::subxt::utils::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for IdentityKilled {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "IdentityKilled";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A judgement was asked from a registrar."]
            pub struct JudgementRequested {
                pub who: ::subxt::utils::AccountId32,
                pub registrar_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for JudgementRequested {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementRequested";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A judgement request was retracted."]
            pub struct JudgementUnrequested {
                pub who: ::subxt::utils::AccountId32,
                pub registrar_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for JudgementUnrequested {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementUnrequested";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A judgement was given by a registrar."]
            pub struct JudgementGiven {
                pub target: ::subxt::utils::AccountId32,
                pub registrar_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for JudgementGiven {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "JudgementGiven";
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A registrar was added."]
            pub struct RegistrarAdded {
                pub registrar_index: ::core::primitive::u32,
            }
            impl ::subxt::events::StaticEvent for RegistrarAdded {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "RegistrarAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A sub-identity was added to an identity and the deposit paid."]
            pub struct SubIdentityAdded {
                pub sub: ::subxt::utils::AccountId32,
                pub main: ::subxt::utils::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for SubIdentityAdded {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityAdded";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A sub-identity was removed from an identity and the deposit freed."]
            pub struct SubIdentityRemoved {
                pub sub: ::subxt::utils::AccountId32,
                pub main: ::subxt::utils::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for SubIdentityRemoved {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityRemoved";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A sub-identity was cleared, and the given deposit repatriated from the"]
            #[doc = "main identity account to the sub-identity account."]
            pub struct SubIdentityRevoked {
                pub sub: ::subxt::utils::AccountId32,
                pub main: ::subxt::utils::AccountId32,
                pub deposit: ::core::primitive::u128,
            }
            impl ::subxt::events::StaticEvent for SubIdentityRevoked {
                const PALLET: &'static str = "Identity";
                const EVENT: &'static str = "SubIdentityRevoked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " Information that is pertinent to identify the entity behind an account."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
                pub fn identity_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_identity::types::Registration<::core::primitive::u128>,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "IdentityOf",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            239u8, 55u8, 5u8, 97u8, 227u8, 243u8, 118u8, 13u8, 98u8, 30u8, 141u8,
                            84u8, 170u8, 90u8, 166u8, 116u8, 17u8, 122u8, 190u8, 76u8, 34u8, 51u8,
                            239u8, 41u8, 14u8, 135u8, 11u8, 164u8, 106u8, 228u8, 48u8, 26u8,
                        ],
                    )
                }
                #[doc = " Information that is pertinent to identify the entity behind an account."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
                pub fn identity_of_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_identity::types::Registration<::core::primitive::u128>,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "IdentityOf",
                        Vec::new(),
                        [
                            239u8, 55u8, 5u8, 97u8, 227u8, 243u8, 118u8, 13u8, 98u8, 30u8, 141u8,
                            84u8, 170u8, 90u8, 166u8, 116u8, 17u8, 122u8, 190u8, 76u8, 34u8, 51u8,
                            239u8, 41u8, 14u8, 135u8, 11u8, 164u8, 106u8, 228u8, 48u8, 26u8,
                        ],
                    )
                }
                #[doc = " The super-identity of an alternative \"sub\" identity together with its name, within that"]
                #[doc = " context. If the account is not some other account's sub-identity, then just `None`."]
                pub fn super_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        ::subxt::utils::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    ),
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "SuperOf",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            51u8, 225u8, 21u8, 92u8, 85u8, 14u8, 14u8, 211u8, 61u8, 99u8, 176u8,
                            236u8, 212u8, 156u8, 103u8, 175u8, 208u8, 105u8, 94u8, 226u8, 136u8,
                            69u8, 162u8, 170u8, 11u8, 116u8, 72u8, 242u8, 119u8, 14u8, 14u8, 142u8,
                        ],
                    )
                }
                #[doc = " The super-identity of an alternative \"sub\" identity together with its name, within that"]
                #[doc = " context. If the account is not some other account's sub-identity, then just `None`."]
                pub fn super_of_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        ::subxt::utils::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    ),
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "SuperOf",
                        Vec::new(),
                        [
                            51u8, 225u8, 21u8, 92u8, 85u8, 14u8, 14u8, 211u8, 61u8, 99u8, 176u8,
                            236u8, 212u8, 156u8, 103u8, 175u8, 208u8, 105u8, 94u8, 226u8, 136u8,
                            69u8, 162u8, 170u8, 11u8, 116u8, 72u8, 242u8, 119u8, 14u8, 14u8, 142u8,
                        ],
                    )
                }
                #[doc = " Alternative \"sub\" identities of this account."]
                #[doc = ""]
                #[doc = " The first item is the deposit, the second is a vector of the accounts."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
                pub fn subs_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        ::core::primitive::u128,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::utils::AccountId32,
                        >,
                    ),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "SubsOf",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            93u8, 124u8, 154u8, 157u8, 159u8, 103u8, 233u8, 225u8, 59u8, 20u8,
                            201u8, 239u8, 128u8, 209u8, 207u8, 38u8, 123u8, 48u8, 119u8, 102u8,
                            88u8, 42u8, 245u8, 187u8, 244u8, 206u8, 124u8, 216u8, 185u8, 155u8,
                            207u8, 0u8,
                        ],
                    )
                }
                #[doc = " Alternative \"sub\" identities of this account."]
                #[doc = ""]
                #[doc = " The first item is the deposit, the second is a vector of the accounts."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
                pub fn subs_of_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    (
                        ::core::primitive::u128,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::utils::AccountId32,
                        >,
                    ),
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "SubsOf",
                        Vec::new(),
                        [
                            93u8, 124u8, 154u8, 157u8, 159u8, 103u8, 233u8, 225u8, 59u8, 20u8,
                            201u8, 239u8, 128u8, 209u8, 207u8, 38u8, 123u8, 48u8, 119u8, 102u8,
                            88u8, 42u8, 245u8, 187u8, 244u8, 206u8, 124u8, 216u8, 185u8, 155u8,
                            207u8, 0u8,
                        ],
                    )
                }
                #[doc = " The set of registrars. Not expected to get very big as can only be added through a"]
                #[doc = " special origin (likely a council motion)."]
                #[doc = ""]
                #[doc = " The index into this can be cast to `RegistrarIndex` to get a valid value."]
                pub fn registrars(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::option::Option<
                            runtime_types::pallet_identity::types::RegistrarInfo<
                                ::core::primitive::u128,
                                ::subxt::utils::AccountId32,
                            >,
                        >,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "Registrars",
                        vec![],
                        [
                            207u8, 253u8, 229u8, 237u8, 228u8, 85u8, 173u8, 74u8, 164u8, 67u8,
                            144u8, 144u8, 5u8, 242u8, 84u8, 187u8, 110u8, 181u8, 2u8, 162u8, 239u8,
                            212u8, 72u8, 233u8, 160u8, 196u8, 121u8, 218u8, 100u8, 0u8, 219u8,
                            181u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The amount held on deposit for a registered identity"]
                pub fn basic_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Identity",
                        "BasicDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount held on deposit per additional field for a registered identity."]
                pub fn field_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Identity",
                        "FieldDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount held on deposit for a registered subaccount. This should account for the fact"]
                #[doc = " that one storage item's value will increase by the size of an account ID, and there will"]
                #[doc = " be another trie item whose value is the size of an account ID plus 32 bytes."]
                pub fn sub_account_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Identity",
                        "SubAccountDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum number of sub-accounts allowed per identified account."]
                pub fn max_sub_accounts(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Identity",
                        "MaxSubAccounts",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Maximum number of additional fields that may be stored in an ID. Needed to bound the I/O"]
                #[doc = " required to access an identity, but can be pretty high."]
                pub fn max_additional_fields(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Identity",
                        "MaxAdditionalFields",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Maxmimum number of registrars allowed in the system. Needed to bound the complexity"]
                #[doc = " of, e.g., updating judgements."]
                pub fn max_registrars(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Identity",
                        "MaxRegistrars",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod multisig {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
        pub type Error = runtime_types::pallet_multisig::pallet::Error;
        #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
        pub type Call = runtime_types::pallet_multisig::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AsMultiThreshold1 {
                    pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    pub call: ::std::boxed::Box<runtime_types::local_runtime::RuntimeCall>,
                }
                impl ::subxt::blocks::StaticExtrinsic for AsMultiThreshold1 {
                    const PALLET: &'static str = "Multisig";
                    const CALL: &'static str = "as_multi_threshold_1";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AsMulti {
                    pub threshold: ::core::primitive::u16,
                    pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    pub maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    pub call: ::std::boxed::Box<runtime_types::local_runtime::RuntimeCall>,
                    pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for AsMulti {
                    const PALLET: &'static str = "Multisig";
                    const CALL: &'static str = "as_multi";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ApproveAsMulti {
                    pub threshold: ::core::primitive::u16,
                    pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    pub maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    pub call_hash: [::core::primitive::u8; 32usize],
                    pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
                }
                impl ::subxt::blocks::StaticExtrinsic for ApproveAsMulti {
                    const PALLET: &'static str = "Multisig";
                    const CALL: &'static str = "approve_as_multi";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CancelAsMulti {
                    pub threshold: ::core::primitive::u16,
                    pub other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    pub timepoint:
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    pub call_hash: [::core::primitive::u8; 32usize],
                }
                impl ::subxt::blocks::StaticExtrinsic for CancelAsMulti {
                    const PALLET: &'static str = "Multisig";
                    const CALL: &'static str = "cancel_as_multi";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Immediately dispatch a multi-signature call using a single approval from the caller."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `other_signatories`: The accounts (other than the sender) who are part of the"]
                #[doc = "multi-signature, but do not participate in the approval process."]
                #[doc = "- `call`: The call to be executed."]
                #[doc = ""]
                #[doc = "Result is equivalent to the dispatched result."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "O(Z + C) where Z is the length of the call and C its execution weight."]
                pub fn as_multi_threshold_1(
                    &self,
                    other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    call: runtime_types::local_runtime::RuntimeCall,
                ) -> ::subxt::tx::Payload<types::AsMultiThreshold1> {
                    ::subxt::tx::Payload::new_static(
                        "Multisig",
                        "as_multi_threshold_1",
                        types::AsMultiThreshold1 {
                            other_signatories,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            238u8, 126u8, 199u8, 143u8, 9u8, 71u8, 121u8, 195u8, 119u8, 128u8,
                            242u8, 175u8, 104u8, 77u8, 115u8, 237u8, 231u8, 183u8, 242u8, 131u8,
                            241u8, 123u8, 238u8, 56u8, 229u8, 160u8, 86u8, 255u8, 230u8, 18u8,
                            149u8, 98u8,
                        ],
                    )
                }
                #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                #[doc = ""]
                #[doc = "If there are enough, then dispatch the call."]
                #[doc = ""]
                #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                #[doc = "is cancelled."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                #[doc = "dispatch. May not be empty."]
                #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                #[doc = "transaction index) of the first approval transaction."]
                #[doc = "- `call`: The call to be executed."]
                #[doc = ""]
                #[doc = "NOTE: Unless this is the final approval, you will generally want to use"]
                #[doc = "`approve_as_multi` instead, since it only requires a hash of the call."]
                #[doc = ""]
                #[doc = "Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise"]
                #[doc = "on success, result is `Ok` and the result from the interior call, if it was executed,"]
                #[doc = "may be found in the deposited `MultisigExecuted` event."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(S + Z + Call)`."]
                #[doc = "- Up to one balance-reserve or unreserve operation."]
                #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                #[doc = "- One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len."]
                #[doc = "- One encode & hash, both of complexity `O(S)`."]
                #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                #[doc = "- One event."]
                #[doc = "- The weight of the `call`."]
                #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                pub fn as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call: runtime_types::local_runtime::RuntimeCall,
                    max_weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::AsMulti> {
                    ::subxt::tx::Payload::new_static(
                        "Multisig",
                        "as_multi",
                        types::AsMulti {
                            threshold,
                            other_signatories,
                            maybe_timepoint,
                            call: ::std::boxed::Box::new(call),
                            max_weight,
                        },
                        [
                            66u8, 174u8, 162u8, 57u8, 156u8, 13u8, 4u8, 158u8, 134u8, 237u8, 217u8,
                            15u8, 46u8, 31u8, 142u8, 206u8, 240u8, 253u8, 64u8, 63u8, 32u8, 241u8,
                            175u8, 75u8, 218u8, 114u8, 29u8, 165u8, 113u8, 122u8, 202u8, 98u8,
                        ],
                    )
                }
                #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                #[doc = ""]
                #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                #[doc = "is cancelled."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                #[doc = "dispatch. May not be empty."]
                #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                #[doc = "transaction index) of the first approval transaction."]
                #[doc = "- `call_hash`: The hash of the call to be executed."]
                #[doc = ""]
                #[doc = "NOTE: If this is the final approval, you will want to use `as_multi` instead."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(S)`."]
                #[doc = "- Up to one balance-reserve or unreserve operation."]
                #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                #[doc = "- One encode & hash, both of complexity `O(S)`."]
                #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                #[doc = "- One event."]
                #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                pub fn approve_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                    max_weight: runtime_types::sp_weights::weight_v2::Weight,
                ) -> ::subxt::tx::Payload<types::ApproveAsMulti> {
                    ::subxt::tx::Payload::new_static(
                        "Multisig",
                        "approve_as_multi",
                        types::ApproveAsMulti {
                            threshold,
                            other_signatories,
                            maybe_timepoint,
                            call_hash,
                            max_weight,
                        },
                        [
                            240u8, 17u8, 138u8, 10u8, 165u8, 3u8, 88u8, 240u8, 11u8, 208u8, 9u8,
                            123u8, 95u8, 53u8, 142u8, 8u8, 30u8, 5u8, 130u8, 205u8, 102u8, 95u8,
                            71u8, 92u8, 184u8, 92u8, 218u8, 224u8, 146u8, 87u8, 93u8, 224u8,
                        ],
                    )
                }
                #[doc = "Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously"]
                #[doc = "for this operation will be unreserved on success."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                #[doc = "dispatch. May not be empty."]
                #[doc = "- `timepoint`: The timepoint (block number and transaction index) of the first approval"]
                #[doc = "transaction for this dispatch."]
                #[doc = "- `call_hash`: The hash of the call to be executed."]
                #[doc = ""]
                #[doc = "## Complexity"]
                #[doc = "- `O(S)`."]
                #[doc = "- Up to one balance-reserve or unreserve operation."]
                #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                #[doc = "- One encode & hash, both of complexity `O(S)`."]
                #[doc = "- One event."]
                #[doc = "- I/O: 1 read `O(S)`, one remove."]
                #[doc = "- Storage: removes one item."]
                pub fn cancel_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    call_hash: [::core::primitive::u8; 32usize],
                ) -> ::subxt::tx::Payload<types::CancelAsMulti> {
                    ::subxt::tx::Payload::new_static(
                        "Multisig",
                        "cancel_as_multi",
                        types::CancelAsMulti {
                            threshold,
                            other_signatories,
                            timepoint,
                            call_hash,
                        },
                        [
                            14u8, 123u8, 126u8, 239u8, 174u8, 101u8, 28u8, 221u8, 117u8, 75u8,
                            82u8, 249u8, 151u8, 59u8, 224u8, 239u8, 54u8, 196u8, 244u8, 46u8, 31u8,
                            218u8, 224u8, 58u8, 146u8, 165u8, 135u8, 101u8, 189u8, 93u8, 149u8,
                            130u8,
                        ],
                    )
                }
            }
        }
        #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
        pub type Event = runtime_types::pallet_multisig::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A new multisig operation has begun."]
            pub struct NewMultisig {
                pub approving: ::subxt::utils::AccountId32,
                pub multisig: ::subxt::utils::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::events::StaticEvent for NewMultisig {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "NewMultisig";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A multisig operation has been approved by someone."]
            pub struct MultisigApproval {
                pub approving: ::subxt::utils::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::utils::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::events::StaticEvent for MultisigApproval {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigApproval";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A multisig operation has been executed."]
            pub struct MultisigExecuted {
                pub approving: ::subxt::utils::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::utils::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::events::StaticEvent for MultisigExecuted {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigExecuted";
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "A multisig operation has been cancelled."]
            pub struct MultisigCancelled {
                pub cancelling: ::subxt::utils::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::utils::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::events::StaticEvent for MultisigCancelled {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigCancelled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " The set of open multisig operations."]
                pub fn multisigs(
                    &self,
                    _0: impl ::std::borrow::Borrow<::subxt::utils::AccountId32>,
                    _1: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_multisig::Multisig<
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::subxt::utils::AccountId32,
                    >,
                    ::subxt::storage::address::Yes,
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Multisig",
                        "Multisigs",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            22u8, 46u8, 92u8, 90u8, 193u8, 51u8, 12u8, 187u8, 247u8, 141u8, 101u8,
                            133u8, 220u8, 5u8, 124u8, 197u8, 149u8, 81u8, 51u8, 194u8, 194u8, 72u8,
                            63u8, 249u8, 227u8, 208u8, 58u8, 253u8, 33u8, 107u8, 10u8, 44u8,
                        ],
                    )
                }
                #[doc = " The set of open multisig operations."]
                pub fn multisigs_root(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::pallet_multisig::Multisig<
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::subxt::utils::AccountId32,
                    >,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Multisig",
                        "Multisigs",
                        Vec::new(),
                        [
                            22u8, 46u8, 92u8, 90u8, 193u8, 51u8, 12u8, 187u8, 247u8, 141u8, 101u8,
                            133u8, 220u8, 5u8, 124u8, 197u8, 149u8, 81u8, 51u8, 194u8, 194u8, 72u8,
                            63u8, 249u8, 227u8, 208u8, 58u8, 253u8, 33u8, 107u8, 10u8, 44u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " The base amount of currency needed to reserve for creating a multisig execution or to"]
                #[doc = " store a dispatch call for later."]
                #[doc = ""]
                #[doc = " This is held for an additional storage item whose value size is"]
                #[doc = " `4 + sizeof((BlockNumber, Balance, AccountId))` bytes and whose key size is"]
                #[doc = " `32 + sizeof(AccountId)` bytes."]
                pub fn deposit_base(&self) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Multisig",
                        "DepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The amount of currency needed per unit threshold when creating a multisig execution."]
                #[doc = ""]
                #[doc = " This is held for adding 32 bytes more into a pre-existing storage value."]
                pub fn deposit_factor(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Multisig",
                        "DepositFactor",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                #[doc = " The maximum amount of signatories allowed in the multisig."]
                pub fn max_signatories(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Multisig",
                        "MaxSignatories",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod dhatu {
        use super::root_mod;
        use super::runtime_types;
        #[doc = "A public part of the pallet."]
        pub type Call = runtime_types::pallet_dhatu::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SubmitData {
                    pub data: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for SubmitData {
                    const PALLET: &'static str = "Dhatu";
                    const CALL: &'static str = "submit_data";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SubmitDataUnsigned {
                    pub block_number: ::core::primitive::u32,
                    pub data: ::core::primitive::u32,
                }
                impl ::subxt::blocks::StaticExtrinsic for SubmitDataUnsigned {
                    const PALLET: &'static str = "Dhatu";
                    const CALL: &'static str = "submit_data_unsigned";
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SubmitDataUnsignedWithSignedPayload {
                    pub data_payload: runtime_types::pallet_dhatu::DataPayload<
                        runtime_types::sp_runtime::MultiSigner,
                        ::core::primitive::u32,
                    >,
                    pub signature: runtime_types::sp_runtime::MultiSignature,
                }
                impl ::subxt::blocks::StaticExtrinsic for SubmitDataUnsignedWithSignedPayload {
                    const PALLET: &'static str = "Dhatu";
                    const CALL: &'static str = "submit_data_unsigned_with_signed_payload";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                #[doc = "Submit new data to the list."]
                #[doc = ""]
                #[doc = "This method is a public function of the module and can be called from within"]
                #[doc = "a transaction. It appends given `data` to current list of data."]
                #[doc = "In our example the `offchain worker` will create, sign & submit a transaction that"]
                #[doc = "calls this function passing the data."]
                #[doc = ""]
                #[doc = "The transaction needs to be signed (see `ensure_signed`) check, so that the caller"]
                #[doc = "pays a fee to execute it."]
                #[doc = "This makes sure that it's not easy (or rather cheap) to attack the chain by submitting"]
                #[doc = "excesive transactions, but note that it doesn't ensure the data oracle is actually"]
                #[doc = "working and receives (and provides) meaningful data."]
                #[doc = "This example is not focused on correctness of the oracle itself, but rather its"]
                #[doc = "purpose is to showcase offchain worker capabilities."]
                pub fn submit_data(
                    &self,
                    data: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::SubmitData> {
                    ::subxt::tx::Payload::new_static(
                        "Dhatu",
                        "submit_data",
                        types::SubmitData { data },
                        [
                            170u8, 151u8, 252u8, 232u8, 50u8, 231u8, 152u8, 196u8, 255u8, 186u8,
                            32u8, 16u8, 213u8, 125u8, 155u8, 59u8, 243u8, 137u8, 32u8, 255u8,
                            230u8, 27u8, 83u8, 92u8, 106u8, 154u8, 76u8, 237u8, 142u8, 128u8,
                            102u8, 106u8,
                        ],
                    )
                }
                #[doc = "Submit new data to the list via unsigned transaction."]
                #[doc = ""]
                #[doc = "Works exactly like the `submit_data` function, but since we allow sending the"]
                #[doc = "transaction without a signature, and hence without paying any fees,"]
                #[doc = "we need a way to make sure that only some transactions are accepted."]
                #[doc = "This function can be called only once every `T::UnsignedInterval` blocks."]
                #[doc = "Transactions that call that function are de-duplicated on the pool level"]
                #[doc = "via `validate_unsigned` implementation and also are rendered invalid if"]
                #[doc = "the function has already been called in current \"session\"."]
                #[doc = ""]
                #[doc = "It's important to specify `weight` for unsigned calls as well, because even though"]
                #[doc = "they don't charge fees, we still don't want a single block to contain unlimited"]
                #[doc = "number of such transactions."]
                #[doc = ""]
                #[doc = "This example is not focused on correctness of the oracle itself, but rather its"]
                #[doc = "purpose is to showcase offchain worker capabilities."]
                pub fn submit_data_unsigned(
                    &self,
                    block_number: ::core::primitive::u32,
                    data: ::core::primitive::u32,
                ) -> ::subxt::tx::Payload<types::SubmitDataUnsigned> {
                    ::subxt::tx::Payload::new_static(
                        "Dhatu",
                        "submit_data_unsigned",
                        types::SubmitDataUnsigned { block_number, data },
                        [
                            161u8, 255u8, 73u8, 83u8, 128u8, 218u8, 178u8, 246u8, 92u8, 241u8,
                            117u8, 207u8, 155u8, 248u8, 251u8, 138u8, 38u8, 206u8, 200u8, 50u8,
                            109u8, 170u8, 185u8, 67u8, 150u8, 42u8, 167u8, 187u8, 47u8, 105u8,
                            108u8, 42u8,
                        ],
                    )
                }
                pub fn submit_data_unsigned_with_signed_payload(
                    &self,
                    data_payload: runtime_types::pallet_dhatu::DataPayload<
                        runtime_types::sp_runtime::MultiSigner,
                        ::core::primitive::u32,
                    >,
                    signature: runtime_types::sp_runtime::MultiSignature,
                ) -> ::subxt::tx::Payload<types::SubmitDataUnsignedWithSignedPayload>
                {
                    ::subxt::tx::Payload::new_static(
                        "Dhatu",
                        "submit_data_unsigned_with_signed_payload",
                        types::SubmitDataUnsignedWithSignedPayload {
                            data_payload,
                            signature,
                        },
                        [
                            59u8, 154u8, 192u8, 251u8, 202u8, 74u8, 115u8, 98u8, 203u8, 59u8,
                            163u8, 140u8, 62u8, 24u8, 125u8, 52u8, 108u8, 100u8, 87u8, 20u8, 47u8,
                            232u8, 83u8, 245u8, 223u8, 246u8, 202u8, 91u8, 116u8, 80u8, 11u8, 4u8,
                        ],
                    )
                }
            }
        }
        #[doc = "Events for the pallet."]
        pub type Event = runtime_types::pallet_dhatu::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Event generated when new data is accepted to contribute to the average."]
            pub struct NewData {
                pub data: ::core::primitive::u32,
                pub maybe_who: ::core::option::Option<::subxt::utils::AccountId32>,
            }
            impl ::subxt::events::StaticEvent for NewData {
                const PALLET: &'static str = "Dhatu";
                const EVENT: &'static str = "NewData";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct StorageApi;
            impl StorageApi {
                #[doc = " A vector of recently submitted data."]
                #[doc = ""]
                #[doc = " This is used to fetch data."]
                pub fn all_data(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u32,
                    >,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Dhatu",
                        "AllData",
                        vec![],
                        [
                            4u8, 208u8, 162u8, 46u8, 117u8, 49u8, 202u8, 16u8, 176u8, 1u8, 36u8,
                            54u8, 56u8, 145u8, 21u8, 17u8, 128u8, 17u8, 19u8, 185u8, 26u8, 27u8,
                            4u8, 122u8, 137u8, 214u8, 18u8, 140u8, 20u8, 225u8, 52u8, 193u8,
                        ],
                    )
                }
                #[doc = " Defines the block when next unsigned transaction will be accepted."]
                #[doc = ""]
                #[doc = " To prevent spam of unsigned (and unpayed!) transactions on the network,"]
                #[doc = " we only allow one transaction every `T::UnsignedInterval` blocks."]
                #[doc = " This storage entry defines when new transaction is going to be accepted."]
                pub fn next_unsigned_at(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    ::core::primitive::u32,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Dhatu",
                        "NextUnsignedAt",
                        vec![],
                        [
                            128u8, 69u8, 217u8, 175u8, 94u8, 131u8, 228u8, 97u8, 2u8, 91u8, 12u8,
                            95u8, 35u8, 105u8, 136u8, 55u8, 49u8, 3u8, 224u8, 213u8, 96u8, 45u8,
                            19u8, 217u8, 5u8, 6u8, 20u8, 51u8, 219u8, 66u8, 143u8, 50u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                #[doc = " A grace period after we send transaction."]
                #[doc = ""]
                #[doc = " To avoid sending too many transactions, we only attempt to send one"]
                #[doc = " every `GRACE_PERIOD` blocks. We use Local Storage to coordinate"]
                #[doc = " sending between distinct runs of this offchain worker."]
                pub fn grace_period(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Dhatu",
                        "GracePeriod",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " Number of blocks of cooldown after unsigned transaction is included."]
                #[doc = ""]
                #[doc = " This ensures that we only accept unsigned transactions once, every `UnsignedInterval`"]
                #[doc = " blocks."]
                pub fn unsigned_interval(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Dhatu",
                        "UnsignedInterval",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                #[doc = " A configuration for base priority of unsigned transactions."]
                #[doc = ""]
                #[doc = " This is exposed so that it can be tuned for particular runtime, when"]
                #[doc = " multiple pallets send unsigned transactions."]
                pub fn unsigned_priority(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u64> {
                    ::subxt::constants::Address::new_static(
                        "Dhatu",
                        "UnsignedPriority",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                #[doc = " Maximum number of data."]
                pub fn max_data(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Dhatu",
                        "MaxData",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: ext :: codec :: Decode,
                            :: subxt :: ext :: codec :: Encode,
                            :: subxt :: ext :: scale_decode :: DecodeAsType,
                            :: subxt :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt :: ext :: codec)]
                        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)`"]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    #[doc = "Set the new runtime code."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(C)` where `C` length of `code`"]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Error for the System pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Event for the System pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::utils::AccountId32,
                        hash: ::subxt::utils::H256,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod local_runtime {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum HoldReason {
                #[codec(index = 0)]
                Nis,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Runtime;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 6)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 7)]
                TemplateModule(runtime_types::pallet_template::pallet::Call),
                #[codec(index = 8)]
                Contracts(runtime_types::pallet_contracts::pallet::Call),
                #[codec(index = 10)]
                Identity(runtime_types::pallet_identity::pallet::Call),
                #[codec(index = 11)]
                Multisig(runtime_types::pallet_multisig::pallet::Call),
                #[codec(index = 12)]
                Dhatu(runtime_types::pallet_dhatu::pallet::Call),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 5)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
                #[codec(index = 6)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 7)]
                TemplateModule(runtime_types::pallet_template::pallet::Event),
                #[codec(index = 8)]
                Contracts(runtime_types::pallet_contracts::pallet::Event),
                #[codec(index = 10)]
                Identity(runtime_types::pallet_identity::pallet::Event),
                #[codec(index = 11)]
                Multisig(runtime_types::pallet_multisig::pallet::Event),
                #[codec(index = 12)]
                Dhatu(runtime_types::pallet_dhatu::pallet::Event),
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer_allow_death` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    transfer_allow_death {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the regular balance of a given account; it also takes a reserved balance but this"]
                    #[doc = "must be the same as the account's current reserved balance."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    #[doc = ""]
                    #[doc = "WARNING: This call is DEPRECATED! Use `force_set_balance` instead."]
                    set_balance_deprecated {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        old_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer_allow_death`, except the origin must be root and the source account"]
                    #[doc = "may be specified."]
                    force_transfer {
                        source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer_allow_death`] call, but with a check that the transfer will not"]
                    #[doc = "kill the origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer_allow_death`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer_allow_death`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true)."]
                    transfer_all {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Upgrade a specified account."]
                    #[doc = ""]
                    #[doc = "- `origin`: Must be `Signed`."]
                    #[doc = "- `who`: The account to be upgraded."]
                    #[doc = ""]
                    #[doc = "This will waive the transaction fee if at least all but 10% of the accounts needed to"]
                    #[doc = "be upgraded. (We let some not have to be upgraded just in order to allow for the"]
                    #[doc = "possibililty of churn)."]
                    upgrade_accounts {
                        who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Alias for `transfer_allow_death`, provided only for name-wise compatibility."]
                    #[doc = ""]
                    #[doc = "WARNING: DEPRECATED! Will be released in approximately 3 months."]
                    transfer {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Set the regular balance of a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    force_set_balance {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value."]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal."]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value."]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit."]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account."]
                    Expendability,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account."]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist."]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed `MaxReserves`."]
                    TooManyReserves,
                    #[codec(index = 8)]
                    #[doc = "Number of holds exceed `MaxHolds`."]
                    TooManyHolds,
                    #[codec(index = 9)]
                    #[doc = "Number of freezes exceed `MaxFreezes`."]
                    TooManyFreezes,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::utils::AccountId32,
                        free: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    #[doc = "Some amount was minted into an account."]
                    Minted {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    #[doc = "Some amount was burned from an account."]
                    Burned {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    #[doc = "Some amount was suspended from an account (it can be restored later)."]
                    Suspended {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 13)]
                    #[doc = "Some amount was restored into an account."]
                    Restored {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 14)]
                    #[doc = "An account was upgraded."]
                    Upgraded { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 15)]
                    #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
                    Issued { amount: ::core::primitive::u128 },
                    #[codec(index = 16)]
                    #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
                    Rescinded { amount: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    #[doc = "Some balance was locked."]
                    Locked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    #[doc = "Some balance was unlocked."]
                    Unlocked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 19)]
                    #[doc = "Some balance was frozen."]
                    Frozen {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    #[doc = "Some balance was thawed."]
                    Thawed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::pallet_balances::types::Reasons,
                }
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ExtraFlags(pub ::core::primitive::u128);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct IdAmount<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod pallet_contracts {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Deprecated version if [`Self::call`] for use in an in-storage `Call`."]
                    call_old_weight {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Deprecated version if [`Self::instantiate_with_code`] for use in an in-storage `Call`."]
                    instantiate_with_code_old_weight {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Deprecated version if [`Self::instantiate`] for use in an in-storage `Call`."]
                    instantiate_old_weight {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code_hash: ::subxt::utils::H256,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Upload new `code` without instantiating a contract from it."]
                    #[doc = ""]
                    #[doc = "If the code does not already exist a deposit is reserved from the caller"]
                    #[doc = "and unreserved only when [`Self::remove_code`] is called. The size of the reserve"]
                    #[doc = "depends on the instrumented size of the the supplied `code`."]
                    #[doc = ""]
                    #[doc = "If the code already exists in storage it will still return `Ok` and upgrades"]
                    #[doc = "the in storage version to the current"]
                    #[doc = "[`InstructionWeights::version`](InstructionWeights)."]
                    #[doc = ""]
                    #[doc = "- `determinism`: If this is set to any other value but [`Determinism::Enforced`] then"]
                    #[doc = "  the only way to use this code is to delegate call into it from an offchain execution."]
                    #[doc = "  Set to [`Determinism::Enforced`] if in doubt."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "Anyone can instantiate a contract from any uploaded code and thus prevent its removal."]
                    #[doc = "To avoid this situation a constructor could employ access control so that it can"]
                    #[doc = "only be instantiated by permissioned entities. The same is true when uploading"]
                    #[doc = "through [`Self::instantiate_with_code`]."]
                    upload_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        determinism: runtime_types::pallet_contracts::wasm::Determinism,
                    },
                    #[codec(index = 4)]
                    #[doc = "Remove the code stored under `code_hash` and refund the deposit to its owner."]
                    #[doc = ""]
                    #[doc = "A code can only be removed by its original uploader (its owner) and only if it is"]
                    #[doc = "not used by any contract."]
                    remove_code { code_hash: ::subxt::utils::H256 },
                    #[codec(index = 5)]
                    #[doc = "Privileged function that changes the code of an existing contract."]
                    #[doc = ""]
                    #[doc = "This takes care of updating refcounts and all other necessary operations. Returns"]
                    #[doc = "an error if either the `code_hash` or `dest` do not exist."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "This does **not** change the address of the contract in question. This means"]
                    #[doc = "that the contract address is no longer derived from its code hash after calling"]
                    #[doc = "this dispatchable."]
                    set_code {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        code_hash: ::subxt::utils::H256,
                    },
                    #[codec(index = 6)]
                    #[doc = "Makes a call to an account, optionally transferring some balance."]
                    #[doc = ""]
                    #[doc = "# Parameters"]
                    #[doc = ""]
                    #[doc = "* `dest`: Address of the contract to call."]
                    #[doc = "* `value`: The balance to transfer from the `origin` to `dest`."]
                    #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                    #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged from the"]
                    #[doc = "  caller to pay for the storage consumed."]
                    #[doc = "* `data`: The input data to pass to the contract."]
                    #[doc = ""]
                    #[doc = "* If the account is a smart-contract account, the associated code will be"]
                    #[doc = "executed and any value will be transferred."]
                    #[doc = "* If the account is a regular account, any value will be transferred."]
                    #[doc = "* If no account exists and the call value is not less than `existential_deposit`,"]
                    #[doc = "a regular account will be created and any value will be transferred."]
                    call {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Instantiates a new contract from the supplied `code` optionally transferring"]
                    #[doc = "some balance."]
                    #[doc = ""]
                    #[doc = "This dispatchable has the same effect as calling [`Self::upload_code`] +"]
                    #[doc = "[`Self::instantiate`]. Bundling them together provides efficiency gains. Please"]
                    #[doc = "also check the documentation of [`Self::upload_code`]."]
                    #[doc = ""]
                    #[doc = "# Parameters"]
                    #[doc = ""]
                    #[doc = "* `value`: The balance to transfer from the `origin` to the newly created contract."]
                    #[doc = "* `gas_limit`: The gas limit enforced when executing the constructor."]
                    #[doc = "* `storage_deposit_limit`: The maximum amount of balance that can be charged/reserved"]
                    #[doc = "  from the caller to pay for the storage consumed."]
                    #[doc = "* `code`: The contract code to deploy in raw bytes."]
                    #[doc = "* `data`: The input data to pass to the contract constructor."]
                    #[doc = "* `salt`: Used for the address derivation. See [`Pallet::contract_address`]."]
                    #[doc = ""]
                    #[doc = "Instantiation is executed as follows:"]
                    #[doc = ""]
                    #[doc = "- The supplied `code` is instrumented, deployed, and a `code_hash` is created for that"]
                    #[doc = "  code."]
                    #[doc = "- If the `code_hash` already exists on the chain the underlying `code` will be shared."]
                    #[doc = "- The destination address is computed based on the sender, code_hash and the salt."]
                    #[doc = "- The smart-contract account is created at the computed address."]
                    #[doc = "- The `value` is transferred to the new account."]
                    #[doc = "- The `deploy` function is executed in the context of the newly-created account."]
                    instantiate_with_code {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 8)]
                    #[doc = "Instantiates a contract from a previously deployed wasm binary."]
                    #[doc = ""]
                    #[doc = "This function is identical to [`Self::instantiate_with_code`] but without the"]
                    #[doc = "code deployment step. Instead, the `code_hash` of an on-chain deployed wasm binary"]
                    #[doc = "must be supplied."]
                    instantiate {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code_hash: ::subxt::utils::H256,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "A new schedule must have a greater version than the current one."]
                    InvalidScheduleVersion,
                    #[codec(index = 1)]
                    #[doc = "Invalid combination of flags supplied to `seal_call` or `seal_delegate_call`."]
                    InvalidCallFlags,
                    #[codec(index = 2)]
                    #[doc = "The executed contract exhausted its gas limit."]
                    OutOfGas,
                    #[codec(index = 3)]
                    #[doc = "The output buffer supplied to a contract API call was too small."]
                    OutputBufferTooSmall,
                    #[codec(index = 4)]
                    #[doc = "Performing the requested transfer failed. Probably because there isn't enough"]
                    #[doc = "free balance in the sender's account."]
                    TransferFailed,
                    #[codec(index = 5)]
                    #[doc = "Performing a call was denied because the calling depth reached the limit"]
                    #[doc = "of what is specified in the schedule."]
                    MaxCallDepthReached,
                    #[codec(index = 6)]
                    #[doc = "No contract was found at the specified address."]
                    ContractNotFound,
                    #[codec(index = 7)]
                    #[doc = "The code supplied to `instantiate_with_code` exceeds the limit specified in the"]
                    #[doc = "current schedule."]
                    CodeTooLarge,
                    #[codec(index = 8)]
                    #[doc = "No code could be found at the supplied code hash."]
                    CodeNotFound,
                    #[codec(index = 9)]
                    #[doc = "A buffer outside of sandbox memory was passed to a contract API function."]
                    OutOfBounds,
                    #[codec(index = 10)]
                    #[doc = "Input passed to a contract API function failed to decode as expected type."]
                    DecodingFailed,
                    #[codec(index = 11)]
                    #[doc = "Contract trapped during execution."]
                    ContractTrapped,
                    #[codec(index = 12)]
                    #[doc = "The size defined in `T::MaxValueSize` was exceeded."]
                    ValueTooLarge,
                    #[codec(index = 13)]
                    #[doc = "Termination of a contract is not allowed while the contract is already"]
                    #[doc = "on the call stack. Can be triggered by `seal_terminate`."]
                    TerminatedWhileReentrant,
                    #[codec(index = 14)]
                    #[doc = "`seal_call` forwarded this contracts input. It therefore is no longer available."]
                    InputForwarded,
                    #[codec(index = 15)]
                    #[doc = "The subject passed to `seal_random` exceeds the limit."]
                    RandomSubjectTooLong,
                    #[codec(index = 16)]
                    #[doc = "The amount of topics passed to `seal_deposit_events` exceeds the limit."]
                    TooManyTopics,
                    #[codec(index = 17)]
                    #[doc = "The chain does not provide a chain extension. Calling the chain extension results"]
                    #[doc = "in this error. Note that this usually  shouldn't happen as deploying such contracts"]
                    #[doc = "is rejected."]
                    NoChainExtension,
                    #[codec(index = 18)]
                    #[doc = "A contract with the same AccountId already exists."]
                    DuplicateContract,
                    #[codec(index = 19)]
                    #[doc = "A contract self destructed in its constructor."]
                    #[doc = ""]
                    #[doc = "This can be triggered by a call to `seal_terminate`."]
                    TerminatedInConstructor,
                    #[codec(index = 20)]
                    #[doc = "A call tried to invoke a contract that is flagged as non-reentrant."]
                    #[doc = "The only other cause is that a call from a contract into the runtime tried to call back"]
                    #[doc = "into `pallet-contracts`. This would make the whole pallet reentrant with regard to"]
                    #[doc = "contract code execution which is not supported."]
                    ReentranceDenied,
                    #[codec(index = 21)]
                    #[doc = "Origin doesn't have enough balance to pay the required storage deposits."]
                    StorageDepositNotEnoughFunds,
                    #[codec(index = 22)]
                    #[doc = "More storage was created than allowed by the storage deposit limit."]
                    StorageDepositLimitExhausted,
                    #[codec(index = 23)]
                    #[doc = "Code removal was denied because the code is still in use by at least one contract."]
                    CodeInUse,
                    #[codec(index = 24)]
                    #[doc = "The contract ran to completion but decided to revert its storage changes."]
                    #[doc = "Please note that this error is only returned from extrinsics. When called directly"]
                    #[doc = "or via RPC an `Ok` will be returned. In this case the caller needs to inspect the flags"]
                    #[doc = "to determine whether a reversion has taken place."]
                    ContractReverted,
                    #[codec(index = 25)]
                    #[doc = "The contract's code was found to be invalid during validation or instrumentation."]
                    #[doc = ""]
                    #[doc = "The most likely cause of this is that an API was used which is not supported by the"]
                    #[doc = "node. This happens if an older node is used with a new version of ink!. Try updating"]
                    #[doc = "your node to the newest available version."]
                    #[doc = ""]
                    #[doc = "A more detailed error can be found on the node console if debug messages are enabled"]
                    #[doc = "by supplying `-lruntime::contracts=debug`."]
                    CodeRejected,
                    #[codec(index = 26)]
                    #[doc = "An indetermistic code was used in a context where this is not permitted."]
                    Indeterministic,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Contract deployed by address at the specified address."]
                    Instantiated {
                        deployer: ::subxt::utils::AccountId32,
                        contract: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    #[doc = "Contract has been removed."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "The only way for a contract to be removed and emitting this event is by calling"]
                    #[doc = "`seal_terminate`."]
                    Terminated {
                        contract: ::subxt::utils::AccountId32,
                        beneficiary: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Code with the specified hash has been stored."]
                    CodeStored { code_hash: ::subxt::utils::H256 },
                    #[codec(index = 3)]
                    #[doc = "A custom event emitted by the contract."]
                    ContractEmitted {
                        contract: ::subxt::utils::AccountId32,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "A code with the specified hash was removed."]
                    CodeRemoved { code_hash: ::subxt::utils::H256 },
                    #[codec(index = 5)]
                    #[doc = "A contract's code was updated."]
                    ContractCodeUpdated {
                        contract: ::subxt::utils::AccountId32,
                        new_code_hash: ::subxt::utils::H256,
                        old_code_hash: ::subxt::utils::H256,
                    },
                    #[codec(index = 6)]
                    #[doc = "A contract was called either by a plain account or another contract."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "Please keep in mind that like all events this is only emitted for successful"]
                    #[doc = "calls. This is because on failure all storage changes including events are"]
                    #[doc = "rolled back."]
                    Called {
                        caller: ::subxt::utils::AccountId32,
                        contract: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 7)]
                    #[doc = "A contract delegate called a code hash."]
                    #[doc = ""]
                    #[doc = "# Note"]
                    #[doc = ""]
                    #[doc = "Please keep in mind that like all events this is only emitted for successful"]
                    #[doc = "calls. This is because on failure all storage changes including events are"]
                    #[doc = "rolled back."]
                    DelegateCalled {
                        contract: ::subxt::utils::AccountId32,
                        code_hash: ::subxt::utils::H256,
                    },
                }
            }
            pub mod schedule {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct HostFnWeights {
                    pub caller: runtime_types::sp_weights::weight_v2::Weight,
                    pub is_contract: runtime_types::sp_weights::weight_v2::Weight,
                    pub code_hash: runtime_types::sp_weights::weight_v2::Weight,
                    pub own_code_hash: runtime_types::sp_weights::weight_v2::Weight,
                    pub caller_is_origin: runtime_types::sp_weights::weight_v2::Weight,
                    pub address: runtime_types::sp_weights::weight_v2::Weight,
                    pub gas_left: runtime_types::sp_weights::weight_v2::Weight,
                    pub balance: runtime_types::sp_weights::weight_v2::Weight,
                    pub value_transferred: runtime_types::sp_weights::weight_v2::Weight,
                    pub minimum_balance: runtime_types::sp_weights::weight_v2::Weight,
                    pub block_number: runtime_types::sp_weights::weight_v2::Weight,
                    pub now: runtime_types::sp_weights::weight_v2::Weight,
                    pub weight_to_fee: runtime_types::sp_weights::weight_v2::Weight,
                    pub gas: runtime_types::sp_weights::weight_v2::Weight,
                    pub input: runtime_types::sp_weights::weight_v2::Weight,
                    pub input_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub r#return: runtime_types::sp_weights::weight_v2::Weight,
                    pub return_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub terminate: runtime_types::sp_weights::weight_v2::Weight,
                    pub random: runtime_types::sp_weights::weight_v2::Weight,
                    pub deposit_event: runtime_types::sp_weights::weight_v2::Weight,
                    pub deposit_event_per_topic: runtime_types::sp_weights::weight_v2::Weight,
                    pub deposit_event_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub debug_message: runtime_types::sp_weights::weight_v2::Weight,
                    pub debug_message_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub set_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub set_storage_per_new_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub set_storage_per_old_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub set_code_hash: runtime_types::sp_weights::weight_v2::Weight,
                    pub clear_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub clear_storage_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub contains_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub contains_storage_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub get_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub get_storage_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub take_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub take_storage_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub transfer: runtime_types::sp_weights::weight_v2::Weight,
                    pub call: runtime_types::sp_weights::weight_v2::Weight,
                    pub delegate_call: runtime_types::sp_weights::weight_v2::Weight,
                    pub call_transfer_surcharge: runtime_types::sp_weights::weight_v2::Weight,
                    pub call_per_cloned_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiate: runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiate_transfer_surcharge:
                        runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiate_per_input_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiate_per_salt_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_sha2_256: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_sha2_256_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_keccak_256: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_keccak_256_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_blake2_256: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_blake2_256_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_blake2_128: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_blake2_128_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub ecdsa_recover: runtime_types::sp_weights::weight_v2::Weight,
                    pub ecdsa_to_eth_address: runtime_types::sp_weights::weight_v2::Weight,
                    pub sr25519_verify: runtime_types::sp_weights::weight_v2::Weight,
                    pub sr25519_verify_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub reentrance_count: runtime_types::sp_weights::weight_v2::Weight,
                    pub account_reentrance_count: runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiation_nonce: runtime_types::sp_weights::weight_v2::Weight,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InstructionWeights {
                    pub version: ::core::primitive::u32,
                    pub fallback: ::core::primitive::u32,
                    pub i64const: ::core::primitive::u32,
                    pub i64load: ::core::primitive::u32,
                    pub i64store: ::core::primitive::u32,
                    pub select: ::core::primitive::u32,
                    pub r#if: ::core::primitive::u32,
                    pub br: ::core::primitive::u32,
                    pub br_if: ::core::primitive::u32,
                    pub br_table: ::core::primitive::u32,
                    pub br_table_per_entry: ::core::primitive::u32,
                    pub call: ::core::primitive::u32,
                    pub call_indirect: ::core::primitive::u32,
                    pub call_per_local: ::core::primitive::u32,
                    pub local_get: ::core::primitive::u32,
                    pub local_set: ::core::primitive::u32,
                    pub local_tee: ::core::primitive::u32,
                    pub global_get: ::core::primitive::u32,
                    pub global_set: ::core::primitive::u32,
                    pub memory_current: ::core::primitive::u32,
                    pub memory_grow: ::core::primitive::u32,
                    pub i64clz: ::core::primitive::u32,
                    pub i64ctz: ::core::primitive::u32,
                    pub i64popcnt: ::core::primitive::u32,
                    pub i64eqz: ::core::primitive::u32,
                    pub i64extendsi32: ::core::primitive::u32,
                    pub i64extendui32: ::core::primitive::u32,
                    pub i32wrapi64: ::core::primitive::u32,
                    pub i64eq: ::core::primitive::u32,
                    pub i64ne: ::core::primitive::u32,
                    pub i64lts: ::core::primitive::u32,
                    pub i64ltu: ::core::primitive::u32,
                    pub i64gts: ::core::primitive::u32,
                    pub i64gtu: ::core::primitive::u32,
                    pub i64les: ::core::primitive::u32,
                    pub i64leu: ::core::primitive::u32,
                    pub i64ges: ::core::primitive::u32,
                    pub i64geu: ::core::primitive::u32,
                    pub i64add: ::core::primitive::u32,
                    pub i64sub: ::core::primitive::u32,
                    pub i64mul: ::core::primitive::u32,
                    pub i64divs: ::core::primitive::u32,
                    pub i64divu: ::core::primitive::u32,
                    pub i64rems: ::core::primitive::u32,
                    pub i64remu: ::core::primitive::u32,
                    pub i64and: ::core::primitive::u32,
                    pub i64or: ::core::primitive::u32,
                    pub i64xor: ::core::primitive::u32,
                    pub i64shl: ::core::primitive::u32,
                    pub i64shrs: ::core::primitive::u32,
                    pub i64shru: ::core::primitive::u32,
                    pub i64rotl: ::core::primitive::u32,
                    pub i64rotr: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Limits {
                    pub event_topics: ::core::primitive::u32,
                    pub globals: ::core::primitive::u32,
                    pub locals: ::core::primitive::u32,
                    pub parameters: ::core::primitive::u32,
                    pub memory_pages: ::core::primitive::u32,
                    pub table_size: ::core::primitive::u32,
                    pub br_table_size: ::core::primitive::u32,
                    pub subject_len: ::core::primitive::u32,
                    pub payload_len: ::core::primitive::u32,
                    pub runtime_memory: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Schedule {
                    pub limits: runtime_types::pallet_contracts::schedule::Limits,
                    pub instruction_weights:
                        runtime_types::pallet_contracts::schedule::InstructionWeights,
                    pub host_fn_weights: runtime_types::pallet_contracts::schedule::HostFnWeights,
                }
            }
            pub mod storage {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ContractInfo {
                    pub trie_id: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub deposit_account: runtime_types::pallet_contracts::storage::DepositAccount,
                    pub code_hash: ::subxt::utils::H256,
                    pub storage_bytes: ::core::primitive::u32,
                    pub storage_items: ::core::primitive::u32,
                    pub storage_byte_deposit: ::core::primitive::u128,
                    pub storage_item_deposit: ::core::primitive::u128,
                    pub storage_base_deposit: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DeletionQueueManager {
                    pub insert_counter: ::core::primitive::u32,
                    pub delete_counter: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DepositAccount(pub ::subxt::utils::AccountId32);
            }
            pub mod wasm {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Determinism {
                    #[codec(index = 0)]
                    Enforced,
                    #[codec(index = 1)]
                    Relaxed,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct OwnerInfo {
                    pub owner: ::subxt::utils::AccountId32,
                    #[codec(compact)]
                    pub deposit: ::core::primitive::u128,
                    #[codec(compact)]
                    pub refcount: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PrefabWasmModule {
                    #[codec(compact)]
                    pub instruction_weights_version: ::core::primitive::u32,
                    #[codec(compact)]
                    pub initial: ::core::primitive::u32,
                    #[codec(compact)]
                    pub maximum: ::core::primitive::u32,
                    pub code: runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub determinism: runtime_types::pallet_contracts::wasm::Determinism,
                }
            }
        }
        pub mod pallet_dhatu {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "A public part of the pallet."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Submit new data to the list."]
                    #[doc = ""]
                    #[doc = "This method is a public function of the module and can be called from within"]
                    #[doc = "a transaction. It appends given `data` to current list of data."]
                    #[doc = "In our example the `offchain worker` will create, sign & submit a transaction that"]
                    #[doc = "calls this function passing the data."]
                    #[doc = ""]
                    #[doc = "The transaction needs to be signed (see `ensure_signed`) check, so that the caller"]
                    #[doc = "pays a fee to execute it."]
                    #[doc = "This makes sure that it's not easy (or rather cheap) to attack the chain by submitting"]
                    #[doc = "excesive transactions, but note that it doesn't ensure the data oracle is actually"]
                    #[doc = "working and receives (and provides) meaningful data."]
                    #[doc = "This example is not focused on correctness of the oracle itself, but rather its"]
                    #[doc = "purpose is to showcase offchain worker capabilities."]
                    submit_data { data: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "Submit new data to the list via unsigned transaction."]
                    #[doc = ""]
                    #[doc = "Works exactly like the `submit_data` function, but since we allow sending the"]
                    #[doc = "transaction without a signature, and hence without paying any fees,"]
                    #[doc = "we need a way to make sure that only some transactions are accepted."]
                    #[doc = "This function can be called only once every `T::UnsignedInterval` blocks."]
                    #[doc = "Transactions that call that function are de-duplicated on the pool level"]
                    #[doc = "via `validate_unsigned` implementation and also are rendered invalid if"]
                    #[doc = "the function has already been called in current \"session\"."]
                    #[doc = ""]
                    #[doc = "It's important to specify `weight` for unsigned calls as well, because even though"]
                    #[doc = "they don't charge fees, we still don't want a single block to contain unlimited"]
                    #[doc = "number of such transactions."]
                    #[doc = ""]
                    #[doc = "This example is not focused on correctness of the oracle itself, but rather its"]
                    #[doc = "purpose is to showcase offchain worker capabilities."]
                    submit_data_unsigned {
                        block_number: ::core::primitive::u32,
                        data: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    submit_data_unsigned_with_signed_payload {
                        data_payload: runtime_types::pallet_dhatu::DataPayload<
                            runtime_types::sp_runtime::MultiSigner,
                            ::core::primitive::u32,
                        >,
                        signature: runtime_types::sp_runtime::MultiSignature,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Events for the pallet."]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Event generated when new data is accepted to contribute to the average."]
                    NewData {
                        data: ::core::primitive::u32,
                        maybe_who: ::core::option::Option<::subxt::utils::AccountId32>,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct DataPayload<_0, _1> {
                pub block_number: _1,
                pub data: _1,
                pub public: _0,
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 1)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                    #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                    #[doc = "if the block author is defined it will be defined as the equivocation"]
                    #[doc = "reporter."]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 2)]
                    #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                    #[doc = ""]
                    #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                    #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                    #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                    #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                    #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                    #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                    #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                    #[doc = "block of all validators of the new authority set."]
                    #[doc = ""]
                    #[doc = "Only callable by root."]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
                    #[doc = "(either paused or already pending pause)."]
                    PauseFailed,
                    #[codec(index = 1)]
                    #[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
                    #[doc = "(either live or already pending resume)."]
                    ResumeFailed,
                    #[codec(index = 2)]
                    #[doc = "Attempt to signal GRANDPA change with one already pending."]
                    ChangePending,
                    #[codec(index = 3)]
                    #[doc = "Cannot signal forced change so soon after last."]
                    TooSoon,
                    #[codec(index = 4)]
                    #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    #[doc = "A given equivocation report is valid but already previously reported."]
                    DuplicateOffenceReport,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New authority set has been applied."]
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Current authority set has been paused."]
                    Paused,
                    #[codec(index = 2)]
                    #[doc = "Current authority set has been resumed."]
                    Resumed,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_identity {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Identity pallet declaration."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Add a registrar to the system."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `T::RegistrarOrigin`."]
                    #[doc = ""]
                    #[doc = "- `account`: the account of the registrar."]
                    #[doc = ""]
                    #[doc = "Emits `RegistrarAdded` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R)` where `R` registrar-count (governance-bounded and code-bounded)."]
                    add_registrar {
                        account: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set an account's identity information and reserve the appropriate deposit."]
                    #[doc = ""]
                    #[doc = "If the account already has identity information, the deposit is taken as part payment"]
                    #[doc = "for the new deposit."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `info`: The identity information."]
                    #[doc = ""]
                    #[doc = "Emits `IdentitySet` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(X + X' + R)`"]
                    #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)"]
                    #[doc = "  - where `R` judgements-count (registrar-count-bounded)"]
                    set_identity {
                        info:
                            ::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Set the sub-accounts of the sender."]
                    #[doc = ""]
                    #[doc = "Payment: Any aggregate balance reserved by previous `set_subs` calls will be returned"]
                    #[doc = "and an amount `SubAccountDeposit` will be reserved for each item in `subs`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "identity."]
                    #[doc = ""]
                    #[doc = "- `subs`: The identity's (new) sub-accounts."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(P + S)`"]
                    #[doc = "  - where `P` old-subs-count (hard- and deposit-bounded)."]
                    #[doc = "  - where `S` subs-count (hard- and deposit-bounded)."]
                    set_subs {
                        subs: ::std::vec::Vec<(
                            ::subxt::utils::AccountId32,
                            runtime_types::pallet_identity::types::Data,
                        )>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Clear an account's identity info and all sub-accounts and return all deposits."]
                    #[doc = ""]
                    #[doc = "Payment: All reserved balances on the account are returned."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "identity."]
                    #[doc = ""]
                    #[doc = "Emits `IdentityCleared` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R + S + X)`"]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    #[doc = "  - where `S` subs-count (hard- and deposit-bounded)."]
                    #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                    clear_identity,
                    #[codec(index = 4)]
                    #[doc = "Request a judgement from a registrar."]
                    #[doc = ""]
                    #[doc = "Payment: At most `max_fee` will be reserved for payment to the registrar if judgement"]
                    #[doc = "given."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a"]
                    #[doc = "registered identity."]
                    #[doc = ""]
                    #[doc = "- `reg_index`: The index of the registrar whose judgement is requested."]
                    #[doc = "- `max_fee`: The maximum fee that may be paid. This should just be auto-populated as:"]
                    #[doc = ""]
                    #[doc = "```nocompile"]
                    #[doc = "Self::registrars().get(reg_index).unwrap().fee"]
                    #[doc = "```"]
                    #[doc = ""]
                    #[doc = "Emits `JudgementRequested` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R + X)`."]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                    request_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        #[codec(compact)]
                        max_fee: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Cancel a previous request."]
                    #[doc = ""]
                    #[doc = "Payment: A previously reserved deposit is returned on success."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a"]
                    #[doc = "registered identity."]
                    #[doc = ""]
                    #[doc = "- `reg_index`: The index of the registrar whose judgement is no longer requested."]
                    #[doc = ""]
                    #[doc = "Emits `JudgementUnrequested` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R + X)`."]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                    cancel_request { reg_index: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    #[doc = "Set the fee required for a judgement to be requested from a registrar."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                    #[doc = "of the registrar whose index is `index`."]
                    #[doc = ""]
                    #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                    #[doc = "- `fee`: the new fee."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R)`."]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    set_fee {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        #[codec(compact)]
                        fee: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    #[doc = "Change the account associated with a registrar."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                    #[doc = "of the registrar whose index is `index`."]
                    #[doc = ""]
                    #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                    #[doc = "- `new`: the new account ID."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R)`."]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    set_account_id {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 8)]
                    #[doc = "Set the field information for a registrar."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                    #[doc = "of the registrar whose index is `index`."]
                    #[doc = ""]
                    #[doc = "- `index`: the index of the registrar whose fee is to be set."]
                    #[doc = "- `fields`: the fields that the registrar concerns themselves with."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R)`."]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    set_fields {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        fields: runtime_types::pallet_identity::types::BitFlags<
                            runtime_types::pallet_identity::types::IdentityField,
                        >,
                    },
                    #[codec(index = 9)]
                    #[doc = "Provide a judgement for an account's identity."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must be the account"]
                    #[doc = "of the registrar whose index is `reg_index`."]
                    #[doc = ""]
                    #[doc = "- `reg_index`: the index of the registrar whose judgement is being made."]
                    #[doc = "- `target`: the account whose identity the judgement is upon. This must be an account"]
                    #[doc = "  with a registered identity."]
                    #[doc = "- `judgement`: the judgement of the registrar of index `reg_index` about `target`."]
                    #[doc = "- `identity`: The hash of the [`IdentityInfo`] for that the judgement is provided."]
                    #[doc = ""]
                    #[doc = "Emits `JudgementGiven` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R + X)`."]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                    provide_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        judgement: runtime_types::pallet_identity::types::Judgement<
                            ::core::primitive::u128,
                        >,
                        identity: ::subxt::utils::H256,
                    },
                    #[codec(index = 10)]
                    #[doc = "Remove an account's identity and sub-account information and slash the deposits."]
                    #[doc = ""]
                    #[doc = "Payment: Reserved balances from `set_subs` and `set_identity` are slashed and handled by"]
                    #[doc = "`Slash`. Verification request deposits are not returned; they should be cancelled"]
                    #[doc = "manually using `cancel_request`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must match `T::ForceOrigin`."]
                    #[doc = ""]
                    #[doc = "- `target`: the account whose identity the judgement is upon. This must be an account"]
                    #[doc = "  with a registered identity."]
                    #[doc = ""]
                    #[doc = "Emits `IdentityKilled` if successful."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(R + S + X)`"]
                    #[doc = "  - where `R` registrar-count (governance-bounded)."]
                    #[doc = "  - where `S` subs-count (hard- and deposit-bounded)."]
                    #[doc = "  - where `X` additional-field-count (deposit-bounded and code-bounded)."]
                    kill_identity {
                        target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 11)]
                    #[doc = "Add the given account to the sender's subs."]
                    #[doc = ""]
                    #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                    #[doc = "to the sender."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "sub identity of `sub`."]
                    add_sub {
                        sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    #[codec(index = 12)]
                    #[doc = "Alter the associated name of the given sub-account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "sub identity of `sub`."]
                    rename_sub {
                        sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    #[codec(index = 13)]
                    #[doc = "Remove the given account from the sender's subs."]
                    #[doc = ""]
                    #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                    #[doc = "to the sender."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "sub identity of `sub`."]
                    remove_sub {
                        sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 14)]
                    #[doc = "Remove the sender as a sub-account."]
                    #[doc = ""]
                    #[doc = "Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated"]
                    #[doc = "to the sender (*not* the original depositor)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_ and the sender must have a registered"]
                    #[doc = "super-identity."]
                    #[doc = ""]
                    #[doc = "NOTE: This should not normally be used, but is provided in the case that the non-"]
                    #[doc = "controller of an account is maliciously registered as a sub-account."]
                    quit_sub,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Too many subs-accounts."]
                    TooManySubAccounts,
                    #[codec(index = 1)]
                    #[doc = "Account isn't found."]
                    NotFound,
                    #[codec(index = 2)]
                    #[doc = "Account isn't named."]
                    NotNamed,
                    #[codec(index = 3)]
                    #[doc = "Empty index."]
                    EmptyIndex,
                    #[codec(index = 4)]
                    #[doc = "Fee is changed."]
                    FeeChanged,
                    #[codec(index = 5)]
                    #[doc = "No identity found."]
                    NoIdentity,
                    #[codec(index = 6)]
                    #[doc = "Sticky judgement."]
                    StickyJudgement,
                    #[codec(index = 7)]
                    #[doc = "Judgement given."]
                    JudgementGiven,
                    #[codec(index = 8)]
                    #[doc = "Invalid judgement."]
                    InvalidJudgement,
                    #[codec(index = 9)]
                    #[doc = "The index is invalid."]
                    InvalidIndex,
                    #[codec(index = 10)]
                    #[doc = "The target is invalid."]
                    InvalidTarget,
                    #[codec(index = 11)]
                    #[doc = "Too many additional fields."]
                    TooManyFields,
                    #[codec(index = 12)]
                    #[doc = "Maximum amount of registrars reached. Cannot add any more."]
                    TooManyRegistrars,
                    #[codec(index = 13)]
                    #[doc = "Account ID is already named."]
                    AlreadyClaimed,
                    #[codec(index = 14)]
                    #[doc = "Sender is not a sub-account."]
                    NotSub,
                    #[codec(index = 15)]
                    #[doc = "Sub-account isn't owned by sender."]
                    NotOwned,
                    #[codec(index = 16)]
                    #[doc = "The provided judgement was for a different identity."]
                    JudgementForDifferentIdentity,
                    #[codec(index = 17)]
                    #[doc = "Error that occurs when there is an issue paying for judgement."]
                    JudgementPaymentFailed,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A name was set or reset (which will remove all judgements)."]
                    IdentitySet { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 1)]
                    #[doc = "A name was cleared, and the given balance returned."]
                    IdentityCleared {
                        who: ::subxt::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "A name was removed and the given balance slashed."]
                    IdentityKilled {
                        who: ::subxt::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A judgement was asked from a registrar."]
                    JudgementRequested {
                        who: ::subxt::utils::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "A judgement request was retracted."]
                    JudgementUnrequested {
                        who: ::subxt::utils::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "A judgement was given by a registrar."]
                    JudgementGiven {
                        target: ::subxt::utils::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "A registrar was added."]
                    RegistrarAdded {
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "A sub-identity was added to an identity and the deposit paid."]
                    SubIdentityAdded {
                        sub: ::subxt::utils::AccountId32,
                        main: ::subxt::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "A sub-identity was removed from an identity and the deposit freed."]
                    SubIdentityRemoved {
                        sub: ::subxt::utils::AccountId32,
                        main: ::subxt::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "A sub-identity was cleared, and the given deposit repatriated from the"]
                    #[doc = "main identity account to the sub-identity account."]
                    SubIdentityRevoked {
                        sub: ::subxt::utils::AccountId32,
                        main: ::subxt::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BitFlags<_0>(
                    pub ::core::primitive::u64,
                    #[codec(skip)] pub ::core::marker::PhantomData<_0>,
                );
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Data {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    Raw0([::core::primitive::u8; 0usize]),
                    #[codec(index = 2)]
                    Raw1([::core::primitive::u8; 1usize]),
                    #[codec(index = 3)]
                    Raw2([::core::primitive::u8; 2usize]),
                    #[codec(index = 4)]
                    Raw3([::core::primitive::u8; 3usize]),
                    #[codec(index = 5)]
                    Raw4([::core::primitive::u8; 4usize]),
                    #[codec(index = 6)]
                    Raw5([::core::primitive::u8; 5usize]),
                    #[codec(index = 7)]
                    Raw6([::core::primitive::u8; 6usize]),
                    #[codec(index = 8)]
                    Raw7([::core::primitive::u8; 7usize]),
                    #[codec(index = 9)]
                    Raw8([::core::primitive::u8; 8usize]),
                    #[codec(index = 10)]
                    Raw9([::core::primitive::u8; 9usize]),
                    #[codec(index = 11)]
                    Raw10([::core::primitive::u8; 10usize]),
                    #[codec(index = 12)]
                    Raw11([::core::primitive::u8; 11usize]),
                    #[codec(index = 13)]
                    Raw12([::core::primitive::u8; 12usize]),
                    #[codec(index = 14)]
                    Raw13([::core::primitive::u8; 13usize]),
                    #[codec(index = 15)]
                    Raw14([::core::primitive::u8; 14usize]),
                    #[codec(index = 16)]
                    Raw15([::core::primitive::u8; 15usize]),
                    #[codec(index = 17)]
                    Raw16([::core::primitive::u8; 16usize]),
                    #[codec(index = 18)]
                    Raw17([::core::primitive::u8; 17usize]),
                    #[codec(index = 19)]
                    Raw18([::core::primitive::u8; 18usize]),
                    #[codec(index = 20)]
                    Raw19([::core::primitive::u8; 19usize]),
                    #[codec(index = 21)]
                    Raw20([::core::primitive::u8; 20usize]),
                    #[codec(index = 22)]
                    Raw21([::core::primitive::u8; 21usize]),
                    #[codec(index = 23)]
                    Raw22([::core::primitive::u8; 22usize]),
                    #[codec(index = 24)]
                    Raw23([::core::primitive::u8; 23usize]),
                    #[codec(index = 25)]
                    Raw24([::core::primitive::u8; 24usize]),
                    #[codec(index = 26)]
                    Raw25([::core::primitive::u8; 25usize]),
                    #[codec(index = 27)]
                    Raw26([::core::primitive::u8; 26usize]),
                    #[codec(index = 28)]
                    Raw27([::core::primitive::u8; 27usize]),
                    #[codec(index = 29)]
                    Raw28([::core::primitive::u8; 28usize]),
                    #[codec(index = 30)]
                    Raw29([::core::primitive::u8; 29usize]),
                    #[codec(index = 31)]
                    Raw30([::core::primitive::u8; 30usize]),
                    #[codec(index = 32)]
                    Raw31([::core::primitive::u8; 31usize]),
                    #[codec(index = 33)]
                    Raw32([::core::primitive::u8; 32usize]),
                    #[codec(index = 34)]
                    BlakeTwo256([::core::primitive::u8; 32usize]),
                    #[codec(index = 35)]
                    Sha256([::core::primitive::u8; 32usize]),
                    #[codec(index = 36)]
                    Keccak256([::core::primitive::u8; 32usize]),
                    #[codec(index = 37)]
                    ShaThree256([::core::primitive::u8; 32usize]),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum IdentityField {
                    #[codec(index = 1)]
                    Display,
                    #[codec(index = 2)]
                    Legal,
                    #[codec(index = 4)]
                    Web,
                    #[codec(index = 8)]
                    Riot,
                    #[codec(index = 16)]
                    Email,
                    #[codec(index = 32)]
                    PgpFingerprint,
                    #[codec(index = 64)]
                    Image,
                    #[codec(index = 128)]
                    Twitter,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct IdentityInfo {
                    pub additional: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                        runtime_types::pallet_identity::types::Data,
                        runtime_types::pallet_identity::types::Data,
                    )>,
                    pub display: runtime_types::pallet_identity::types::Data,
                    pub legal: runtime_types::pallet_identity::types::Data,
                    pub web: runtime_types::pallet_identity::types::Data,
                    pub riot: runtime_types::pallet_identity::types::Data,
                    pub email: runtime_types::pallet_identity::types::Data,
                    pub pgp_fingerprint: ::core::option::Option<[::core::primitive::u8; 20usize]>,
                    pub image: runtime_types::pallet_identity::types::Data,
                    pub twitter: runtime_types::pallet_identity::types::Data,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Judgement<_0> {
                    #[codec(index = 0)]
                    Unknown,
                    #[codec(index = 1)]
                    FeePaid(_0),
                    #[codec(index = 2)]
                    Reasonable,
                    #[codec(index = 3)]
                    KnownGood,
                    #[codec(index = 4)]
                    OutOfDate,
                    #[codec(index = 5)]
                    LowQuality,
                    #[codec(index = 6)]
                    Erroneous,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RegistrarInfo<_0, _1> {
                    pub account: _1,
                    pub fee: _0,
                    pub fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Registration<_0> {
                    pub judgements: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                        ::core::primitive::u32,
                        runtime_types::pallet_identity::types::Judgement<_0>,
                    )>,
                    pub deposit: _0,
                    pub info: runtime_types::pallet_identity::types::IdentityInfo,
                }
            }
        }
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Immediately dispatch a multi-signature call using a single approval from the caller."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who are part of the"]
                    #[doc = "multi-signature, but do not participate in the approval process."]
                    #[doc = "- `call`: The call to be executed."]
                    #[doc = ""]
                    #[doc = "Result is equivalent to the dispatched result."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "O(Z + C) where Z is the length of the call and C its execution weight."]
                    as_multi_threshold_1 {
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        call: ::std::boxed::Box<runtime_types::local_runtime::RuntimeCall>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                    #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                    #[doc = ""]
                    #[doc = "If there are enough, then dispatch the call."]
                    #[doc = ""]
                    #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                    #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                    #[doc = "is cancelled."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                    #[doc = "dispatch. May not be empty."]
                    #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                    #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                    #[doc = "transaction index) of the first approval transaction."]
                    #[doc = "- `call`: The call to be executed."]
                    #[doc = ""]
                    #[doc = "NOTE: Unless this is the final approval, you will generally want to use"]
                    #[doc = "`approve_as_multi` instead, since it only requires a hash of the call."]
                    #[doc = ""]
                    #[doc = "Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise"]
                    #[doc = "on success, result is `Ok` and the result from the interior call, if it was executed,"]
                    #[doc = "may be found in the deposited `MultisigExecuted` event."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(S + Z + Call)`."]
                    #[doc = "- Up to one balance-reserve or unreserve operation."]
                    #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                    #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                    #[doc = "- One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len."]
                    #[doc = "- One encode & hash, both of complexity `O(S)`."]
                    #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                    #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                    #[doc = "- One event."]
                    #[doc = "- The weight of the `call`."]
                    #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                    #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call: ::std::boxed::Box<runtime_types::local_runtime::RuntimeCall>,
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
                    #[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
                    #[doc = ""]
                    #[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
                    #[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
                    #[doc = "is cancelled."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                    #[doc = "dispatch. May not be empty."]
                    #[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
                    #[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
                    #[doc = "transaction index) of the first approval transaction."]
                    #[doc = "- `call_hash`: The hash of the call to be executed."]
                    #[doc = ""]
                    #[doc = "NOTE: If this is the final approval, you will want to use `as_multi` instead."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(S)`."]
                    #[doc = "- Up to one balance-reserve or unreserve operation."]
                    #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                    #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                    #[doc = "- One encode & hash, both of complexity `O(S)`."]
                    #[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
                    #[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
                    #[doc = "- One event."]
                    #[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
                    #[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 3)]
                    #[doc = "Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously"]
                    #[doc = "for this operation will be unreserved on success."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
                    #[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
                    #[doc = "dispatch. May not be empty."]
                    #[doc = "- `timepoint`: The timepoint (block number and transaction index) of the first approval"]
                    #[doc = "transaction for this dispatch."]
                    #[doc = "- `call_hash`: The hash of the call to be executed."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(S)`."]
                    #[doc = "- Up to one balance-reserve or unreserve operation."]
                    #[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
                    #[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
                    #[doc = "- One encode & hash, both of complexity `O(S)`."]
                    #[doc = "- One event."]
                    #[doc = "- I/O: 1 read `O(S)`, one remove."]
                    #[doc = "- Storage: removes one item."]
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Threshold must be 2 or greater."]
                    MinimumThreshold,
                    #[codec(index = 1)]
                    #[doc = "Call is already approved by this signatory."]
                    AlreadyApproved,
                    #[codec(index = 2)]
                    #[doc = "Call doesn't need any (more) approvals."]
                    NoApprovalsNeeded,
                    #[codec(index = 3)]
                    #[doc = "There are too few signatories in the list."]
                    TooFewSignatories,
                    #[codec(index = 4)]
                    #[doc = "There are too many signatories in the list."]
                    TooManySignatories,
                    #[codec(index = 5)]
                    #[doc = "The signatories were provided out of order; they should be ordered."]
                    SignatoriesOutOfOrder,
                    #[codec(index = 6)]
                    #[doc = "The sender was contained in the other signatories; it shouldn't be."]
                    SenderInSignatories,
                    #[codec(index = 7)]
                    #[doc = "Multisig operation not found when attempting to cancel."]
                    NotFound,
                    #[codec(index = 8)]
                    #[doc = "Only the account that originally created the multisig is able to cancel it."]
                    NotOwner,
                    #[codec(index = 9)]
                    #[doc = "No timepoint was given, yet the multisig operation is already underway."]
                    NoTimepoint,
                    #[codec(index = 10)]
                    #[doc = "A different timepoint was given to the multisig operation that is underway."]
                    WrongTimepoint,
                    #[codec(index = 11)]
                    #[doc = "A timepoint was given, yet no multisig operation is underway."]
                    UnexpectedTimepoint,
                    #[codec(index = 12)]
                    #[doc = "The maximum weight information provided was too low."]
                    MaxWeightTooLow,
                    #[codec(index = 13)]
                    #[doc = "The data to be stored is already stored."]
                    AlreadyStored,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A new multisig operation has begun."]
                    NewMultisig {
                        approving: ::subxt::utils::AccountId32,
                        multisig: ::subxt::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    #[doc = "A multisig operation has been approved by someone."]
                    MultisigApproval {
                        approving: ::subxt::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    #[doc = "A multisig operation has been executed."]
                    MultisigExecuted {
                        approving: ::subxt::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 3)]
                    #[doc = "A multisig operation has been cancelled."]
                    MultisigCancelled {
                        cancelling: ::subxt::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: runtime_types::bounded_collections::bounded_vec::BoundedVec<_2>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::local_runtime::RuntimeCall>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::local_runtime::RuntimeCall>,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    set_key {
                        new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- O(1)."]
                    sudo_as {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        call: ::std::boxed::Box<runtime_types::local_runtime::RuntimeCall>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Error for the Sudo pallet"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account"]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
                    KeyChanged {
                        old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 2)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_template {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "An example dispatchable that takes a singles value as a parameter, writes the value to"]
                    #[doc = "storage and emits an event. This function must be dispatched by a signed extrinsic."]
                    do_something { something: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "An example dispatchable that may throw a custom error."]
                    cause_error,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t"]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Error names should be descriptive."]
                    NoneValue,
                    #[codec(index = 1)]
                    #[doc = "Errors should have helpful documentation associated with them."]
                    StorageOverflow,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Event documentation should end with an array that provides descriptive names for event"]
                    #[doc = "parameters. [something, who]"]
                    SomethingStored {
                        something: ::core::primitive::u32,
                        who: ::subxt::utils::AccountId32,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "`MinimumPeriod`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Inherent`."]
                    #[doc = ""]
                    #[doc = "## Complexity"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                    #[doc = "has been paid by `who`."]
                    TransactionFeePaid {
                        who: ::subxt::utils::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: CompactAs,
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
            }
        }
        pub mod sp_consensus_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Void {}
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1, _2, _3)>,
                    );
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum MultiSigner {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Public),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Public),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Public),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
    }
}
