use cumulus_primitives_core::ParaId;
use kpron_runtime::{AccountId, AuraId, Signature};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup, Properties};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_core::crypto::Ss58Codec;

/// Specialized `ChainSpec` for the normal Kpron runtime.
pub type ChainSpec = sc_service::GenericChainSpec<kpron_runtime::GenesisConfig, Extensions>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_dev_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}
/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(seed, None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_dev_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_dev_seed::<TPublic>(seed)).into_account()
}
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn get_account_id_from_str<TPublic: Public>(s: &str) -> AccountId
	where AccountId: From<TPublic>
{
	let public = TPublic::from_string(s).unwrap();
	AccountId::from(public)
}

pub fn kpron_config(id: ParaId, relay_chain: String) -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Kpron",
		// ID
		"kpron",
		ChainType::Live,
		move || {
			generate_genesis(
				get_account_id_from_str::<sr25519::Public>("5DoJDZNU84uLQz19kj4KhpFDxdnaQv9mNw8QTDSGDaPWdxfE"),
				vec![
					AuraId::from_string("5Dh7s4b8rs2emsq7hvvpTJTtJwNXoGxHbbrxwBGeNd9VuXo1").unwrap(),
					AuraId::from_string("5GYqdDCfzTExVaUbZ3neycG6mR8iFrYFK6HPJhJictBuksRj").unwrap(),
					AuraId::from_string("5FLQNyYeMz2ucJcMuGsQBAjsdb5XhkFYBPWrv91CUiiKqTEr").unwrap(),
				],
				vec![
					(get_account_id_from_str::<sr25519::Public>("5DoJDZNU84uLQz19kj4KhpFDxdnaQv9mNw8QTDSGDaPWdxfE"), 1_000_000_000_000_000_000_000),
				],
				id,
			)
		},
		Vec::new(),
		None,
		Some("kpron"),
		chain_properties(),
		Extensions {
			relay_chain, // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}
pub fn development_config(id: ParaId, relay_chain: String) -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Kpron Development",
		// ID
		"kpron_dev",
		ChainType::Development,
		move || {
			generate_genesis(
				get_account_id_from_dev_seed::<sr25519::Public>("Alice"),
				vec![
					get_from_dev_seed::<AuraId>("Alice"),
					get_from_dev_seed::<AuraId>("Bob"),
				],
				vec![
					(get_account_id_from_dev_seed::<sr25519::Public>("Alice"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Bob"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Alice//stash"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Bob//stash"), 1 << 60),
				],
				id,
			)
		},
		Vec::new(),
		None,
		Some("kpron"),
		chain_properties(),
		Extensions {
			relay_chain, // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

pub fn local_testnet_config(id: ParaId, relay_chain: String) -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Kpron Local",
		// ID
		"kpron_local",
		ChainType::Local,
		move || {
			generate_genesis(
				get_account_id_from_dev_seed::<sr25519::Public>("Alice"),
				vec![
					get_from_dev_seed::<AuraId>("Alice"),
					get_from_dev_seed::<AuraId>("Bob"),
				],
				vec![
					(get_account_id_from_dev_seed::<sr25519::Public>("Alice"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Bob"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Charlie"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Dave"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Eve"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Ferdie"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Alice//stash"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Bob//stash"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Charlie//stash"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Dave//stash"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Eve//stash"), 1 << 60),
					(get_account_id_from_dev_seed::<sr25519::Public>("Ferdie//stash"), 1 << 60),
				],
				id,
			)
		},
		Vec::new(),
		None,
		Some("kpron"),
		chain_properties(),
		Extensions {
			relay_chain, // You MUST set this to the correct network!
			para_id: id.into(),
		},
	)
}

fn generate_genesis(
	root_key: AccountId,
	initial_authorities: Vec<AuraId>,
	endowed_accounts: Vec<(AccountId, u128)>,
	id: ParaId,
) -> kpron_runtime::GenesisConfig {
	kpron_runtime::GenesisConfig {
		frame_system: kpron_runtime::SystemConfig {
			code: kpron_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		pallet_balances: kpron_runtime::BalancesConfig {
			balances: endowed_accounts,
		},
		pallet_sudo: kpron_runtime::SudoConfig { key: root_key },
		parachain_info: kpron_runtime::ParachainInfoConfig { parachain_id: id },
		pallet_aura: kpron_runtime::AuraConfig {
			authorities: initial_authorities,
		},
		cumulus_pallet_aura_ext: Default::default(),
	}
}

fn chain_properties() -> Option<Properties> {
	let mut p = Properties::new();

	p.insert("tokenSymbol".into(), "KPN".into());
	p.insert("tokenDecimals".into(), 12.into());
	p.insert("ss58Format".into(), 42.into());

	Some(p)
}


#[cfg(test)]
mod spec_tests {
	use super::*;
	use std::str::FromStr;
	use sp_core::crypto::Ss58Codec;
	use sp_core::ed25519;

	#[test]
	fn test_get_account() {
		let addr_str = "xx";
		let public_str = "xx";
		let seed_str = "xx";

		let result = AccountId::from_str(addr_str);
		println!("from str: {}", result.unwrap());
		let result = AccountId::from_str(public_str);
		let account = result.unwrap();
		println!("from str: {}", account);

		let result1 = AuraId::from_string(addr_str);
		let public = result1.unwrap();
		println!("AuraId, {}", public.to_ss58check());

		let result1 = sr25519::Public::from_string(public_str);
		let public = result1.unwrap();
		println!("sr25519::Public, {}", public.to_ss58check());

		let addr1 = get_from_seed::<AuraId>(seed_str);
		let addr2 = get_from_seed::<sr25519::Public>(seed_str);
		let addr3 = get_from_seed::<ed25519::Public>(seed_str);
		println!("addr1: {} \naddr2: {}\naddr3: {}", addr1, addr2, addr3);

	}
}
