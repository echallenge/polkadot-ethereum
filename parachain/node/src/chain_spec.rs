use sp_core::{Pair, Public, sr25519};
use artemis_runtime::{
	AccountId, EthereumHeader,
	AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SystemConfig, VerifierConfig, VerifierLightclientConfig,
	ETHConfig, ERC20Config,
	WASM_BINARY, Signature,
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;

use hex_literal::hex;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Aura
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
			// Relay account
			get_account_id_from_seed::<sr25519::Public>("Relay"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Relay"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Relay//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			// Relay Account
			get_account_id_from_seed::<sr25519::Public>("Relay"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Relay"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Relay//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	relay_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		pallet_aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		verifier: Some(VerifierConfig {
			key: relay_key,
		}),
		verifier_lightclient: Some(VerifierLightclientConfig {
			initial_header: EthereumHeader {
				parent_hash: hex!("a0e69c4c28dffaf0b5819ba776a237ce1dcd589013e9665bb70852d81113b43e").into(),
				timestamp: 0x5fb73641u64.into(),
				number: 0xac4f62u64.into(),
				author: hex!("ea674fdde714fd979de3edf0f56aa9716b898ec8").into(),
				transactions_root: hex!("ac1717f7af2e70b90310404bb14e1a1f80d90367c92acf189e033936378f3f0d").into(),
				ommers_hash: hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347").into(),
				extra_data: hex!("65746865726d696e652d65752d6e6f727468312d322d67657468").into(),
				state_root: hex!("937a6c31254223743757dd919b27abc685b543c37846d1e40eb7b785ffe812af").into(),
				receipts_root: hex!("a0fd611f9c656561e99b4496681501e6a8d26495b41b2149610aa207e4c73aae").into(),
				logs_bloom: (&hex!("1434538c456ec5545da036c0e1d13aa6690b8e90440ea22e6185186c00ab121adc91e040614014116805c13610225d0217606196c91711450388c06cd374343612a08241a6c3c60b4c89934800c44a669a0521550a4456005b8211788dc455981808253612288044b776072064231a28421c92a227912c142bad78323780726288b78641261e2e1ed64322aa00a818a608466da31123445bc422004fa617182d0388e1806820231224752190e019188132002803a15b1cc0000eb600a122a38d240b810a9068a004b413ca86014384ae4502bb07906d8112110982a318602029849835141b36104dd2021c8e93b02880403108aa690006f6409084063833043a")).into(),
				gas_used: 0xbe15e5.into(),
				gas_limit: 0xbe2af0.into(),
				difficulty: 0xc70c5a3bbfffeu64.into(),
				seal: vec![
					vec![160, 217, 57, 103, 159, 35, 90, 12, 201, 122, 168, 193, 234, 216, 117, 122, 54, 28, 93, 38, 100, 173, 87, 100, 242, 223, 128, 178, 154, 53, 179, 62, 111],
					vec![136, 245, 207, 159, 155, 31, 114, 21, 17],
				],
			},
			initial_difficulty: 0x3fc9cb4448510a2b525u128.into(),
		}),
		eth_app: Some(ETHConfig {
			address: hex!["fc97a6197dc90bef6bbefd672742ed75e9768553"].into()
		}),
		erc20_app: Some(ERC20Config {
			address: hex!["eda338e4dc46038493b885327842fd3e301cab39"].into()
		})
	}
}
