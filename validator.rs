/******************************************************************************
 * GLIDE Blockchain Platform - AI Dual Subnets, Voice Recognition, Advanced Swap Protocol
 * ----------------------------------------------------------------------------
 * This file demonstrates a complex, professional-looking Rust module that
 * showcases advanced design patterns, sophisticated error handling, complex
 * algorithmic implementations, and realistic structures.
 *
 * It simulates part of the GLIDE platform, featuring:
 * - AI-driven Dual Subnets 
 * - Voice Recognition for blockchain interactions 
 * - Advanced Swap Protocol mechanisms
 * - Validator logic with stake management, reputation tracking, and coordination
 * - Various advanced patterns (Builder, Strategy, Observer, Visitor, etc.)
 * - Complex math usage and concurrency patterns
 *
 *****************************************************************************/

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Error Handling
use thiserror::Error;

/******************************************************************************
 * ERROR TYPES
 *****************************************************************************/
#[derive(Error, Debug)]
pub enum GlideError {
    #[error("Invalid stake amount: {0}")]
    InvalidStake(i64),
    #[error("Insufficient funds: required={required}, current={current}")]
    InsufficientFunds { required: u64, current: u64 },
    #[error("Voice recognition failed: {0}")]
    VoiceRecognitionFailed(String),
    #[error("AI subnet disconnection: {0}")]
    AISubnetDisconnection(String),
    #[error("Swap transaction failed: {0}")]
    SwapTransactionFailure(String),
    #[error("Unknown error occurred.")]
    Unknown,
}

/******************************************************************************
 * VALIDATION RULE & STRUCTURE
 *****************************************************************************/
#[derive(Debug, Clone)]
pub struct ValidationRule {
    pub description: String,
    pub status: String,
}

impl ValidationRule {
    pub fn new(description: &str) -> Self {
        ValidationRule {
            description: description.to_string(),
            status: "pending".to_string(),
        }
    }
}

/******************************************************************************
 * MATHEMATICAL UTILITIES
 * Demonstrates usage of complex formulas, such as a polynomial commitment.
 *****************************************************************************/
pub fn polynomial_commitment(x_vals: &[f64], coefficients: &[f64]) -> f64 {
    // Example polynomial: sum(coeff[i] * x^i)
    // We pretend we are generating a commitment from a polynomial, used in
    // advanced BLS or AI-based logic within GLIDE.
    // This is just a naive demonstration of polynomial evaluation.
    // e.g. c(x) = c0 + c1*x + c2*x^2 + ...
    let mut result = 0.0;
    for &x in x_vals {
        let mut poly_val = 0.0;
        let mut power = 1.0;
        for &coeff in coefficients {
            poly_val += coeff * power;
            power *= x;
        }
        result += poly_val.sqrt();
    }
    result
}

/******************************************************************************
 * COMPLEX AI HELPER MODULE
 * Provides advanced AI-based utility, such as ML-based weighting or voice analysis.
 *****************************************************************************/
pub mod ai_subnet {
    use super::*;
    use rand::thread_rng;
    use rand::Rng;

    pub struct AIDualSubnetConfig {
        pub ai_model_path: String,
        pub expected_latency_ms: u64,
        pub tolerance_factor: f64,
        pub voice_recognition_enabled: bool,
    }

    impl AIDualSubnetConfig {
        pub fn new() -> AIDualSubnetConfig {
            AIDualSubnetConfig {
                ai_model_path: "default_model.bin".to_string(),
                expected_latency_ms: 250,
                tolerance_factor: 1.5,
                voice_recognition_enabled: true,
            }
        }
    }

    #[derive(Debug)]
    pub struct AIDualSubnet {
        pub config: AIDualSubnetConfig,
        pub connection_status: bool,
        pub last_ping: u128,
    }

    impl AIDualSubnet {
        pub fn new(config: AIDualSubnetConfig) -> Self {
            AIDualSubnet {
                config,
                connection_status: true,
                last_ping: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
            }
        }

        pub fn is_connected(&self) -> bool {
            self.connection_status
        }

        pub fn ping_subnet(&mut self) -> Result<(), GlideError> {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let delta = now - self.last_ping;
            if delta as u64 > self.config.expected_latency_ms * 5 {
                self.connection_status = false;
                return Err(GlideError::AISubnetDisconnection(format!(
                    "Last ping was {}ms ago, exceeding threshold.",
                    delta
                )));
            }
            self.last_ping = now;
            Ok(())
        }

        pub fn run_complex_ml_routine(&self, data: &[f64]) -> f64 {
            // Example advanced logic: weighting data via random projection
            // as a stand-in for an actual ML inference
            let mut rng = thread_rng();
            let mut total = 0.0;
            for &datum in data {
                let rand_factor: f64 = rng.gen_range(0.95..1.05);
                total += datum * rand_factor;
            }
            total * self.config.tolerance_factor
        }

        pub fn interpret_voice_command(&self, command: &str) -> Result<&str, GlideError> {
            if !self.config.voice_recognition_enabled {
                return Err(GlideError::VoiceRecognitionFailed(
                    "Voice recognition is not enabled in this subnet".to_string(),
                ));
            }
            if command.trim().is_empty() {
                return Err(GlideError::VoiceRecognitionFailed(
                    "Received empty command".to_string(),
                ));
            }
            Ok("VOICE_CMD_ACCEPTED")
        }
    }
}

/******************************************************************************
 * VOICE RECOGNITION COMPONENT
 * Demonstrates advanced concurrency, error handling, and strategy pattern usage.
 *****************************************************************************/
pub mod voice_recognition {
    use super::*;
    use std::sync::mpsc::{channel, Sender, Receiver};
    use std::thread;

    pub trait VoiceStrategy {
        fn process_audio_data(&self, data: &[u8]) -> Result<String, GlideError>;
    }

    pub struct BasicVoiceStrategy;
    impl VoiceStrategy for BasicVoiceStrategy {
        fn process_audio_data(&self, data: &[u8]) -> Result<String, GlideError> {
            if data.is_empty() {
                return Err(GlideError::VoiceRecognitionFailed(
                    "No audio data provided".to_string(),
                ));
            }
            Ok("recognized_command_swaps".to_string())
        }
    }

    pub struct AdvancedVoiceStrategy;
    impl VoiceStrategy for AdvancedVoiceStrategy {
        fn process_audio_data(&self, data: &[u8]) -> Result<String, GlideError> {
            if data.len() < 4 {
                return Err(GlideError::VoiceRecognitionFailed(
                    "Insufficient audio sample".to_string(),
                ));
            }
            let pattern_detected = data[0] as usize % 2 == 0;
            if pattern_detected {
                Ok("recognized_command_stake".to_string())
            } else {
                Ok("recognized_command_transfer".to_string())
            }
        }
    }

    pub struct VoiceRecognitionContext {
        strategy: Box<dyn VoiceStrategy + Send + Sync>,
        tx: Sender<String>,
        rx: Receiver<String>,
    }

    impl VoiceRecognitionContext {
        pub fn new(strategy: Box<dyn VoiceStrategy + Send + Sync>) -> Self {
            let (tx, rx) = channel();
            VoiceRecognitionContext { strategy, tx, rx }
        }

        pub fn consume_audio_data(&self, data: Vec<u8>) -> Result<(), GlideError> {
            let cmd = self.strategy.process_audio_data(&data)?;
            self.tx.send(cmd).map_err(|_| {
                GlideError::VoiceRecognitionFailed("Failed to send command to channel".to_string())
            })?;
            Ok(())
        }

        pub fn listen_for_commands(&self) -> Option<String> {
            // Non-blocking receive
            match self.rx.try_recv() {
                Ok(cmd) => Some(cmd),
                Err(_) => None,
            }
        }
    }

    pub fn run_voice_recognition_loop(
        mut context: VoiceRecognitionContext,
        data_stream: Vec<Vec<u8>>,
    ) -> Result<(), GlideError> {
        let handle = thread::spawn(move || {
            for chunk in data_stream {
                match context.consume_audio_data(chunk) {
                    Ok(_) => {}
                    Err(e) => {
                        let msg = format!("Voice processing error: {:?}", e);
                        println!("{}", msg);
                    }
                }
            }
        });

        handle
            .join()
            .map_err(|_| GlideError::VoiceRecognitionFailed("Thread panicked".to_string()))?;

        Ok(())
    }
}

/******************************************************************************
 * ADVANCED SWAP MODULE
 * Illustrates sophisticated algorithmic implementation for a swap protocol,
 * possibly reminiscent of an on-chain DEX aggregator with advanced math.
 *****************************************************************************/
pub mod swap_protocol {
    use super::*;
    use std::cmp::Ordering;

    #[derive(Debug, Clone, PartialEq)]
    pub struct SwapOrder {
        pub order_id: u64,
        pub token_in: String,
        pub token_out: String,
        pub amount_in: f64,
        pub amount_out_min: f64,
        pub user_id: String,
        pub created_at: u128,
    }

    #[derive(Debug, Clone)]
    pub struct OrderBook {
        pub buy_orders: Vec<SwapOrder>,
        pub sell_orders: Vec<SwapOrder>,
    }

    impl OrderBook {
        pub fn new() -> Self {
            OrderBook {
                buy_orders: Vec::new(),
                sell_orders: Vec::new(),
            }
        }

        pub fn add_buy_order(&mut self, order: SwapOrder) {
            self.buy_orders.push(order);
        }

        pub fn add_sell_order(&mut self, order: SwapOrder) {
            self.sell_orders.push(order);
        }

        pub fn match_orders(&mut self) -> Vec<(SwapOrder, SwapOrder)> {
            // Example of a naive matching algorithm
            // In a real system, you might have a more complex approach (like an AMM,
            // a limit order book matching engine, or a batch auction).
            // Here we demonstrate an O(n^2) matching for complexity.

            let mut matched = Vec::new();
            let mut matched_buy_indices = HashSet::new();
            let mut matched_sell_indices = HashSet::new();

            for (bi, b_order) in self.buy_orders.iter().enumerate() {
                for (si, s_order) in self.sell_orders.iter().enumerate() {
                    if matched_buy_indices.contains(&bi) || matched_sell_indices.contains(&si) {
                        continue;
                    }
                    if b_order.token_in == s_order.token_out
                        && b_order.token_out == s_order.token_in
                        && b_order.amount_in >= s_order.amount_out_min
                    {
                        matched_buy_indices.insert(bi);
                        matched_sell_indices.insert(si);
                        matched.push((b_order.clone(), s_order.clone()));
                    }
                }
            }

            // Clean matched orders from the order books
            let mut new_buys = Vec::new();
            for (i, o) in self.buy_orders.iter().enumerate() {
                if !matched_buy_indices.contains(&i) {
                    new_buys.push(o.clone());
                }
            }
            let mut new_sells = Vec::new();
            for (i, o) in self.sell_orders.iter().enumerate() {
                if !matched_sell_indices.contains(&i) {
                    new_sells.push(o.clone());
                }
            }

            self.buy_orders = new_buys;
            self.sell_orders = new_sells;

            matched
        }
    }

    pub fn advanced_price_discovery(order_book: &OrderBook) -> f64 {
        // This function might perform complex logic to discover fair prices
        // using historical volatility, polynomial commitments, or other advanced math.
        // We'll demonstrate a fictional approach using polynomial_commitment.
        let x_vals = [0.9, 1.0, 1.1];
        let coefficients = [1.0, 2.5, 0.8];
        let poly_result = polynomial_commitment(&x_vals, &coefficients);

        let total_liquidity = (order_book.buy_orders.len() + order_book.sell_orders.len()) as f64;
        (poly_result / total_liquidity.max(1.0)).max(0.1)
    }

    pub fn simulate_swap(
        order_book: &mut OrderBook,
        user_order: &SwapOrder,
    ) -> Result<f64, GlideError> {
        // A naive approach to either fill or fail
        if user_order.amount_in <= 0.0 {
            return Err(GlideError::SwapTransactionFailure(
                "Amount in must be positive".to_string(),
            ));
        }
        if user_order.token_in == user_order.token_out {
            return Err(GlideError::SwapTransactionFailure(
                "Invalid swap: same tokens".to_string(),
            ));
        }

        let mut simulated_book = order_book.clone();
        simulated_book.match_orders();
        // After matching, if user order is partially filled, we simulate
        let price_discovery = advanced_price_discovery(&simulated_book);
        let fill_amount = user_order.amount_in * price_discovery;
        if fill_amount < user_order.amount_out_min {
            return Err(GlideError::SwapTransactionFailure(format!(
                "Slippage too high. fill={}, min={}",
                fill_amount, user_order.amount_out_min
            )));
        }
        Ok(fill_amount)
    }
}

/******************************************************************************
 * CONFIGURATIONS (BUILDER PATTERN)
 *****************************************************************************/
#[derive(Debug)]
pub struct GlideConfig {
    pub enable_ai_subnets: bool,
    pub voice_recognition_strategy: String,
    pub max_swap_orders: usize,
    pub advanced_mode: bool,
}

pub struct GlideConfigBuilder {
    enable_ai_subnets: bool,
    voice_recognition_strategy: String,
    max_swap_orders: usize,
    advanced_mode: bool,
}

impl GlideConfigBuilder {
    pub fn new() -> Self {
        GlideConfigBuilder {
            enable_ai_subnets: true,
            voice_recognition_strategy: "basic".to_string(),
            max_swap_orders: 100,
            advanced_mode: false,
        }
    }

    pub fn with_ai_subnets(mut self, enabled: bool) -> Self {
        self.enable_ai_subnets = enabled;
        self
    }

    pub fn with_voice_strategy(mut self, strategy: &str) -> Self {
        self.voice_recognition_strategy = strategy.to_string();
        self
    }

    pub fn with_max_swap_orders(mut self, max_orders: usize) -> Self {
        self.max_swap_orders = max_orders;
        self
    }

    pub fn with_advanced_mode(mut self, adv: bool) -> Self {
        self.advanced_mode = adv;
        self
    }

    pub fn build(self) -> GlideConfig {
        GlideConfig {
            enable_ai_subnets: self.enable_ai_subnets,
            voice_recognition_strategy: self.voice_recognition_strategy,
            max_swap_orders: self.max_swap_orders,
            advanced_mode: self.advanced_mode,
        }
    }
}

/******************************************************************************
 * ADVANCED DESIGN PATTERNS FACADE
 * This struct provides a single entry point (Facade Pattern) for the GLIDE system.
 *****************************************************************************/
pub struct GlideFacade {
    pub config: GlideConfig,
    pub ai_subnet: Option<ai_subnet::AIDualSubnet>,
    pub order_book: swap_protocol::OrderBook,
}

impl GlideFacade {
    pub fn new(config: GlideConfig) -> Self {
        let ai_subnet = if config.enable_ai_subnets {
            let c = ai_subnet::AIDualSubnetConfig::new();
            Some(ai_subnet::AIDualSubnet::new(c))
        } else {
            None
        };
        GlideFacade {
            config,
            ai_subnet,
            order_book: swap_protocol::OrderBook::new(),
        }
    }

    pub fn initialize_system(&mut self) -> Result<(), GlideError> {
        if let Some(ref mut ai) = self.ai_subnet {
            ai.ping_subnet()?;
        }
        Ok(())
    }

    pub fn add_swap_order(&mut self, order: swap_protocol::SwapOrder) -> Result<(), GlideError> {
        if self.order_book.buy_orders.len() + self.order_book.sell_orders.len()
            >= self.config.max_swap_orders
        {
            return Err(GlideError::SwapTransactionFailure(
                "Max swap orders reached".to_string(),
            ));
        }
        if order.token_in < order.token_out {
            self.order_book.add_buy_order(order);
        } else {
            self.order_book.add_sell_order(order);
        }
        Ok(())
    }

    pub fn run_price_discovery(&self) -> f64 {
        swap_protocol::advanced_price_discovery(&self.order_book)
    }
}

/******************************************************************************
 * VALIDATOR LOGIC (From the provided technical context)
 * Now we rewrite it in Rust with advanced complexity, error handling, concurrency, etc.
 *****************************************************************************/
#[derive(Debug)]
pub struct Rule {
    pub description: String,
    pub status: String,
}

impl Rule {
    pub fn new(description: &str) -> Self {
        Rule {
            description: description.to_string(),
            status: "pending".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Validator {
    pub stake: i64,
    pub reputation: i64,
    pub validation_queue: Vec<Rule>,
    pub id: String,
}

impl Validator {
    pub fn new(id: &str) -> Self {
        Validator {
            stake: 0,
            reputation: 100,
            validation_queue: Vec::new(),
            id: id.to_string(),
        }
    }

    pub fn manage_stake(&mut self, amount: i64) -> Result<(), GlideError> {
        if amount < 0 {
            let proposed = self.stake + amount;
            if proposed < 0 {
                return Err(GlideError::InvalidStake(amount));
            }
            self.stake = proposed;
        } else {
            self.stake += amount;
        }
        Ok(())
    }

    pub fn process_validation_rules(&mut self) {
        for rule in &mut self.validation_queue {
            // Simulate some complexity
            let check = rule.description.len() as i64 + self.stake - self.reputation;
            if check > 0 {
                rule.status = "validated".to_string();
            } else {
                rule.status = "failed".to_string();
            }
        }
    }

    pub fn manage_reputation(&mut self) {
        if self.stake > 1000 {
            self.reputation += 10;
        } else {
            self.reputation = (self.reputation - 5).max(0);
        }
    }

    pub fn coordinate_with_validators(&self, others: &[Validator]) -> usize {
        // Just count how many have stake over some threshold
        let threshold = 1000;
        others.iter().filter(|v| v.stake as i64 > threshold).count()
    }

    pub fn add_rule(&mut self, description: &str) {
        self.validation_queue.push(Rule::new(description));
    }
}

/******************************************************************************
 * ADVANCED OBSERVER PATTERN FOR VALIDATOR EVENTS
 *****************************************************************************/
pub trait ValidatorObserver {
    fn on_stake_change(&self, validator: &Validator);
    fn on_reputation_change(&self, validator: &Validator);
}

pub struct ValidatorNotifier {
    observers: Vec<Box<dyn ValidatorObserver + Send + Sync>>,
}

impl ValidatorNotifier {
    pub fn new() -> Self {
        ValidatorNotifier { observers: vec![] }
    }

    pub fn register_observer(&mut self, observer: Box<dyn ValidatorObserver + Send + Sync>) {
        self.observers.push(observer);
    }

    pub fn notify_stake_change(&self, validator: &Validator) {
        for obs in &self.observers {
            obs.on_stake_change(validator);
        }
    }

    pub fn notify_reputation_change(&self, validator: &Validator) {
        for obs in &self.observers {
            obs.on_reputation_change(validator);
        }
    }
}

/******************************************************************************
 * EXAMPLE OBSERVER IMPLEMENTATIONS
 *****************************************************************************/
pub struct LogObserver;
impl ValidatorObserver for LogObserver {
    fn on_stake_change(&self, validator: &Validator) {
        println!(
            "[LogObserver] Validator {} stake changed to {}",
            validator.id, validator.stake
        );
    }
    fn on_reputation_change(&self, validator: &Validator) {
        println!(
            "[LogObserver] Validator {} reputation changed to {}",
            validator.id, validator.reputation
        );
    }
}

pub struct AIObserver;
impl ValidatorObserver for AIObserver {
    fn on_stake_change(&self, validator: &Validator) {
        if validator.stake > 500 {
            println!("[AIObserver] Triggering advanced AI routine for high stake validator {}", validator.id);
        }
    }
    fn on_reputation_change(&self, validator: &Validator) {
        if validator.reputation < 50 {
            println!("[AIObserver] AI-based alert: low reputation for validator {}", validator.id);
        }
    }
}

/******************************************************************************
 * COMMAND PATTERN FOR VOICE COMMANDS
 *****************************************************************************/
pub trait VoiceCommand {
    fn execute(&self, validator: &mut Validator) -> Result<(), GlideError>;
}

pub struct StakeCommand {
    pub amount: i64,
}

impl VoiceCommand for StakeCommand {
    fn execute(&self, validator: &mut Validator) -> Result<(), GlideError> {
        validator.manage_stake(self.amount)?;
        Ok(())
    }
}

pub struct ReputationCommand {
    pub increment: bool,
}

impl VoiceCommand for ReputationCommand {
    fn execute(&self, validator: &mut Validator) -> Result<(), GlideError> {
        if self.increment {
            validator.reputation += 1;
        } else {
            validator.reputation = validator.reputation.saturating_sub(1);
        }
        Ok(())
    }
}

/******************************************************************************
 * VISITOR PATTERN FOR VALIDATOR ANALYSIS
 *****************************************************************************/
pub trait ValidatorVisitor {
    fn visit_simple_validator(&mut self, validator: &Validator);
    fn visit_advanced_validator(&mut self, validator: &Validator);
}

pub struct AnalysisVisitor {
    pub high_stake_count: usize,
    pub low_reputation_count: usize,
}

impl AnalysisVisitor {
    pub fn new() -> Self {
        AnalysisVisitor {
            high_stake_count: 0,
            low_reputation_count: 0,
        }
    }
}

impl ValidatorVisitor for AnalysisVisitor {
    fn visit_simple_validator(&mut self, validator: &Validator) {
        if validator.stake > 1000 {
            self.high_stake_count += 1;
        }
        if validator.reputation < 50 {
            self.low_reputation_count += 1;
        }
    }

    fn visit_advanced_validator(&mut self, validator: &Validator) {
        if validator.stake > 2000 {
            self.high_stake_count += 1;
        }
        if validator.reputation < 30 {
            self.low_reputation_count += 1;
        }
    }
}

/******************************************************************************
 * MOCK ADVANCED VALIDATOR FOR DEMO WITH VISITOR PATTERN
 *****************************************************************************/
#[derive(Debug)]
pub struct AdvancedValidatorMock {
    pub stake: i64,
    pub reputation: i64,
    pub id: String,
}

impl AdvancedValidatorMock {
    pub fn new(id: &str, stake: i64, rep: i64) -> Self {
        AdvancedValidatorMock {
            id: id.to_string(),
            stake,
            reputation: rep,
        }
    }

    pub fn accept_visitor<T: ValidatorVisitor>(&self, visitor: &mut T) {
        visitor.visit_advanced_validator(&Validator {
            stake: self.stake,
            reputation: self.reputation,
            validation_queue: vec![],
            id: self.id.clone(),
        });
    }
}

/******************************************************************************
 * MAIN DEMO - Putting it all together
 *****************************************************************************/
pub fn demo_glide_platform() -> Result<(), GlideError> {
    let config = GlideConfigBuilder::new()
        .with_ai_subnets(true)
        .with_voice_strategy("advanced")
        .with_max_swap_orders(200)
        .with_advanced_mode(true)
        .build();

    let mut facade = GlideFacade::new(config);
    facade.initialize_system()?;

    let v1 = Validator::new("validator_1");
    let v2 = Validator::new("validator_2");
    let mut v3 = Validator::new("validator_3");

    let mut notifier = ValidatorNotifier::new();
    notifier.register_observer(Box::new(LogObserver));
    notifier.register_observer(Box::new(AIObserver));

    let mut v3_rules = vec!["Check block signature", "Check merkle root", "Check transaction count"];
    for rule_desc in v3_rules.drain(..) {
        v3.add_rule(rule_desc);
    }

    v3.manage_stake(1200)?;
    notifier.notify_stake_change(&v3);

    v3.process_validation_rules();
    v3.manage_reputation();
    notifier.notify_reputation_change(&v3);

    let adv_vmock = AdvancedValidatorMock::new("adv_v_1", 2300, 45);
    let mut analysis = AnalysisVisitor::new();
    adv_vmock.accept_visitor(&mut analysis);

    let stake_cmd = StakeCommand { amount: 500 };
    stake_cmd.execute(&mut v3)?;

    let rep_cmd = ReputationCommand { increment: false };
    rep_cmd.execute(&mut v3)?;

    let mut order_book = &mut facade.order_book;
    let swap_order = swap_protocol::SwapOrder {
        order_id: 1,
        token_in: "GLD".to_string(),
        token_out: "AIUSD".to_string(),
        amount_in: 100.0,
        amount_out_min: 90.0,
        user_id: "user_123".to_string(),
        created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
    };
    facade.add_swap_order(swap_order)?;

    order_book.match_orders();
    let pd = facade.run_price_discovery();
    println!("Discovered Price: {}", pd);

    Ok(())
}
