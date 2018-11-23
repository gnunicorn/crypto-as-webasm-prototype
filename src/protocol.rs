
use as_types::{ActivityStreamEntity, Ztm};
use multihash::Multihash;

/// An enum of different KeyAlgorithms
pub enum KeyAlgo {
//TBD, e.g. pgp, ed25519, etc

}
/// A struct similar to a MultiHash:  prefixed with the
/// Algorithm used and followed by the content
pub struct MultiKey<'a> {
	alg: KeyAlgo,
	key: &'a [u8]
}

/// The threshold to reach (greater or equal than)
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

/// Custom Thresholds for different actions
pub enum AuthThreshold {
	/// Threshold required for a ciphered block
	Cipher(AuthThresholdFraction),
	/// Threshold required for a public text block
	Public(AuthThresholdFraction),
	/// Threshold required for a block that upgrades the permissions
	Upgrade(AuthThresholdFraction),
	/// Threshold required for a snapshot block
	Snapshot(AuthThresholdFraction),
	/// Threshold required for a merge block
	Merge(AuthThresholdFraction),
	/// Threshold for all actions, primarily as a fallback
	All(AuthThresholdFraction),
}

/// Settings to sign a specific block
pub struct SignSettings<'a> {
	/// A list of public keys and their respective weight
	/// the weight is calculated as a ratio compared to the sum of all weights
	weights: Vec<(MultiKey<'a>, u8)>,
	/// for individual actions specific thresholds can be configured
	/// first match wins, if no match is found, a threshold of 1/1 is
	/// required for the block
	thresholds: Vec<AuthThreshold>
}

/// Payload of a Block
pub enum Payload {
	Plain(Ztm<ActivityStreamEntity>),
	Cipher(Vec<u8>) // Q: make this a multi-cypher-type obj?
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
		/// Signature over parents + data
		sign: Vec<MultiKey<'a>>
	},
	Snapshot {
		/// hash over parents + data + sign
		id: Multihash<'a>,
		/// The hash-id of the previous Snapshot,
		/// or Genesis-Block, if first, containing
		/// the calculated diff to that snapshot
		prev: Multihash<'a>,
		/// The Block this Snapshot was created at 
		/// on the regular chain
		at: Multihash<'a>,
		/// payload merging content of Blocks
		data: Option<Payload>,
		/// optional configuration valid from at foregoing
		/// if any changes to the configuration had happened
		/// between the `.prev.at` and `.at` on the change,
		/// they _must_ be present here in their final form
		/// Peers must check this and reject the block if that
		/// is not the case
		config: Option<SignSettings<'a>>,
		/// Signature over prev + at + data + config
		sign: Vec<MultiKey<'a>>
	}
}