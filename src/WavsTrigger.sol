// SPDX-License-Identifier Apache-2.0
pragma solidity ^0.8.0;

import {ILayerTrigger} from "wavs/interfaces/ILayerTrigger.sol";

contract WavsTrigger {
    // Data structures
    struct Trigger {
        string serviceId;
        string workflowId;
        address creator;
        bytes data;
    }

    // Storage

    mapping(ILayerTrigger.TriggerId => Trigger) public triggersById;

    mapping(address => ILayerTrigger.TriggerId[]) public triggerIdsByCreator;

    // Events
    event NewTrigger(string serviceId, string workflowId, ILayerTrigger.TriggerId indexed triggerId);

    // Global vars
    ILayerTrigger.TriggerId public nextTriggerId;

    // Functions

    /**
     * @notice Add a new trigger.
     * @param serviceId The service identifier (string).
     * @param data The request data (bytes).
     */
    function addTrigger(string memory serviceId, string memory workflowId, bytes memory data) public {
        // Get the next trigger id
        nextTriggerId = ILayerTrigger.TriggerId.wrap(ILayerTrigger.TriggerId.unwrap(nextTriggerId) + 1);
        ILayerTrigger.TriggerId triggerId = nextTriggerId;

        // Create the trigger
        Trigger memory trigger = Trigger({
            serviceId: serviceId,
            workflowId: workflowId,
            creator: msg.sender,
            data: data
        });

        // update storages
        triggersById[triggerId] = trigger;

        triggerIdsByCreator[msg.sender].push(triggerId);

        // emit event
        emit NewTrigger(serviceId, workflowId, triggerId);
    }

    /**
     * @notice Get a single trigger by triggerId.
     * @param triggerId The identifier of the trigger.
     */
    function getTrigger(ILayerTrigger.TriggerId triggerId) public view returns (ILayerTrigger.TriggerResponse memory) {
        Trigger storage trigger = triggersById[triggerId];

        return ILayerTrigger.TriggerResponse({
            triggerId: triggerId,
            workflowId: trigger.workflowId,
            serviceId: trigger.serviceId,
            creator: trigger.creator,
            data: trigger.data
        });
    }

}
