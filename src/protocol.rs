
use as_types::{ActivityStreamEntity, Ztm};
use multihash::Multihash;

pub enum KeyAlgo {

}

pub struct MultiKey<'a> {
	alg: KeyAlgo,
	key: &'a [u8]
}

/// The threshold to reach (greater or eaqual than)
/// through the provided signatures to consider this
/// block as valid. Represented as a mathematical fraction:
/// nominator / denominators (this order). Both can be any
/// value from 1-255, allowing for very complex thresholds.
/// Remember, however, that each key's weight is calculated
/// in relation to the sum of all weights, so while a fraction
/// higher than 1/1 is type safe, it can never be reach. 
/// 0, while technically possible to express with this u8 is
/// not allowed. Any Configuration that contains a threshold with
/// 0 in either field (or both) must is invalid.   
type AuthThresholdFraction = (u8, u8);

/// Custom Thresdholds for different actions
pub enum AuthThreshold {
	/// Thresdhold required for a ciphered block
	Cipher(AuthThresholdFraction),
	/// Thresdhold required for a public text block
	Public(AuthThresholdFraction),
	/// Thresdhold required for a cipher text block
	Upgrade(AuthThresholdFraction),
	/// Thresdhold required for a merge block
	Merge(AuthThresholdFraction),
	/// Threshold for all actions, primarily as a fallback
	All(AuthThresholdFraction),
}

/// Settings to sign a specific block
pub struct SignSettings<'a> {
	/// A list of public keys and their respective weight
	/// the weight is calculated as a ratio of the sum of all weights
	weights: Vec<(MultiKey<'a>, u8)>,
	/// for individual actions specific thresholds can be configured
	/// first match wins, if no match is found, a threshold of 1/1 is
	/// required for the block
	thresholds: Vec<AuthThreshold>
}

/// Payload of a Block
pub enum Payload {
	Plain(Ztm<ActivityStreamEntity>),
	Cipher(Vec<u8>)
}


/// A Block in the chain
pub enum Block<'a> {
	/// Starting Block for any chain
	Genesis {
		/// hash over key + content + signature
		id: Multihash<'a>,
		/// public key
		config: SignSettings<'a>,
		/// any extra payload
		data: Payload,
		/// Signature over key + data
		sign: Vec<MultiKey<'a>>
	},
	/// Just a regular content block, which contains
	/// either public content (and requires AuthThreshold::Public to
	/// have been reached) or 
	Regular {
		/// hash over prev + data + sign
		id: Multihash<'a>,
		/// The previous hash-ids
		prev: Multihash<'a>,
		/// the content of this Block
		data: Payload,
		/// Signature over prev + data
		sign: Vec<MultiKey<'a>>
	},
	/// Upgrade the chain configuration, set new keys,
	Upgrade {
		/// hash over parents + config + sign
		id: Multihash<'a>,
		/// The previous hash-ids
		prev: Multihash<'a>,
		/// the new config valid from this block
		config: SignSettings<'a>,
		/// Signature over prev + config, valid at 'prev
		sign: Vec<MultiKey<'a>>
	},
	/// Merge two heads back into one chain. optionally
	/// provide data and configuration if
	Merge  {
		/// hash over parents + data + sign
		id: Multihash<'a>,
		/// The previous hash-ids to merge
		parents: Vec<Multihash<'a>>,
		/// optional payload merging content of Blocks
		data: Option<Payload>,
		/// optional configuration valid from now
		/// _must be present_ if both chains had Upgrade-Blocks
		/// since they diverged
		/// if present both `Merge` and `Upgrade`-Threshold
		/// must be met
		config: Option<SignSettings<'a>>,
		/// Signature over prev + data
		sign: Vec<MultiKey<'a>>

	},
}