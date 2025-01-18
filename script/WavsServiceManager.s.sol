// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {WavsServiceManager} from "../src/WavsServiceManager.sol";
import {ECDSAStakeRegistry} from "@eigenlayer/middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {IDelegationManager} from "@eigenlayer/middleware/lib/eigenlayer-contracts/src/contracts/interfaces/IDelegationManager.sol";
import {Quorum, StrategyParams} from "@eigenlayer/middleware/src/interfaces/IECDSAStakeRegistryEventsAndErrors.sol";
import {IStrategy} from "@eigenlayer/middleware/lib/eigenlayer-contracts/src/contracts/interfaces/IStrategy.sol";

// forge script ./script/WavsServiceManager.s.sol --rpc-url http://localhost:8545 --broadcast
contract WavsServiceManagerScript is Script {
    address public delegation_manager = vm.envAddress("CLI_EIGEN_CORE_DELEGATION_MANAGER");
    address public rewards_coordinator = vm.envAddress("CLI_EIGEN_CORE_REWARDS_COORDINATOR");
    address public avs_directory = vm.envAddress("CLI_EIGEN_CORE_AVS_DIRECTORY");

    uint privateKey = vm.envUint("FOUNDRY_ANVIL_PRIVATE_KEY");

    function setUp() public {}

    function run() public {
        vm.startBroadcast(privateKey);

        ECDSAStakeRegistry ecdsa_registry = new ECDSAStakeRegistry(IDelegationManager(delegation_manager));

        console.log("delegation_manager:", delegation_manager);
        console.log("rewards_coordinator:", rewards_coordinator);
        console.log("avs_directory:", avs_directory);

        WavsServiceManager sm = new WavsServiceManager(
            avs_directory,
            address(ecdsa_registry),
            rewards_coordinator,
            delegation_manager
        );


        IStrategy mockStrategy = IStrategy(address(0x1234));
        Quorum memory quorum = Quorum({strategies: new StrategyParams[](1)});
        quorum.strategies[0] = StrategyParams({
            strategy: mockStrategy,
            multiplier: 10_000
        });
        ecdsa_registry.initialize(address(sm), 0, quorum);

        vm.stopBroadcast();

        console.log("ServiceManager:", address(sm));
        console.log("ecdssa_registry (deployed):", address(ecdsa_registry));
    }

}
