"""
AI Dual Subnets, Voice Recognition, and Advanced Swap Protocols
-----------------------------------------------------------------------------------------
ResourceAllocator Module

The ResourceAllocator module is designed to predict, allocate, and optimize resources across
the GLIDE blockchain platform. It leverages machine learning, complex mathematical
algorithms, and advanced design patterns to dynamically manage CPU, memory, and
network bandwidth in real-time.

Key Features: 
1. AI-driven resource prediction using sophisticated forecasting algorithms.
2. Dynamic CPU/memory management with concurrency and adaptative load balancing.
3. Advanced network bandwidth optimization utilizing multi-threaded scheduling.
4. Real-time resource usage analytics for improved cost efficiency and performance.
5. Adaptive scaling algorithms responding to blockchain node metrics and transaction throughput.
6. Extensive configuration capabilities allowing custom thresholds, time windows,
   and scaling factors.
7. Robust error handling and logging for real-world production environments.
8. Designed to be integrated into GLIDE’s AI Dual Subnets architecture, enabling
   automated resource orchestration across heterogeneous nodes.

Usage Example:
-----------------------------------------------------------------------------------------
from resource_allocator import ResourceAllocator, ResourceConfig, ResourceAllocatorFactory

config = ResourceConfig(
    ai_forecast_window=20,
    max_cpu_percent=90.0,
    min_cpu_percent=30.0,
    max_memory_percent=85.0, 
    min_memory_percent=25.0,
    network_opt_level="advanced",
    fallback_strategy="weighted_retry"
)

allocator = ResourceAllocatorFactory.get_allocator("strategic", config)

try:
    allocator.predict_resources()
    allocator.allocate_resources()
    allocator.manage_cpu_and_memory()
    allocator.optimize_network_bandwidth()
    allocator.analyze_resource_usage()
    allocator.scale_adaptively()
except Exception as e:
    print(f"Unhandled error: {str(e)}")
-----------------------------------------------------------------------------------------
"""

import logging
import threading
import math
import numpy as np
import time
from abc import ABC, abstractmethod
from typing import Dict, Any, Optional, List, Callable
from concurrent.futures import ThreadPoolExecutor, as_completed
from queue import Queue
from dataclasses import dataclass, field
from sklearn.linear_model import LinearRegression
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import MinMaxScaler
from collections import deque
import random

# -----------------------------------------------------------------------------------------
# Set up logging for the resource allocator
# -----------------------------------------------------------------------------------------
logging.basicConfig(
    level=logging.DEBUG,
    format="%(asctime)s [%(levelname)s] %(name)s: %(message)s"
)
logger = logging.getLogger("ResourceAllocator")

# -----------------------------------------------------------------------------------------
# Custom Exceptions
# -----------------------------------------------------------------------------------------
class ResourceAllocationError(Exception):
    """Raised when resource allocation fails unexpectedly."""
    pass

class PredictionModelError(Exception):
    """Raised when the AI-driven resource prediction model encounters an error."""
    pass

class ScalingError(Exception):
    """Raised when adaptive scaling fails or produces invalid results."""
    pass

class NetworkOptimizationError(Exception):
    """Raised when network optimization steps fail or exceed configured limits."""
    pass

class ConfigValidationError(Exception):
    """Raised when the provided configuration is invalid."""
    pass

# -----------------------------------------------------------------------------------------
# Configuration DataClass
# -----------------------------------------------------------------------------------------
@dataclass
class ResourceConfig:
    ai_forecast_window: int
    max_cpu_percent: float
    min_cpu_percent: float
    max_memory_percent: float
    min_memory_percent: float
    network_opt_level: str
    fallback_strategy: str
    advanced_tuning_params: Dict[str, Any] = field(default_factory=dict)

    def validate(self):
        if self.ai_forecast_window <= 0:
            raise ConfigValidationError("ai_forecast_window must be > 0.")
        if not (0 < self.min_cpu_percent < self.max_cpu_percent <= 100):
            raise ConfigValidationError("CPU percent thresholds must be valid and within 0-100 range.")
        if not (0 < self.min_memory_percent < self.max_memory_percent <= 100):
            raise ConfigValidationError("Memory percent thresholds must be valid and within 0-100 range.")
        valid_opt_levels = {"basic", "standard", "advanced"}
        if self.network_opt_level not in valid_opt_levels:
            raise ConfigValidationError(f"network_opt_level must be one of {valid_opt_levels}.")
        valid_fallbacks = {"retry", "weighted_retry", "abort"}
        if self.fallback_strategy not in valid_fallbacks:
            raise ConfigValidationError(f"fallback_strategy must be one of {valid_fallbacks}.")

# -----------------------------------------------------------------------------------------
# Base Strategy Interface for Allocation (Strategy Pattern)
# -----------------------------------------------------------------------------------------
class AllocationStrategy(ABC):
    @abstractmethod
    def allocate(self, current_load: float, config: ResourceConfig) -> float:
        pass

# -----------------------------------------------------------------------------------------
# Concrete Strategies
# -----------------------------------------------------------------------------------------
class LinearAllocationStrategy(AllocationStrategy):
    def allocate(self, current_load: float, config: ResourceConfig) -> float:
        """
        Uses a simple linear approach to resource allocation.
        If current_load is high, allocate near max thresholds.
        Otherwise, allocate near min thresholds.
        """
        logger.debug(f"LinearAllocationStrategy -> current_load: {current_load}")
        factor = current_load / 100.0
        if factor > 1.0:
            factor = 1.0
        allocated_cpu = config.min_cpu_percent + (config.max_cpu_percent - config.min_cpu_percent) * factor
        logger.debug(f"LinearAllocationStrategy -> allocated_cpu: {allocated_cpu}")
        return allocated_cpu

class StrategicAllocationStrategy(AllocationStrategy):
    def allocate(self, current_load: float, config: ResourceConfig) -> float:
        """
        A more sophisticated approach:
        (1) Uses polynomial scaling to ramp up CPU quickly after a threshold.
        (2) Leaves room for overhead if random spikes appear.
        """
        logger.debug(f"StrategicAllocationStrategy -> current_load: {current_load}")
        polynomial_factor = math.pow(current_load / 100.0, 2)
        if polynomial_factor > 1.0:
            polynomial_factor = 1.0
        allocated_cpu = config.min_cpu_percent + (config.max_cpu_percent - config.min_cpu_percent) * polynomial_factor
        allocated_cpu = min(allocated_cpu, config.max_cpu_percent)
        allocated_cpu = max(allocated_cpu, config.min_cpu_percent)
        overhead_buffer = random.uniform(0.01, 0.05) * allocated_cpu
        allocated_cpu -= overhead_buffer
        logger.debug(f"StrategicAllocationStrategy -> allocated_cpu with overhead buffer: {allocated_cpu}")
        return allocated_cpu

# -----------------------------------------------------------------------------------------
# ResourceAllocator Abstract Base
# -----------------------------------------------------------------------------------------
class BaseResourceAllocator(ABC):
    def __init__(self, config: ResourceConfig):
        self.config = config
        self.resources: Dict[str, Any] = {}
        self._train_data: Optional[np.ndarray] = None
        self._model: Optional[LinearRegression] = None
        self._scaler: Optional[MinMaxScaler] = None
        self._executor = ThreadPoolExecutor(max_workers=4)
        self._cpu_usage_history: deque = deque(maxlen=config.ai_forecast_window)
        self._memory_usage_history: deque = deque(maxlen=config.ai_forecast_window)
        self._network_usage_history: deque = deque(maxlen=config.ai_forecast_window)
        self.allocation_strategy: AllocationStrategy = LinearAllocationStrategy()

    @abstractmethod
    def predict_resources(self):
        pass

    @abstractmethod
    def allocate_resources(self):
        pass

    @abstractmethod
    def manage_cpu_and_memory(self):
        pass

    @abstractmethod
    def optimize_network_bandwidth(self):
        pass

    @abstractmethod
    def analyze_resource_usage(self):
        pass

    @abstractmethod
    def scale_adaptively(self):
        pass

    def _generate_mock_usage_data(self):
        return {
            "cpu_percent": random.uniform(0.0, 100.0),
            "memory_percent": random.uniform(0.0, 100.0),
            "network_kbps": random.uniform(100.0, 10000.0)
        }

    def _simulate_resource_loading(self):
        data = []
        for i in range(self.config.ai_forecast_window):
            usage = self._generate_mock_usage_data()
            data.append([usage["cpu_percent"], usage["memory_percent"], usage["network_kbps"]])
        return np.array(data)

# -----------------------------------------------------------------------------------------
# Concrete ResourceAllocator Implementation
# -----------------------------------------------------------------------------------------
class ResourceAllocator(BaseResourceAllocator):
    def __init__(self, config: ResourceConfig):
        super().__init__(config)
        self.config.validate()
        self.allocation_strategy = StrategicAllocationStrategy() if config.fallback_strategy == "weighted_retry" \
            else LinearAllocationStrategy()

    def predict_resources(self):
        """
        Utilizes AI-driven forecasting (simple demonstration here using LinearRegression).
        In practice, this could be replaced with more complex ML or deep learning models.
        """
        try:
            raw_data = self._simulate_resource_loading()
            cpu_data = raw_data[:, 0]
            memory_data = raw_data[:, 1]
            network_data = raw_data[:, 2]

            x_values = np.array([list(range(len(cpu_data)))]).T
            # Train for CPU usage forecast
            self._model = LinearRegression()
            x_train, x_test, y_train, y_test = train_test_split(x_values, cpu_data, test_size=0.2, random_state=42)
            self._model.fit(x_train, y_train)
            predicted = self._model.predict(x_test)
            mse = np.mean((predicted - y_test) ** 2)
            logger.info(f"AI forecast model trained. MSE for CPU usage: {mse:.2f}")

            # Additional usage predictions or advanced ML steps can be added here
            # as needed, including memory_data, network_data, etc.

        except Exception as ex:
            logger.error("Error during resource prediction", exc_info=True)
            raise PredictionModelError(str(ex)) from ex

    def allocate_resources(self):
        """
        Dynamically allocates CPU based on predicted usage. Memory and other resources
        could be allocated in similar or different patterns. 
        """
        try:
            usage_snapshot = self._generate_mock_usage_data()
            load_avg = (usage_snapshot["cpu_percent"] + usage_snapshot["memory_percent"]) / 2.0

            logger.debug(f"allocate_resources -> usage_snapshot: {usage_snapshot}, load_avg: {load_avg}")

            allocated_cpu = self.allocation_strategy.allocate(load_avg, self.config)
            self.resources["cpu_allocated_percent"] = allocated_cpu

            # For demonstration, memory allocation is linearly tied to CPU usage
            allocated_memory = self.config.min_memory_percent + (allocated_cpu / self.config.max_cpu_percent) * \
                               (self.config.max_memory_percent - self.config.min_memory_percent)

            self.resources["memory_allocated_percent"] = allocated_memory

            logger.info(f"Resources allocated -> CPU: {allocated_cpu:.2f}%, Memory: {allocated_memory:.2f}%")

        except Exception as ex:
            logger.error("Resource allocation failed", exc_info=True)
            raise ResourceAllocationError(str(ex)) from ex

    def manage_cpu_and_memory(self):
        """
        Uses concurrency to manage large ephemeral tasks. Simulates splitting tasks across threads
        to handle dynamic CPU load. Each task is monitored for usage spikes. 
        """
        def cpu_memory_task(task_id: int, usage_val: float) -> str:
            try:
                # Simulate some complex computations
                result = math.log(usage_val + 1) * (task_id + 1) * random.uniform(1.0, 2.0)
                time.sleep(random.uniform(0.05, 0.15))
                logger.debug(f"Task #{task_id} -> Computation result: {result}")
                return f"Task #{task_id} completed"
            except Exception as ex:
                raise ResourceAllocationError(f"Task #{task_id} failed: {str(ex)}")

        tasks = []
        for i in range(self.config.ai_forecast_window):
            usage_val = random.uniform(0.0, 100.0)
            tasks.append(self._executor.submit(cpu_memory_task, i, usage_val))

        for future in as_completed(tasks):
            try:
                result = future.result()
                logger.debug(result)
            except ResourceAllocationError as ex:
                logger.error(f"manage_cpu_and_memory -> {str(ex)}")

    def optimize_network_bandwidth(self):
        """
        Depending on the configured optimization level, choose between a single-threaded
        or multi-threaded approach to handle network I/O. This approach tries to reorder
        network tasks by priority (mimicked with random priority values).
        """
        try:
            num_network_tasks = random.randint(5, 15)
            task_queue = Queue()
            # Fill the queue with mock tasks and random priorities
            for _ in range(num_network_tasks):
                priority = random.randint(1, 10)
                data_size = random.uniform(10.0, 1000.0)  # in KB
                task_queue.put((priority, data_size))

            if self.config.network_opt_level == "advanced":
                # Sort tasks by priority
                sorted_tasks = sorted(list(task_queue.queue), key=lambda x: x[0])
                task_queue.queue.clear()
                for item in sorted_tasks:
                    task_queue.put(item)

            def network_task_executor(pri: int, data_kb: float) -> None:
                try:
                    overhead = 0.01 * pri
                    time.sleep(data_kb / 5000.0 + overhead)
                    logger.debug(f"Network task (priority={pri}, data_kb={data_kb:.2f}) completed")
                except Exception as ex:
                    raise NetworkOptimizationError(str(ex))

            with ThreadPoolExecutor(max_workers=4) as net_executor:
                futures = []
                while not task_queue.empty():
                    p, d = task_queue.get()
                    futures.append(net_executor.submit(network_task_executor, p, d))

                for fut in as_completed(futures):
                    fut.result()

            logger.info(f"Network bandwidth optimization completed for {num_network_tasks} tasks")

        except Exception as ex:
            logger.error("Network optimization failed", exc_info=True)
            raise NetworkOptimizationError(str(ex)) from ex

    def analyze_resource_usage(self):
        """
        Perform advanced analytics by analyzing the historical usage data
        (CPU, memory, network) to compute variance, standard deviation, and
        correlation. Also tries to detect anomalies using a rolling window.
        """
        # Gather some historical data to feed into advanced analytics
        cpu_vals = [random.uniform(0.0, 100.0) for _ in range(self.config.ai_forecast_window)]
        mem_vals = [random.uniform(0.0, 100.0) for _ in range(self.config.ai_forecast_window)]
        net_vals = [random.uniform(100.0, 10000.0) for _ in range(self.config.ai_forecast_window)]

        cpu_var = np.var(cpu_vals)
        mem_var = np.var(mem_vals)
        net_var = np.var(net_vals)

        logger.debug(f"analyze_resource_usage -> CPU Var: {cpu_var:.2f}, Mem Var: {mem_var:.2f}, Net Var: {net_var:.2f}")

        # Calculate correlation between CPU and Memory usage
        corr = np.corrcoef(cpu_vals, mem_vals)[0, 1]
        logger.info(f"Resource usage correlation between CPU and memory: {corr:.3f}")

        # Anomaly detection - simple approach: if usage is 2 std dev above mean
        mean_cpu = np.mean(cpu_vals)
        std_cpu = np.std(cpu_vals)
        anomalies = [x for x in cpu_vals if (x > mean_cpu + 2*std_cpu)]
        if anomalies:
            logger.warning(f"Detected potential CPU usage anomalies: {anomalies}")

    def scale_adaptively(self):
        """
        Implements advanced scaling logic based on real-time usage data and AI-driven forecasts.
        Incorporates a complex formula for scaling decisions:
        S(t) = S(t-1) + α*(UsageForecast - UsageCurrent) + β*(UsageStdDev)
        """
        try:
            usage_data_current = self._generate_mock_usage_data()
            # For demonstration, random forecast error is injected
            forecast_error = random.uniform(-5.0, 5.0)

            # Weighted approach with adjustable alpha, beta from advanced_tuning_params
            alpha = float(self.config.advanced_tuning_params.get("alpha", 0.1))
            beta = float(self.config.advanced_tuning_params.get("beta", 0.05))

            usage_forecast = (
                (usage_data_current["cpu_percent"] + usage_data_current["memory_percent"]) / 2.0
                + forecast_error
            )
            usage_std_dev = np.std([usage_data_current["cpu_percent"], usage_data_current["memory_percent"]])

            if "scale_factor" not in self.resources:
                self.resources["scale_factor"] = 1.0

            scale_factor_prev = self.resources["scale_factor"]
            scale_factor_new = scale_factor_prev + alpha * (usage_forecast - scale_factor_prev) + beta * usage_std_dev

            # Ensure the scale factor remains positive and under some upper limit
            scale_factor_new = max(0.1, min(scale_factor_new, 10.0))
            self.resources["scale_factor"] = scale_factor_new

            # Example use of this scale factor to adjust CPU/Memory
            cpu_alloc = self.resources.get("cpu_allocated_percent", 50.0)
            memory_alloc = self.resources.get("memory_allocated_percent", 50.0)

            cpu_alloc *= scale_factor_new
            memory_alloc *= scale_factor_new

            self.resources["cpu_allocated_percent"] = min(cpu_alloc, self.config.max_cpu_percent)
            self.resources["memory_allocated_percent"] = min(memory_alloc, self.config.max_memory_percent)

            logger.info(
                f"Adaptively scaled resources -> scale_factor: {scale_factor_new:.2f}, "
                f"CPU: {self.resources['cpu_allocated_percent']:.2f}%, "
                f"Memory: {self.resources['memory_allocated_percent']:.2f}%"
            )

        except Exception as ex:
            logger.error("Adaptive scaling failed", exc_info=True)
            raise ScalingError(str(ex)) from ex

# -----------------------------------------------------------------------------------------
# Factory for creating ResourceAllocator with different strategies
# -----------------------------------------------------------------------------------------
class ResourceAllocatorFactory:
    @staticmethod
    def get_allocator(strategy_type: str, config: ResourceConfig) -> ResourceAllocator:
        if strategy_type == "linear":
            alloc = ResourceAllocator(config)
            alloc.allocation_strategy = LinearAllocationStrategy()
            return alloc
        elif strategy_type == "strategic":
            return ResourceAllocator(config)
        else:
            raise ValueError(f"Unknown strategy type: {strategy_type}")

# -----------------------------------------------------------------------------------------
# Singleton Example (Optional Enhancement)
# -----------------------------------------------------------------------------------------
class _SingletonResourceAllocator:
    _instance: Optional[ResourceAllocator] = None

    @classmethod
    def instance(cls, config: ResourceConfig) -> ResourceAllocator:
        if cls._instance is None:
            cls._instance = ResourceAllocator(config)
        return cls._instance

# -----------------------------------------------------------------------------------------
# Utility Functions for Testing or Additional Logic
# -----------------------------------------------------------------------------------------
def run_forecast_simulation(allocator: ResourceAllocator, iterations: int = 5):
    """
    Runs multiple iterations of forecast -> allocate -> scale to demonstrate
    how the system behaves over time.
    """
    for i in range(iterations):
        logger.debug(f"run_forecast_simulation -> Iteration {i+1}/{iterations}")
        allocator.predict_resources()
        allocator.allocate_resources()
        allocator.manage_cpu_and_memory()
        allocator.optimize_network_bandwidth()
        allocator.analyze_resource_usage()
        allocator.scale_adaptively()

def compute_gradient_descent(initial_lr: float, steps: int, data: List[float]) -> float:
    """
    A demonstration of a more advanced algorithmic concept that might be
    used in resource optimization. This simulates a custom gradient descent:
        theta := theta - alpha * dJ/dtheta
    where J is a mock cost function. 
    """
    theta = random.uniform(0.0, 10.0)
    lr = initial_lr
    for step in range(steps):
        grad = 0.0
        for d in data:
            # Example cost function: (theta - d)^2
            grad += 2.0 * (theta - d)
        grad /= len(data)
        theta = theta - lr * grad
        lr = lr * 0.99  # decaying learning rate
        logger.debug(f"compute_gradient_descent -> Step: {step}, Theta: {theta:.3f}, LR: {lr:.5f}")
    return theta

def advanced_configuration_example() -> ResourceConfig:
    """
    Returns an example configuration that might be loaded from a file or environment
    in a real system. This includes advanced tuning parameters for the AI model.
    """
    config = ResourceConfig(
        ai_forecast_window=15,
        max_cpu_percent=95.0,
        min_cpu_percent=10.0,
        max_memory_percent=88.0,
        min_memory_percent=20.0,
        network_opt_level="advanced",
        fallback_strategy="weighted_retry",
        advanced_tuning_params={
            "alpha": 0.12,
            "beta": 0.07,
            "model_type": "LinearRegression",
            "enable_gradient_descent_debug": True
        }
    )
    config.validate()
    return config

def run_advanced_usage_demo():
    config = advanced_configuration_example()
    allocator = ResourceAllocatorFactory.get_allocator("strategic", config)

    # Optionally run a gradient descent as part of advanced usage
    if config.advanced_tuning_params.get("enable_gradient_descent_debug", False):
        mock_data = [random.uniform(0.0, 10.0) for _ in range(20)]
        final_theta = compute_gradient_descent(0.1, 10, mock_data)
        logger.info(f"Advanced usage demo -> Final theta from gradient descent: {final_theta:.3f}")

    run_forecast_simulation(allocator, iterations=3)

# -----------------------------------------------------------------------------------------
# Additional Complex Example: Weighted Resource Distribution
# -----------------------------------------------------------------------------------------
def weighted_resource_distribution(resources: Dict[str, float], weights: Dict[str, float]) -> Dict[str, float]:
    """
    Distributes resources (CPU, memory, etc.) across multiple sub-services based on
    weight factors. This might be used in AI-driven multi-tenant environments.
    """
    total_weight = sum(weights.values())
    result = {}
    for svc, w in weights.items():
        portion = w / total_weight
        distributed_cpu = resources["cpu_allocated_percent"] * portion
        distributed_mem = resources["memory_allocated_percent"] * portion
        result[svc] = {
            "cpu_percent": distributed_cpu,
            "memory_percent": distributed_mem
        }
        logger.debug(f"weighted_resource_distribution -> Service: {svc}, CPU: {distributed_cpu:.2f}, MEM: {distributed_mem:.2f}")
    return result

# -----------------------------------------------------------------------------------------
# Entry Point Check (if needed for direct module execution)
# -----------------------------------------------------------------------------------------
if __name__ == "__main__":
    # Example usage
    config_example = ResourceConfig(
        ai_forecast_window=10,
        max_cpu_percent=90.0,
        min_cpu_percent=30.0,
        max_memory_percent=85.0,
        min_memory_percent=25.0,
        network_opt_level="advanced",
        fallback_strategy="weighted_retry"
    )
    try:
        config_example.validate()
        allocator_example = ResourceAllocatorFactory.get_allocator("strategic", config_example)
        allocator_example.predict_resources()
        allocator_example.allocate_resources()
        allocator_example.manage_cpu_and_memory()
        allocator_example.optimize_network_bandwidth()
        allocator_example.analyze_resource_usage()
        allocator_example.scale_adaptively()

        distribution_result = weighted_resource_distribution(
            allocator_example.resources,
            {
                "voice_recognition_service": 1.5,
                "ai_subnet_node": 2.0,
                "swap_protocol_engine": 3.0
            }
        )
        logger.info(f"Weighted distribution result: {distribution_result}")

        run_advanced_usage_demo()

    except Exception as e:
        logger.error(f"Error in main execution: {str(e)}")
