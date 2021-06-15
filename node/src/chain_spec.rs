use cumulus_primitives_core::ParaId;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup, Properties};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_core::crypto::Ss58Codec;
use kpron_runtime::constants::currency::{EXISTENTIAL_DEPOSIT, SYMBOL, DECIMALS};
use kpron_runtime::constants::address::{SS58_PREFIX};
use statemint_common::{
	Signature, AccountId, AuraId,
};

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
/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_dev_seed(seed: &str) -> AuraId {
	get_from_dev_seed::<AuraId>(seed)
}


/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn kpron_session_keys(keys: AuraId) -> kpron_runtime::opaque::SessionKeys {
	kpron_runtime::opaque::SessionKeys { aura: keys }
}


pub fn kpron_config(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Kpron",
		// ID
		"kpron",
		ChainType::Live,
		move || {
			generate_genesis(
				vec![
					(
						AccountId::from_string("5Dh7s4b8rs2emsq7hvvpTJTtJwNXoGxHbbrxwBGeNd9VuXo1").unwrap(),
						AuraId::from_string("5Dh7s4b8rs2emsq7hvvpTJTtJwNXoGxHbbrxwBGeNd9VuXo1").unwrap()
					),
					(
						AccountId::from_string("5GYqdDCfzTExVaUbZ3neycG6mR8iFrYFK6HPJhJictBuksRj").unwrap(),
						AuraId::from_string("5GYqdDCfzTExVaUbZ3neycG6mR8iFrYFK6HPJhJictBuksRj").unwrap()
					),
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
			relay_chain: "kusama".into(),
			para_id: id.into(),
		},
	)
}


pub fn kpron_testnet_config(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Kpron testnet",
		// ID
		"kpron_testnet",
		ChainType::Custom(String::from("Test")),
		move || {
			generate_genesis(
				vec![
					(
						AccountId::from_string("5Dh7s4b8rs2emsq7hvvpTJTtJwNXoGxHbbrxwBGeNd9VuXo1").unwrap(),
						AuraId::from_string("5Dh7s4b8rs2emsq7hvvpTJTtJwNXoGxHbbrxwBGeNd9VuXo1").unwrap()
					),
					(
						AccountId::from_string("5GYqdDCfzTExVaUbZ3neycG6mR8iFrYFK6HPJhJictBuksRj").unwrap(),
						AuraId::from_string("5GYqdDCfzTExVaUbZ3neycG6mR8iFrYFK6HPJhJictBuksRj").unwrap()
					),
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
			relay_chain: "westend".into(),
			para_id: id.into(),
		},
	)
}

pub fn kpron_dev_config(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Kpron Development",
		// ID
		"kpron_dev",
		ChainType::Development,
		move || {
			test_generate_genesis(
				get_account_id_from_dev_seed::<sr25519::Public>("Alice"),
				vec![
					(
						get_account_id_from_dev_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_dev_seed("Alice"),
					),
					(
						get_account_id_from_dev_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_dev_seed("Bob"),
					)
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
			relay_chain: "kusama-dev".into(),
			para_id: id.into(),
		},
	)
}

pub fn kpron_local_config(id: ParaId) -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Kpron Local",
		// ID
		"kpron_local",
		ChainType::Local,
		move || {
			test_generate_genesis(
				get_account_id_from_dev_seed::<sr25519::Public>("Alice"),
				vec![
					(
						get_account_id_from_dev_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_dev_seed("Alice"),
					),
					(
						get_account_id_from_dev_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_dev_seed("Bob"),
					)
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
			relay_chain: "kusama-local".into(),
			para_id: id.into(),
		},
	)
}

fn generate_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<(AccountId, u128)>,
	id: ParaId,
) -> kpron_runtime::GenesisConfig {
	// TODO check invulnerables balance > STATEMINE_ED * 16
	kpron_runtime::GenesisConfig {
		system: kpron_runtime::SystemConfig {
			code: kpron_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: kpron_runtime::BalancesConfig {
			balances: endowed_accounts,
		},
		parachain_info: kpron_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: kpron_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16, //16KPN
			..Default::default()
		},
		session: kpron_runtime::SessionConfig {
			keys: invulnerables.iter().cloned().map(|(acc, aura)| (
				acc.clone(), // account id
				acc.clone(), // validator id
				kpron_session_keys(aura), // session keys
			)).collect()
		},
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
	}
}

fn test_generate_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<(AccountId, u128)>,
	id: ParaId,
) -> kpron_runtime::GenesisConfig {
	// TODO check invulnerables balance > EXISTENTIAL_DEPOSIT * 16
	kpron_runtime::GenesisConfig {
		system: kpron_runtime::SystemConfig {
			code: kpron_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: kpron_runtime::BalancesConfig {
			balances: endowed_accounts,
		},
		parachain_info: kpron_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: kpron_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16, // 16KPN
			..Default::default()
		},
		session: kpron_runtime::SessionConfig {
			keys: invulnerables.iter().cloned().map(|(acc, aura)| (
				acc.clone(), // account id
				acc.clone(), // validator id
				kpron_session_keys(aura), // session keys
			)).collect()
		},
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
	}
}

fn chain_properties() -> Option<Properties> {
	let mut p = Properties::new();
	p.insert("tokenSymbol".into(), SYMBOL.into());
	p.insert("tokenDecimals".into(), DECIMALS.into());
	p.insert("ss58Format".into(), SS58_PREFIX.into());
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
