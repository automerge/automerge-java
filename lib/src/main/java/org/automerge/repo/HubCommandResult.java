package org.automerge.repo;

class HubCommandResult {
    private final CommandId commandId;
    private final HubResults results;

    HubCommandResult(CommandId commandId, HubResults results) {
        this.commandId = commandId;
        this.results = results;
    }

    CommandId getCommandId() {
        return commandId;
    }

    HubResults getResults() {
        return results;
    }
}
