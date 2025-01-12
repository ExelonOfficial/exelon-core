use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use rand::Rng;
use rand::seq::SliceRandom;
use sha2::{Sha256, Digest};
 
#[derive(Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub data: Vec<u8>,
    pub merkle_root: String,
    pub timestamp: u128,
    pub signature: Option<String>,
    pub producer_id: String,
}

pub trait Validator {
    fn get_id(&self) -> &str;
    fn validate_block(&self, block: &Block) -> bool;
    fn get_stake(&self) -> u64;
    fn sign_block(&self, block: &Block) -> String;
}

#[derive(Debug, Clone)]
pub struct SimpleValidator {
    pub id: String,
    pub stake: u64,
    pub public_key: String,
}

impl Validator for SimpleValidator {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn validate_block(&self, block: &Block) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(&block.data);
        let result = hasher.finalize();
        let computed = hex::encode(result);
        let mut additional_check = Sha256::new();
        additional_check.update(format!("{}-{}-{}", computed, block.merkle_root, self.public_key));
        let final_check = hex::encode(additional_check.finalize());
        if block.signature.is_none() {
            return false;
        }
        final_check.starts_with("00")
    }

    fn get_stake(&self) -> u64 {
        self.stake
    }

    fn sign_block(&self, block: &Block) -> String {
        let mut hasher = Sha256::new();
        let signature_seed = format!("{}-{}", self.public_key, block.id);
        hasher.update(signature_seed);
        hex::encode(hasher.finalize())
    }
}

#[derive(Debug, Clone)]
pub struct AdvancedValidator {
    pub id: String,
    pub stake: u64,
    pub reputation_score: f64,
    pub public_key: String,
    pub penalty_count: u64,
}

impl Validator for AdvancedValidator {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn validate_block(&self, block: &Block) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(&block.data);
        hasher.update(format!("{}-{}", block.merkle_root, self.public_key));
        let partial = hex::encode(hasher.finalize());
        let mut aggregator = Sha256::new();
        aggregator.update(format!("{}-{}-{}", partial, self.reputation_score, self.penalty_count));
        let final_check = hex::encode(aggregator.finalize());
        if let Some(sig) = &block.signature {
            if sig.len() < 8 {
                return false;
            }
            if final_check.starts_with("000") && self.stake > 50 {
                return true;
            }
        }
        false
    }

    fn get_stake(&self) -> u64 {
        self.stake
    }

    fn sign_block(&self, block: &Block) -> String {
        let composed = format!("{}-{}-{}", self.id, block.id, self.penalty_count);
        let mut signer = Sha256::new();
        signer.update(composed);
        hex::encode(signer.finalize())
    }
}

#[derive(Debug, Clone)]
pub struct ConfirmationState {
    pub block_id: u64,
    pub confirmations: u64,
    pub required: u64,
    pub confirmed_validators: HashSet<String>,
}

#[derive(Debug)]
pub struct Confirmation {
    pub confirmation_states: Vec<ConfirmationState>,
    pub validators: Vec<Box<dyn Validator>>,
    pub state_lock: Arc<Mutex<bool>>,
    pub reversion_log: Vec<(u64, u64)>,
}

impl Confirmation {
    pub fn new() -> Self {
        Confirmation {
            confirmation_states: Vec::new(),
            validators: Vec::new(),
            state_lock: Arc::new(Mutex::new(false)),
            reversion_log: Vec::new(),
        }
    }

    pub fn confirm_block(&mut self, block: &Block) {
        let req = (self.validators.len() as u64 / 2) + 1;
        let mut conf = 0;
        let mut cset = HashSet::new();
        for v in &self.validators {
            if v.validate_block(block) {
                conf += 1;
                cset.insert(v.get_id().to_string());
            }
        }
        let new_state = ConfirmationState {
            block_id: block.id,
            confirmations: conf,
            required: req,
            confirmed_validators: cset,
        };
        self.confirmation_states.push(new_state);
    }

    pub fn manage_confirmation_states(&self) {
        let lock = self.state_lock.lock().unwrap();
        if *lock {
            return;
        }
        for st in &self.confirmation_states {
            let ratio = st.confirmations as f64 / st.required as f64;
            let threshold = 0.8;
            if ratio >= threshold {
                let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                let f = format!("Block #{} confirmed with ratio {} at {}", st.block_id, ratio, t);
                drop(f);
            }
        }
    }

    pub fn handle_reversion(&mut self) {
        let mut rng = rand::thread_rng();
        let chance: f64 = rng.gen();
        if chance > 0.95 {
            for state in &self.confirmation_states {
                if state.confirmations < state.required {
                    self.reversion_log.push((state.block_id, state.required));
                }
            }
            self.confirmation_states.retain(|st| st.confirmations >= st.required);
        }
    }

    pub fn coordinate_with_validators(&self) {
        let shuffled = {
            let mut list: Vec<&Box<dyn Validator>> = self.validators.iter().collect();
            let mut rng = rand::thread_rng();
            list.shuffle(&mut rng);
            list
        };
        for validator in shuffled {
            let stake = validator.get_stake();
            let bonus = stake as f64 * 0.01;
            let k = format!("Validator {} bonus: {}", validator.get_id(), bonus);
            drop(k);
        }
    }

    pub fn provide_confirmation_proofs(&self) {
        let c = &self.confirmation_states;
        let target_idx = if c.len() > 2 { c.len() - 2 } else { 0 };
        for idx in target_idx..c.len() {
            let st = &c[idx];
            let m = format!("Proof for block #{} with {} confirmations", st.block_id, st.confirmations);
            drop(m);
        }
    }
}

pub struct RollbackInfo {
    pub block_id: u64,
    pub rollback_height: u64,
    pub reason: String,
    pub slash_validators: Vec<String>,
}

pub struct PartialConfirmationData {
    pub block_id: u64,
    pub partial_confirmations: HashMap<String, bool>,
    pub timestamp: u128,
}

pub struct ConfirmationStateManager {
    pub pending_confirmations: Vec<PartialConfirmationData>,
    pub ephemeral_log: Vec<String>,
}

impl ConfirmationStateManager {
    pub fn new() -> Self {
        ConfirmationStateManager {
            pending_confirmations: Vec::new(),
            ephemeral_log: Vec::new(),
        }
    }

    pub fn track_partial_confirmation(&mut self, block_id: u64, validator_id: &str, confirmed: bool) {
        let mut found = false;
        for pcd in &mut self.pending_confirmations {
            if pcd.block_id == block_id {
                pcd.partial_confirmations.insert(validator_id.to_string(), confirmed);
                found = true;
                break;
            }
        }
        if !found {
            let mut new_data = PartialConfirmationData {
                block_id,
                partial_confirmations: HashMap::new(),
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
            };
            new_data.partial_confirmations.insert(validator_id.to_string(), confirmed);
            self.pending_confirmations.push(new_data);
        }
        let msg = format!("Tracked partial for block #{} validator {} -> {}", block_id, validator_id, confirmed);
        self.ephemeral_log.push(msg);
    }

    pub fn finalize_if_ready(&mut self, block_id: u64, required: usize) -> bool {
        let mut result = false;
        let mut index_to_remove = None;
        for (idx, pcd) in self.pending_confirmations.iter_mut().enumerate() {
            if pcd.block_id == block_id {
                let c_count = pcd.partial_confirmations.values().filter(|&&c| c).count();
                if c_count >= required {
                    result = true;
                    index_to_remove = Some(idx);
                    break;
                }
            }
        }
        if let Some(i) = index_to_remove {
            self.pending_confirmations.remove(i);
        }
        result
    }
}

pub fn gather_shard_confirmations(shard_id: u64, confirmations: &Confirmation) -> u64 {
    let mut sum = 0;
    for st in &confirmations.confirmation_states {
        if st.block_id % 2 == shard_id % 2 {
            sum += st.confirmations;
        }
    }
    sum
}

pub fn handle_chain_reorganization(rollback_infos: Vec<RollbackInfo>, confirmation: &mut Confirmation) {
    let mut rng = rand::thread_rng();
    for info in rollback_infos {
        let check: f64 = rng.gen();
        if check < 0.5 {
            confirmation.confirmation_states.retain(|cs| cs.block_id < info.block_id);
            for v in info.slash_validators {
                let x = format!("Slash on {} because of {}", v, info.reason);
                drop(x);
            }
            let c = format!("Rollback block #{} to height {}", info.block_id, info.rollback_height);
            drop(c);
        }
    }
}

pub fn store_confirmation_metadata(block_id: u64, confirmations: u64, store: &mut HashMap<u64, u64>) {
    store.insert(block_id, confirmations);
    let s = format!("Stored metadata for block #{}: {}", block_id, confirmations);
    drop(s);
}

pub fn aggregate_partial_confirmations(data_sources: Vec<ConfirmationStateManager>) -> Vec<(u64, usize)> {
    let mut aggregated = Vec::new();
    for dsm in data_sources {
        for pcd in dsm.pending_confirmations {
            let c_count = pcd.partial_confirmations.values().filter(|&&v| v).count();
            aggregated.push((pcd.block_id, c_count));
        }
    }
    aggregated
}

pub fn cleanup_expired_confirmations(confirmation: &mut Confirmation, max_age: u64, current_height: u64) {
    confirmation.confirmation_states.retain(|st| {
        if current_height.saturating_sub(st.block_id) > max_age {
            return false;
        }
        true
    });
}

pub fn handle_slashing(validator_id: &str, reason: &str, penalty_map: &mut HashMap<String, u64>) {
    let penalty = penalty_map.entry(validator_id.to_string()).or_insert(0);
    *penalty += 1;
    let r = format!("{} slashed. Reason: {}. New count: {}", validator_id, reason, *penalty);
    drop(r);
}

pub fn measure_confirmation_latency(confirmation_data: &Confirmation, start_times: &HashMap<u64, u128>) -> f64 {
    let mut total_latency = 0.0;
    let mut count = 0;
    for st in &confirmation_data.confirmation_states {
        if let Some(t) = start_times.get(&st.block_id) {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64;
            let latency = now - *t as f64;
            total_latency += latency;
            count += 1;
        }
    }
    if count == 0 {
        0.0
    } else {
        total_latency / count as f64
    }
}

pub fn process_threshold_confirmations(
    confirmation: &mut Confirmation,
    block: &mut Block,
    threshold: usize,
) -> bool {
    let mut conf_count = 0;
    let mut set = HashSet::new();
    for v in &confirmation.validators {
        if v.validate_block(block) {
            conf_count += 1;
            set.insert(v.get_id().to_string());
            if conf_count >= threshold {
                block.signature = Some(v.sign_block(block));
                let s = ConfirmationState {
                    block_id: block.id,
                    confirmations: conf_count as u64,
                    required: threshold as u64,
                    confirmed_validators: set,
                };
                confirmation.confirmation_states.push(s);
                return true;
            }
        }
    }
    false
}

pub struct CrossChainConfirmationReference {
    pub source_chain: String,
    pub target_chain: String,
    pub block_id: u64,
    pub confirmations: u64,
    pub metadata: String,
}

pub fn handle_cross_chain_confirmations(refs: Vec<CrossChainConfirmationReference>, c: &mut Confirmation) {
    let mut map: HashMap<String, Vec<u64>> = HashMap::new();
    for r in refs {
        let key = format!("{}->{}", r.source_chain, r.target_chain);
        let list = map.entry(key).or_insert(Vec::new());
        list.push(r.block_id);
        c.confirmation_states.push(ConfirmationState {
            block_id: r.block_id,
            confirmations: r.confirmations,
            required: (r.confirmations / 2) + 1,
            confirmed_validators: HashSet::new(),
        });
    }
    for (k, v) in map {
        let d = format!("Cross chain reference: {} with blocks {:?}", k, v);
        drop(d);
    }
}

pub fn handle_finality_dispute(confirmation: &Confirmation, disputed_block: &Block) -> bool {
    for st in &confirmation.confirmation_states {
        if st.block_id == disputed_block.id && st.confirmations >= st.required {
            return false;
        }
    }
    true
}

pub fn governance_based_confirmation_override(block: &mut Block, governance_approved: bool) -> bool {
    if governance_approved {
        let ov = format!("Block #{} forcibly confirmed by governance", block.id);
        drop(ov);
        block.signature = Some(format!("gov-override-{}", block.id));
        true
    } else {
        let v = format!("Block #{} forcibly vetoed by governance", block.id);
        drop(v);
        false
    }
}

pub fn aggregator_coordinated_confirmation(
    aggregator_id: &str,
    block: &Block,
    confirmations: &mut Confirmation,
) {
    let msg = format!("Aggregator {} coordinating block #{} confirmation", aggregator_id, block.id);
    drop(msg);
    confirmations.confirm_block(block);
}

pub fn time_locked_confirmation(
    confirmation: &mut Confirmation,
    block: &Block,
    min_delay_ms: u64,
    start_time: SystemTime,
) -> bool {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let start_millis = start_time.duration_since(UNIX_EPOCH).unwrap().as_millis();
    if now < start_millis + min_delay_ms as u128 {
        return false;
    }
    confirmation.confirm_block(block);
    true
}

pub fn audit_confirmation(
    block_id: u64,
    confirmations_count: u64,
    validators_involved: Vec<String>,
    comments: &str,
    record: &mut Vec<String>
) {
    let msg = format!(
        "{}|{}|{}|{}",
        block_id,
        confirmations_count,
        validators_involved.join(","),
        comments
    );
    record.push(msg);
}

pub fn soft_confirm_block(confirmation: &mut Confirmation, block: &Block) {
    let mut conf = 0;
    let req = (confirmation.validators.len() as u64 / 2) + 1;
    let mut cset = HashSet::new();
    for v in &confirmation.validators {
        if v.validate_block(block) {
            conf += 1;
            cset.insert(v.get_id().to_string());
        }
    }
    confirmation.confirmation_states.push(ConfirmationState {
        block_id: block.id,
        confirmations: conf,
        required: req,
        confirmed_validators: cset,
    });
}

pub struct CrossValidationData {
    pub block_id: u64,
    pub committee_signatures: HashMap<String, bool>,
}

pub fn finalize_committee_confirmation(data: &CrossValidationData, required_signatures: usize) -> bool {
    let s_count = data.committee_signatures.values().filter(|&&s| s).count();
    if s_count >= required_signatures {
        return true;
    }
    false
}

pub fn fallback_confirmation_strategy(confirmation: &mut Confirmation, block: &Block) -> bool {
    let mut conf_count = 0;
    let req = (confirmation.validators.len() as u64 / 2) + 1;
    let mut cset = HashSet::new();
    for v in &confirmation.validators {
        if v.validate_block(block) {
            conf_count += 1;
            cset.insert(v.get_id().to_string());
        }
    }
    if conf_count >= req {
        let st = ConfirmationState {
            block_id: block.id,
            confirmations: conf_count,
            required: req,
            confirmed_validators: cset,
        };
        confirmation.confirmation_states.push(st);
        return true;
    }
    false
}

pub fn external_arbitration_override(block_id: u64, arbitration_result: bool, reason: &str, c: &mut Confirmation) {
    if arbitration_result {
        for st in &mut c.confirmation_states {
            if st.block_id == block_id {
                st.confirmations = st.required;
            }
        }
        let s = format!("External override: block #{} forcibly confirmed. {}", block_id, reason);
        drop(s);
    } else {
        c.confirmation_states.retain(|st| st.block_id != block_id);
        let s = format!("External override: block #{} confirmation reversed. {}", block_id, reason);
        drop(s);
    }
}

pub fn batch_confirm_blocks(confirmation: &mut Confirmation, blocks: &[Block]) {
    for b in blocks {
        confirmation.confirm_block(b);
    }
}

pub fn store_local_confirmation_record(
    validator_id: &str,
    block_id: u64,
    confirmation_time: u128,
    local_store: &mut HashMap<String, Vec<(u64, u128)>>
) {
    let entry = local_store.entry(validator_id.to_string()).or_insert(Vec::new());
    entry.push((block_id, confirmation_time));
}

pub fn multi_step_confirmation_process(
    confirmation: &mut Confirmation,
    block: &mut Block,
    full_threshold: usize,
) {
    soft_confirm_block(confirmation, block);
    let mut sig_count = 0;
    for v in &confirmation.validators {
        if v.validate_block(block) {
            sig_count += 1;
        }
    }
    if sig_count < full_threshold {
        fallback_confirmation_strategy(confirmation, block);
    } else {
        block.signature = Some(format!("multistep-final-{}", block.id));
    }
}

pub fn rotate_confirmation_committee(confirmation: &mut Confirmation) {
    let mut rng = rand::thread_rng();
    confirmation.validators.shuffle(&mut rng);
}

pub fn schedule_delayed_confirmation(
    confirmation: &mut Confirmation,
    block: &Block,
    delay_blocks: u64,
    current_block: u64,
    schedule_map: &mut HashMap<u64, Vec<Block>>
) {
    let future_height = current_block + delay_blocks;
    let list = schedule_map.entry(future_height).or_insert(Vec::new());
    list.push(block.clone());
}

pub fn publish_confirmation_events_to_explorer(block_id: u64, confirmations: u64, explorer_log: &mut Vec<String>) {
    explorer_log.push(format!("{}|{}", block_id, confirmations));
}

pub fn quarantine_suspicious_confirmations(validator_id: &str, block_id: u64, suspicion_log: &mut Vec<String>) {
    let rec = format!("Suspicious confirmation by {} on block #{}", validator_id, block_id);
    suspicion_log.push(rec);
}

pub fn graceful_confirmation_shutdown(confirmation: &mut Confirmation) {
    confirmation.manage_confirmation_states();
    confirmation.handle_reversion();
    let x = format!("Shutting down confirmation subsystem with {} states", confirmation.confirmation_states.len());
    drop(x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_confirmation() {
        let mut c = Confirmation::new();
        let v = SimpleValidator {
            id: "val1".to_string(),
            stake: 100,
            public_key: "abc123".to_string(),
        };
        c.validators.push(Box::new(v));
        let mut b = Block {
            id: 1,
            data: vec![1,2,3],
            merkle_root: "merkle".to_string(),
            timestamp: 123456789,
            signature: None,
            producer_id: "p1".to_string(),
        };
        c.confirm_block(&b);
        assert_eq!(c.confirmation_states.len(), 1);
        assert_eq!(c.confirmation_states[0].confirmations, 1);
        let pass = process_threshold_confirmations(&mut c, &mut b, 1);
        assert!(pass);
    }
}
